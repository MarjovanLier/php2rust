//! Eloquent models: the columns of each table that the migrations declare, a struct per
//! model, and the column types told to Mago (P2R-400 to P2R-404).

use std::borrow::Cow;
use std::collections::{HashMap, HashSet};

use mago_allocator::LocalArena;
use mago_codex::metadata::CodebaseMetadata;
use mago_codex::metadata::flags::MetadataFlags;
use mago_codex::metadata::property::PropertyMetadata;
use mago_codex::metadata::ttype::TypeMetadata;
use mago_codex::misc::VariableIdentifier;
use mago_codex::ttype;
use mago_codex::visibility::Visibility;
use mago_database::file::{File, FileId};
use mago_names::ResolvedNames;
use mago_names::resolver::NameResolver;
use mago_span::{HasSpan, Span};
use mago_syntax::cst::*;
use mago_syntax::parser::parse_file;
use mago_syntax::walker::Walker;

use super::{Converter, Mode, ParameterSignature, classes};
use crate::names;
use crate::types::{self, RustType};

impl Converter<'_> {
    /// The properties of the model `key`: its fields, or none when it has no struct; `None`
    /// for a class that is no model (P2R-401).
    pub(super) fn model_fields(&self, key: &[u8]) -> Option<Vec<ParameterSignature>> {
        let model = self.models.iter().find(|model| model.key == key)?;
        Some(match &model.fields {
            Ok(fields) => fields.iter().map(field_signature).collect(),
            Err(_) => Vec::new(),
        })
    }

    /// Refuses what keeps a model from converting: the reason it has no struct, and each
    /// relation, `getRouteKeyName` or scope, which leave it the struct (P2R-401).
    pub(super) fn model_shape(&mut self, declaration: &Class<'_>) {
        let key = self.name_key(&declaration.name, declaration.name.value);
        let reason = self
            .models
            .iter()
            .find(|model| model.key == key)
            .and_then(|model| model.fields.as_ref().err().cloned());
        if let Some((span, message)) = reason {
            self.refuse(&span, message);
        }
        // PHP rejects these when it compiles the file, and Mago does not report them.
        if let Some(modifier) = classes::doubled_modifier(&declaration.modifiers) {
            self.refuse(modifier, classes::DOUBLED_MODIFIER);
        }
        let mut methods: HashSet<Vec<u8>> = HashSet::new();
        let mut properties: HashSet<&[u8]> = HashSet::new();
        // A method that keeps the struct still stops; one that takes it away is the reason
        // above, whose span holds this one (P2R-03).
        for member in declaration.members.iter() {
            match member {
                ClassLikeMember::Method(method)
                    if !methods.insert(method.name.value.to_ascii_lowercase()) =>
                {
                    self.refuse(
                        &method.name,
                        "PHP rejects a second method of one name in a class (P2R-401)",
                    );
                }
                ClassLikeMember::Method(method)
                    if !method.name.value.eq_ignore_ascii_case(b"casts") =>
                {
                    self.refuse(
                        &method.name,
                        "a method of a model is not supported yet (P2R-401)",
                    );
                }
                ClassLikeMember::Property(Property::Plain(plain)) => {
                    for item in plain.items.iter() {
                        if !properties.insert(item.variable().name) {
                            self.refuse(
                                item,
                                "PHP rejects a second property of one name in a class (P2R-401)",
                            );
                        }
                    }
                }
                // A trait of the input that a model uses is used (P2R-280).
                ClassLikeMember::TraitUse(trait_use) => {
                    for name in trait_use.trait_names.iter() {
                        let key = self.identifier_key(name);
                        self.used_traits.insert(key);
                    }
                }
                _ => {}
            }
        }
    }

    /// `#[derive(Clone, Debug)] pub struct M { .. }` of a model with a struct, and nothing
    /// for one without, which has stopped (P2R-401).
    pub(super) fn model_item(&self, index: usize) -> Vec<String> {
        let class = &self.classes[index];
        let has_struct = self
            .models
            .iter()
            .any(|model| model.key == class.key && model.fields.is_ok());
        if !has_struct {
            return Vec::new();
        }
        let name = &class.rust_name;
        let mut lines = vec!["#[derive(Clone, Debug)]".to_owned()];
        if class.fields.is_empty() {
            lines.push(format!("pub struct {name} {{}}"));
        } else {
            lines.push(format!("pub struct {name} {{"));
            for field in &class.fields {
                lines.push(format!(
                    "    pub {}: {},",
                    field.rust_name,
                    field.ty.owned()
                ));
            }
            lines.push("}".to_owned());
        }
        lines
    }

    /// True when `object` has a model type, whose attributes this section only reads
    /// (P2R-404).
    pub(super) fn model_object(&self, object: &Expression<'_>) -> bool {
        let Some(union) = self.expression_type(object) else {
            return false;
        };
        matches!(types::rust_type(union, &self.type_names), Ok(RustType::Class(name))
            if self.class_named(&name).is_some_and(|index| self.classes[index].model))
    }
}

/// The property of a field, as the struct and a read see it (P2R-401, P2R-404).
fn field_signature(field: &Field) -> ParameterSignature {
    let ty = match field.kind {
        Kind::Int => RustType::I64,
        Kind::Bool => RustType::Bool,
        Kind::Text | Kind::Untyped => RustType::String,
    };
    let ty = if field.nullable {
        RustType::Option(Box::new(ty))
    } else {
        ty
    };
    ParameterSignature {
        php_name: field.column.clone().into_bytes(),
        rust_name: field.rust_name.clone(),
        snake: field.rust_name.trim_start_matches("r#").to_owned(),
        ty,
        mode: Mode::Shared,
        default: None,
    }
}

/// Eloquent's model class, lowercase.
const MODEL: &[u8] = b"illuminate\\database\\eloquent\\model";

/// The traits a model may use, lowercase (P2R-401).
const MODEL_TRAITS: [&[u8]; 2] = [
    b"illuminate\\database\\eloquent\\factories\\hasfactory",
    b"illuminate\\database\\eloquent\\softdeletes",
];

/// The properties a model may declare, which configure Eloquent (P2R-401).
const MODEL_PROPERTIES: [&[u8]; 5] = [b"$table", b"$fillable", b"$guarded", b"$hidden", b"$casts"];

/// A model of the input: its lowercase name, and the fields of its struct, or where the
/// problem points and why it has no struct (P2R-401).
pub(super) struct Model {
    pub(super) key: Vec<u8>,
    pub(super) fields: Result<Vec<Field>, (Span, String)>,
}

/// A field of a model's struct (P2R-401).
#[derive(Clone, Debug)]
pub(super) struct Field {
    pub(super) column: String,
    pub(super) rust_name: String,
    pub(super) kind: Kind,
    pub(super) nullable: bool,
}

/// Finds the models of the input, folds the migrations for their tables, and tells Mago the
/// type of each field (P2R-400 to P2R-403). It runs before Mago populates the codebase.
pub(super) fn find(
    units: &[(&[Statement<'_>], &ResolvedNames<'_>)],
    codebase: &mut CodebaseMetadata,
    migrations: Option<&[Migration]>,
    included: &HashSet<FileId>,
) -> Vec<Model> {
    // A Model the input declares makes no class a model.
    let parent_included = codebase
        .get_class_like(MODEL)
        .is_some_and(|parent| included.contains(&parent.span.file_id));
    if !parent_included {
        return Vec::new();
    }
    let mut candidates: Vec<(Vec<u8>, &Class<'_>, &ResolvedNames<'_>)> = Vec::new();
    for (statements, names) in units {
        for statement in statements.iter() {
            let Statement::Class(class) = statement else {
                continue;
            };
            let Some(extends) = &class.extends else {
                continue;
            };
            let [parent] = extends.types.as_slice() else {
                continue;
            };
            if resolved(names, parent, parent.value()) == MODEL {
                let key = resolved(names, &class.name, class.name.value);
                candidates.push((key, class, *names));
            }
        }
    }
    let keyed: HashMap<Vec<u8>, String> = candidates
        .iter()
        .filter(|(_, class, names)| int_key(class, names))
        .map(|(key, class, _)| {
            (
                key.clone(),
                String::from_utf8_lossy(class.name.value).into_owned(),
            )
        })
        .collect();
    let tables = migrations.map(|migrations| fold(migrations, &keyed));
    let public = public_properties(codebase);
    let overridden = model_methods(codebase);
    let models: Vec<Model> = candidates
        .iter()
        .map(|(key, class, names)| Model {
            key: key.clone(),
            fields: model_fields(class, names, tables.as_ref(), &public, &overridden),
        })
        .collect();
    for model in &models {
        if let Ok(fields) = &model.fields {
            tell(codebase, &model.key, fields);
        }
    }
    models
}

/// The lowercase fully qualified name of the name at `node`, written `raw`.
fn resolved(names: &ResolvedNames<'_>, node: &impl HasSpan, raw: &[u8]) -> Vec<u8> {
    let full = names.resolve(&node.span().start).unwrap_or(raw);
    full.strip_prefix(b"\\")
        .unwrap_or(full)
        .to_ascii_lowercase()
}

/// True when the model keeps Laravel's int key, which `foreignIdFor` reads (P2R-400).
fn int_key(class: &Class<'_>, names: &ResolvedNames<'_>) -> bool {
    class.members.iter().all(|member| match member {
        ClassLikeMember::Property(Property::Plain(plain)) => plain.items.iter().all(|item| {
            ![&b"$primaryKey"[..], b"$keyType", b"$incrementing"].contains(&item.variable().name)
        }),
        ClassLikeMember::Method(method) => ![&b"getkeyname"[..], b"getkeytype", b"getforeignkey"]
            .contains(&method.name.value.to_ascii_lowercase().as_slice()),
        ClassLikeMember::TraitUse(trait_use) => trait_use
            .trait_names
            .iter()
            .all(|name| MODEL_TRAITS.contains(&resolved(names, name, name.value()).as_slice())),
        _ => true,
    })
}

/// The names, without `$`, of the public instance properties of Eloquent's model and its
/// traits, which PHP reads before `__get` (P2R-401).
fn public_properties(codebase: &CodebaseMetadata) -> HashSet<String> {
    let mut found = HashSet::new();
    let mut pending: Vec<Vec<u8>> = vec![MODEL.to_vec()];
    let mut seen: HashSet<Vec<u8>> = HashSet::new();
    while let Some(name) = pending.pop() {
        if !seen.insert(name.clone()) {
            continue;
        }
        let Some(class) = codebase.get_class_like(&name) else {
            continue;
        };
        for property in class.properties.values() {
            if property.read_visibility == Visibility::Public && !property.flags.is_static() {
                let text = String::from_utf8_lossy(property.name.0.as_bytes());
                found.insert(text.trim_start_matches('$').to_owned());
            }
        }
        pending.extend(
            class
                .used_traits
                .iter()
                .map(|used| used.as_bytes().to_ascii_lowercase()),
        );
    }
    found
}

/// The fields of a model, or where and why it has no struct (P2R-401, P2R-402).
fn model_fields(
    class: &Class<'_>,
    names: &ResolvedNames<'_>,
    tables: Option<&Tables>,
    public: &HashSet<String>,
    overridden: &HashSet<Vec<u8>>,
) -> Result<Vec<Field>, (Span, String)> {
    if let Some(attributes) = class.attribute_lists.first() {
        return Err((
            attributes.span(),
            "an attribute on a model is not supported, and an observer it names may change what an attribute read gives, so the model takes no struct (P2R-401)".to_owned(),
        ));
    }
    if let Some(modifier) = class
        .modifiers
        .iter()
        .find(|m| !matches!(m, Modifier::Final(_)))
    {
        return Err((
            modifier.span(),
            "a model may be final, and no other modifier is supported (P2R-401)".to_owned(),
        ));
    }
    if let Some(implements) = &class.implements {
        return Err((
            implements.span(),
            "a model that implements an interface is not supported (P2R-401)".to_owned(),
        ));
    }
    let mut table: Option<String> = None;
    let mut casts: HashMap<String, Option<Kind>> = HashMap::new();
    let mut later_casts: HashMap<String, Option<Kind>> = HashMap::new();
    for member in class.members.iter() {
        match model_member(member, names, overridden) {
            Member::Table(name) => table = Some(name),
            Member::Casts(map) => casts.extend(map),
            Member::CastsMethod(map) => later_casts.extend(map),
            Member::LeftOut | Member::KeepsReads => {}
            Member::Other => {
                return Err((
                    member.span(),
                    "PHP may read an attribute of a model through this member (an accessor, an override of a method of Model, another property, constant or trait), so the model takes no struct (P2R-401)".to_owned(),
                ));
            }
        }
    }
    // Laravel merges casts() over $casts.
    casts.extend(later_casts);
    let timestamps = !class.members.iter().any(|member| {
        matches!(member, ClassLikeMember::Property(Property::Plain(plain))
            if matches!(plain.items.as_slice(), [PropertyItem::Concrete(item)]
                if item.variable.name == b"$timestamps"
                    && matches!(item.value, Expression::Literal(Literal::False(_)))))
    });
    let soft_deletes = class.members.iter().any(|member| {
        matches!(member, ClassLikeMember::TraitUse(trait_use)
        if trait_use.trait_names.iter().any(|name| {
            resolved(names, name, name.value()) == b"illuminate\\database\\eloquent\\softdeletes"
        }))
    });
    let Some(tables) = tables else {
        return Err((
            class.name.span(),
            "a model needs --migrations, which declare the columns of its table (P2R-401)"
                .to_owned(),
        ));
    };
    let table = table.unwrap_or_else(|| table_name(&String::from_utf8_lossy(class.name.value)));
    let columns = match tables.get(&table) {
        Some(Table::Known(columns)) => columns,
        Some(Table::Unknown(reason)) => {
            return Err((
                class.name.span(),
                format!(
                    "the migrations leave the table {table} of this model unknown: {reason} (P2R-400)"
                ),
            ));
        }
        None => {
            return Err((
                class.name.span(),
                format!("no migration creates the table {table} of this model (P2R-401)"),
            ));
        }
    };
    let mut cast_names: Vec<&String> = casts.keys().collect();
    cast_names.sort();
    if let Some(missing) = cast_names
        .iter()
        .find(|name| !columns.iter().any(|column| &&column.name == *name))
    {
        return Err((
            class.name.span(),
            format!(
                "a cast names the column {missing}, which the table {table} does not have (P2R-401)"
            ),
        ));
    }
    let mut fields: Vec<Field> = Vec::new();
    for column in columns {
        let kind = match casts.get(&column.name) {
            Some(Some(kind)) => *kind,
            Some(None) => continue,
            // Laravel casts the key `id` to int under the model's casts, as its key type is
            // int and it increments (a model that changes either takes no struct).
            None if column.name == "id" => Kind::Int,
            // And it reads these as dates, whatever the column method.
            None if (timestamps
                && (column.name == "created_at" || column.name == "updated_at"))
                || (soft_deletes && column.name == "deleted_at") =>
            {
                continue;
            }
            None if column.kind == Kind::Untyped => continue,
            None => column.kind,
        };
        // PHP reads a public property of the model instead of the column.
        if public.contains(&column.name) {
            continue;
        }
        let Some(rust_name) = names::rust_name(&column.name) else {
            continue;
        };
        if let Some(other) = fields.iter().find(|field| field.rust_name == rust_name) {
            return Err((
                class.name.span(),
                format!(
                    "the columns {} and {} give the same Rust name {rust_name} (P2R-131)",
                    other.column, column.name
                ),
            ));
        }
        fields.push(Field {
            column: column.name.clone(),
            rust_name,
            kind,
            nullable: column.nullable,
        });
    }
    Ok(fields)
}

/// What a member of a model is (P2R-401).
enum Member {
    /// `protected $table = '<t>';`
    Table(String),
    /// `protected $casts = [..];` and the method `casts()`, each column with the kind of
    /// its cast, or none for a cast that leaves the column out.
    Casts(HashMap<String, Option<Kind>>),
    CastsMethod(HashMap<String, Option<Kind>>),
    /// Another property or trait use that configures Eloquent.
    LeftOut,
    /// A relation, `getRouteKeyName` or a scope, which stop but change no read.
    KeepsReads,
    Other,
}

fn model_member(
    member: &ClassLikeMember<'_>,
    names: &ResolvedNames<'_>,
    overridden: &HashSet<Vec<u8>>,
) -> Member {
    match member {
        ClassLikeMember::TraitUse(trait_use) => {
            let known = trait_use
                .trait_names
                .iter()
                .all(|name| MODEL_TRAITS.contains(&resolved(names, name, name.value()).as_slice()));
            if known && matches!(trait_use.specification, TraitUseSpecification::Abstract(_)) {
                Member::LeftOut
            } else {
                Member::Other
            }
        }
        // `public $timestamps = false;` changes only the date columns, which have no field.
        ClassLikeMember::Property(Property::Plain(plain))
            if matches!(plain.modifiers.as_slice(), [Modifier::Public(_)])
                && plain.hint.is_none()
                && plain.attribute_lists.is_empty()
                && matches!(plain.items.as_slice(), [PropertyItem::Concrete(item)]
                    if item.variable.name == b"$timestamps"
                        && matches!(item.value, Expression::Literal(Literal::True(_) | Literal::False(_)))) =>
        {
            Member::LeftOut
        }
        ClassLikeMember::Property(Property::Plain(plain)) => {
            let protected = matches!(plain.modifiers.as_slice(), [Modifier::Protected(_)]);
            let [PropertyItem::Concrete(item)] = plain.items.as_slice() else {
                return Member::Other;
            };
            if !protected
                || plain.hint.is_some()
                || !plain.attribute_lists.is_empty()
                || !MODEL_PROPERTIES.contains(&item.variable.name)
            {
                return Member::Other;
            }
            match item.variable.name {
                b"$table" => match string_literal(item.value) {
                    Some(table) => Member::Table(table),
                    None => Member::Other,
                },
                b"$casts" => match cast_map(item.value) {
                    Some(map) => Member::Casts(map),
                    None => Member::Other,
                },
                _ if plain_literal(item.value) => Member::LeftOut,
                _ => Member::Other,
            }
        }
        ClassLikeMember::Method(method) => {
            let lower = method.name.value.to_ascii_lowercase();
            if lower == b"casts" {
                return match casts_method(method) {
                    Some(map) => Member::CastsMethod(map),
                    None => Member::Other,
                };
            }
            // Laravel reads an attribute through an accessor, a method returning
            // `Casts\Attribute`, or a method of the model that this one overrides; it runs a
            // method with `#[Boot]` or `#[Initialize]`, or named boot<Trait> or
            // initialize<Trait>, when it boots or builds the model.
            let accessor = lower.len() > b"getattribute".len()
                && (lower.starts_with(b"get") || lower.starts_with(b"set"))
                && lower.ends_with(b"attribute");
            let attribute = method
                .return_type_hint
                .as_ref()
                .is_some_and(|hint| names_attribute(&hint.hint, names));
            let lifecycle = lower.starts_with(b"boot") || lower.starts_with(b"initialize");
            let attributed = method
                .attribute_lists
                .iter()
                .flat_map(|list| list.attributes.iter())
                .any(|attribute| {
                    resolved(names, &attribute.name, attribute.name.value()) != b"override"
                });
            if lower == b"getroutekeyname" && !attributed {
                Member::KeepsReads
            } else if accessor
                || attribute
                || lifecycle
                || attributed
                || overridden.contains(&lower)
            {
                Member::Other
            } else {
                Member::KeepsReads
            }
        }
        _ => Member::Other,
    }
}

/// True for a type that names `Casts\Attribute`, also inside `?` or a union, which Laravel
/// reads as an accessor (P2R-401).
fn names_attribute(hint: &Hint<'_>, names: &ResolvedNames<'_>) -> bool {
    match hint {
        Hint::Identifier(identifier) => {
            resolved(names, identifier, identifier.value())
                == b"illuminate\\database\\eloquent\\casts\\attribute"
        }
        Hint::Nullable(nullable) => names_attribute(nullable.hint, names),
        Hint::Union(union) => {
            names_attribute(union.left, names) || names_attribute(union.right, names)
        }
        Hint::Parenthesized(inner) => names_attribute(inner.hint, names),
        _ => false,
    }
}

/// The lowercase names of the methods of Eloquent's model and its traits at any depth,
/// which a method of a model would override (P2R-401).
fn model_methods(codebase: &CodebaseMetadata) -> HashSet<Vec<u8>> {
    let mut found = HashSet::new();
    let mut pending: Vec<Vec<u8>> = vec![MODEL.to_vec()];
    let mut seen: HashSet<Vec<u8>> = HashSet::new();
    while let Some(name) = pending.pop() {
        if !seen.insert(name.clone()) {
            continue;
        }
        let Some(class) = codebase.get_class_like(&name) else {
            continue;
        };
        found.extend(
            class
                .methods
                .iter()
                .map(|method| method.as_bytes().to_ascii_lowercase()),
        );
        pending.extend(
            class
                .used_traits
                .iter()
                .map(|used| used.as_bytes().to_ascii_lowercase()),
        );
    }
    found
}

/// `protected function casts(): array { return [..]; }` with a literal map.
fn casts_method(method: &Method<'_>) -> Option<HashMap<String, Option<Kind>>> {
    let protected = matches!(method.modifiers.as_slice(), [Modifier::Protected(_)]);
    let returns_array =
        matches!(&method.return_type_hint, Some(hint) if matches!(hint.hint, Hint::Array(_)));
    if !protected || !returns_array || !method.parameter_list.parameters.is_empty() {
        return None;
    }
    let MethodBody::Concrete(block) = &method.body else {
        return None;
    };
    match block.statements.as_slice() {
        [
            Statement::Return(Return {
                value: Some(value), ..
            }),
        ] => cast_map(value),
        _ => None,
    }
}

/// The map of a `$casts` literal: each column with the kind its cast gives, or none for a
/// cast that leaves the column out (P2R-402).
fn cast_map(expression: &Expression<'_>) -> Option<HashMap<String, Option<Kind>>> {
    let Expression::Array(array) = expression else {
        return None;
    };
    let mut map = HashMap::new();
    for element in array.elements.iter() {
        let ArrayElement::KeyValue(entry) = element else {
            return None;
        };
        let column = string_literal(entry.key)?;
        let kind = match entry.value {
            Expression::Literal(Literal::String(_)) => {
                match string_literal(entry.value)?.as_str() {
                    "int" | "integer" => Some(Kind::Int),
                    "string" => Some(Kind::Text),
                    "bool" | "boolean" => Some(Kind::Bool),
                    _ => None,
                }
            }
            value if class_name_literal(value) => None,
            _ => return None,
        };
        map.insert(column, kind);
    }
    Some(map)
}

/// True for `C::class`.
fn class_name_literal(expression: &Expression<'_>) -> bool {
    matches!(expression, Expression::Access(Access::ClassConstant(access))
        if matches!(access.class, Expression::Identifier(_))
            && matches!(&access.constant, ClassLikeConstantSelector::Identifier(constant)
                if constant.value.eq_ignore_ascii_case(b"class")))
}

/// True for a string, or a list or map of strings and `C::class`.
fn plain_literal(expression: &Expression<'_>) -> bool {
    match expression {
        Expression::Literal(Literal::String(string)) => string.value.is_some(),
        Expression::Array(array) => array.elements.iter().all(|element| match element {
            ArrayElement::Value(value) => {
                string_literal(value.value).is_some() || class_name_literal(value.value)
            }
            ArrayElement::KeyValue(entry) => {
                string_literal(entry.key).is_some()
                    && (string_literal(entry.value).is_some() || class_name_literal(entry.value))
            }
            _ => false,
        }),
        _ => false,
    }
}

/// Tells Mago each field of the model as a property PHP reads through `__get`, as a
/// `@property` tag would, replacing a tag of the same name (P2R-403).
fn tell(codebase: &mut CodebaseMetadata, key: &[u8], fields: &[Field]) {
    let Some(class) = codebase
        .class_likes
        .values_mut()
        .find(|class| class.name.as_bytes().eq_ignore_ascii_case(key))
    else {
        return;
    };
    for field in fields {
        let name = mago_word::word(format!("${}", field.column));
        let ty = match field.kind {
            Kind::Int => ttype::get_int(),
            Kind::Text => ttype::get_string(),
            Kind::Bool => ttype::get_bool(),
            Kind::Untyped => continue,
        };
        let ty = if field.nullable { ty.as_nullable() } else { ty };
        let mut property = PropertyMetadata::new(VariableIdentifier(name), MetadataFlags::empty());
        property.type_metadata = Some(TypeMetadata::new(ty, class.span));
        class.magic_properties.insert(name, property);
        let owner = class.name;
        class.magic_property_ids.insert(name, owner);
    }
}

/// A file of `--migrations`: the path its reasons print, and its bytes, or none when it
/// could not be read or is not UTF-8 (P2R-400).
pub struct Migration {
    pub shown: String,
    pub source: Option<Vec<u8>>,
}

/// The Rust type a column reads as, or none for a date, boolean, decimal or float column,
/// whose PHP type a cast may still give; `Bool` comes from a cast only (P2R-400, P2R-402).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Kind {
    Int,
    Text,
    Bool,
    Untyped,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct Column {
    pub(super) name: String,
    pub(super) kind: Kind,
    pub(super) nullable: bool,
}

/// A table after the migrations: its columns in the order of their first declaration, or
/// the reason it is unknown, with the migration and line of that form.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) enum Table {
    Known(Vec<Column>),
    Unknown(String),
}

pub(super) type Tables = HashMap<String, Table>;

/// Folds the `up()` of each migration, in order, into the tables (P2R-400). `keyed` maps
/// the lowercase name of each model of the input with an int key to its short name, which
/// `foreignIdFor` names its column after.
pub(super) fn fold(migrations: &[Migration], keyed: &HashMap<Vec<u8>, String>) -> Tables {
    let mut tables = Tables::new();
    // True once a form the tool does not read may have changed any table, also one it has
    // not seen, which a later hasTable guard may then find.
    let mut unsure = false;
    for migration in migrations {
        let Some(source) = &migration.source else {
            unknown_all(
                &mut tables,
                format!("{}: the file cannot be read as UTF-8", migration.shown),
            );
            unsure = true;
            continue;
        };
        let file = File::ephemeral(
            Cow::Owned(migration.shown.clone().into_bytes()),
            Cow::Owned(source.clone()),
        );
        let arena = LocalArena::new();
        let program = parse_file(&arena, &file);
        if !program.errors.is_empty() {
            unknown_all(
                &mut tables,
                format!("{}: the file does not parse", migration.shown),
            );
            unsure = true;
            continue;
        }
        let names = NameResolver::new(&arena).resolve(program);
        let reader = Reader {
            file: &file,
            shown: &migration.shown,
            names: &names,
            keyed,
        };
        let Some((class, up)) = reader.up(program) else {
            unknown_all(
                &mut tables,
                format!(
                    "{}: the file is not a migration of the form the tool reads",
                    migration.shown
                ),
            );
            unsure = true;
            continue;
        };
        // A migration that only changes rows changes no table.
        if reader.rows_only(class) {
            continue;
        }
        for statement in up.statements.iter() {
            reader.statement(statement, &mut tables, &mut unsure);
        }
    }
    tables
}

/// Makes every table that exists unknown, keeping the first reason of one that already is.
fn unknown_all(tables: &mut Tables, reason: String) {
    for table in tables.values_mut() {
        if matches!(table, Table::Known(_)) {
            *table = Table::Unknown(reason.clone());
        }
    }
}

/// Laravel's snake case: an `_` before each capital letter that follows another character,
/// then lower case (`VATRate` gives `v_a_t_rate`).
pub(super) fn laravel_snake(name: &str) -> String {
    let mut out = String::new();
    for (at, character) in name.chars().enumerate() {
        if at > 0 && character.is_ascii_uppercase() {
            out.push('_');
        }
        out.push(character.to_ascii_lowercase());
    }
    out
}

/// The table of a model without `$table`: Laravel's snake case of its short name with the
/// last word in the regular plural (P2R-401).
pub(super) fn table_name(short: &str) -> String {
    let snake = laravel_snake(short);
    let consonant_y = snake.len() > 1
        && snake.ends_with('y')
        && !matches!(
            snake.as_bytes()[snake.len() - 2],
            b'a' | b'e' | b'i' | b'o' | b'u'
        );
    if consonant_y {
        format!("{}ies", &snake[..snake.len() - 1])
    } else if ["s", "x", "ch", "sh"]
        .iter()
        .any(|end| snake.ends_with(end))
    {
        format!("{snake}es")
    } else {
        format!("{snake}s")
    }
}

/// Reads the forms of one migration.
struct Reader<'r> {
    file: &'r File,
    shown: &'r str,
    names: &'r ResolvedNames<'r>,
    keyed: &'r HashMap<Vec<u8>, String>,
}

impl Reader<'_> {
    /// The reason that names this migration and the line of `node`.
    fn reason(&self, node: &impl HasSpan, what: &str) -> String {
        let line = self.file.line_number(node.span().start.offset) + 1;
        format!("{}:{line}: {what}", self.shown)
    }

    /// The lowercase fully qualified name of a class name.
    fn resolved(&self, identifier: &Identifier<'_>) -> Vec<u8> {
        let full = self
            .names
            .resolve(&identifier.span().start)
            .unwrap_or(identifier.value());
        full.strip_prefix(b"\\")
            .unwrap_or(full)
            .to_ascii_lowercase()
    }

    /// The class and the body of `up()` of `return new class extends Migration { .. };`, the
    /// file's last top-level statement. A class with `shouldRun()`, which Laravel asks
    /// before it runs `up()`, is not of this form.
    fn up<'p, 'a>(
        &self,
        program: &'p Program<'a>,
    ) -> Option<(&'p AnonymousClass<'a>, &'p Block<'a>)> {
        let last = program
            .statements
            .iter()
            .rfind(|statement| !matches!(statement, Statement::Noop(_)))?;
        // Laravel runs code before the return when it requires the file.
        let code = program.statements.iter().filter(|statement| {
            !matches!(
                statement,
                Statement::OpeningTag(_)
                    | Statement::ClosingTag(_)
                    | Statement::Declare(_)
                    | Statement::Use(_)
                    | Statement::Noop(_)
            )
        });
        if code.count() != 1 {
            return None;
        }
        let Statement::Return(Return {
            value: Some(Expression::AnonymousClass(class)),
            ..
        }) = last
        else {
            return None;
        };
        let extends = class.extends.as_ref()?;
        let [parent] = extends.types.as_slice() else {
            return None;
        };
        if self.resolved(parent) != b"illuminate\\database\\migrations\\migration" {
            return None;
        }
        let should_run = class.members.iter().any(|member| {
            matches!(member, ClassLikeMember::Method(method)
                if method.name.value.eq_ignore_ascii_case(b"shouldRun"))
        });
        // PHP rejects a second method, property or constant of one name and a modifier
        // written twice, so Laravel stops there; a property name starts with `$`, and a
        // constant's gets `::`.
        let mut seen: HashSet<Vec<u8>> = HashSet::new();
        let doubled = class.members.iter().any(|member| match member {
            ClassLikeMember::Method(method) => {
                !seen.insert(method.name.value.to_ascii_lowercase())
                    || classes::doubled_modifier(&method.modifiers).is_some()
            }
            ClassLikeMember::Property(Property::Plain(plain)) => {
                plain
                    .items
                    .iter()
                    .any(|item| !seen.insert(item.variable().name.to_vec()))
                    || classes::doubled_modifier(&plain.modifiers).is_some()
            }
            ClassLikeMember::Constant(constant) => {
                constant
                    .items
                    .iter()
                    .any(|item| !seen.insert([b"::", item.name.value].concat()))
                    || classes::doubled_modifier(&constant.modifiers).is_some()
            }
            _ => false,
        });
        // PHP 8.5 allows a closure in a constant expression, and a hook runs on a property
        // read: either may hold code that a property or constant read in up() runs.
        let mut hidden = false;
        for member in class.members.iter() {
            match member {
                ClassLikeMember::Property(Property::Hooked(_)) => hidden = true,
                ClassLikeMember::Property(property) => {
                    HasClosure.walk_property(property, &mut hidden)
                }
                ClassLikeMember::Constant(constant) => {
                    HasClosure.walk_class_like_constant(constant, &mut hidden);
                }
                ClassLikeMember::Method(method) => {
                    HasClosure
                        .walk_function_like_parameter_list(&method.parameter_list, &mut hidden);
                }
                _ => {}
            }
        }
        if should_run || doubled || hidden {
            return None;
        }
        let up = class.members.iter().find_map(|member| match member {
            ClassLikeMember::Method(method)
                if method.name.value.eq_ignore_ascii_case(b"up")
                    && method.parameter_list.parameters.is_empty()
                    && method
                        .modifiers
                        .iter()
                        .all(|m| matches!(m, Modifier::Public(_)))
                    && method
                        .return_type_hint
                        .as_ref()
                        .is_none_or(|hint| matches!(hint.hint, Hint::Void(_))) =>
            {
                match &method.body {
                    MethodBody::Concrete(block) => Some(block),
                    MethodBody::Abstract(_) => None,
                }
            }
            _ => None,
        })?;
        Some((class, up))
    }

    /// True when every call in the class, `down()` aside, is one that changes rows only: a
    /// chain on `DB::table(..)`, `DB::raw(..)`, `now()`, `date(..)`, `count(..)`,
    /// `Carbon::now()` or a call of a method the class declares (P2R-400).
    fn rows_only(&self, class: &AnonymousClass<'_>) -> bool {
        let own: Vec<Vec<u8>> = class
            .members
            .iter()
            .filter_map(|member| match member {
                ClassLikeMember::Method(method) => Some(method.name.value.to_ascii_lowercase()),
                _ => None,
            })
            .collect();
        let walker = RowsOnly { reader: self, own };
        let mut rows = true;
        for member in class.members.iter() {
            let down = matches!(member, ClassLikeMember::Method(method)
                if method.name.value.eq_ignore_ascii_case(b"down"));
            if !down {
                walker.walk_class_like_member(member, &mut rows);
            }
        }
        rows
    }

    /// True for the facade `facade` (`schema`, `db`), by its full name or by Laravel's
    /// global alias.
    fn facade(&self, expression: &Expression<'_>, facade: &str) -> bool {
        let Expression::Identifier(identifier) = expression else {
            return false;
        };
        let name = self.resolved(identifier);
        name == facade.as_bytes()
            || name == format!("illuminate\\support\\facades\\{facade}").as_bytes()
    }

    /// Applies one statement of `up()` to the tables.
    fn statement(&self, statement: &Statement<'_>, tables: &mut Tables, unsure: &mut bool) {
        if let Some((create, table, closure)) = self.schema_call(statement) {
            self.apply(statement, create, table, closure, tables);
            return;
        }
        // `if (! Schema::hasTable('t')) { Schema::create('t', ..); }` creates a missing `t`;
        // after a form the tool does not read, `t` may exist without its knowing.
        if let Some((table, inner)) = self.guarded_create(statement) {
            if tables.contains_key(&table) {
                return;
            }
            if *unsure {
                let reason = self.reason(
                    statement,
                    "a hasTable guard after a form the tool does not read",
                );
                tables.insert(table, Table::Unknown(reason));
            } else if let Some((true, _, closure)) = self.schema_call(inner) {
                self.apply(inner, true, table, closure, tables);
            }
            return;
        }
        // A chain on `DB::table(..)` changes rows, not columns, when its arguments call only
        // the forms of a rows-only migration; a callback of chunk() may call Schema.
        if let Statement::Expression(expression) = statement
            && self.on_db_table(expression.expression)
        {
            let walker = RowsOnly {
                reader: self,
                own: Vec::new(),
            };
            let mut rows = true;
            walker.walk_expression(expression.expression, &mut rows);
            if rows {
                return;
            }
        }
        unknown_all(
            tables,
            self.reason(
                statement,
                "a statement other than Schema::create, Schema::table and a DB::table chain",
            ),
        );
        *unsure = true;
    }

    /// `Schema::create('<t>', <closure>);` or `Schema::table('<t>', <closure>);`: whether it
    /// creates, the table, and the closure.
    fn schema_call<'s, 'a>(
        &self,
        statement: &'s Statement<'a>,
    ) -> Option<(bool, String, &'s Expression<'a>)> {
        let Statement::Expression(expression) = statement else {
            return None;
        };
        let Expression::Call(Call::StaticMethod(call)) = expression.expression else {
            return None;
        };
        if !self.facade(call.class, "schema") {
            return None;
        }
        let ClassLikeMemberSelector::Identifier(method) = &call.method else {
            return None;
        };
        let create = if method.value.eq_ignore_ascii_case(b"create") {
            true
        } else if method.value.eq_ignore_ascii_case(b"table") {
            false
        } else {
            return None;
        };
        let [table, closure] = positional(&call.argument_list)?;
        Some((create, string_literal(table)?, closure))
    }

    /// `if (! Schema::hasTable('<t>')) { <one statement> }` with no else: the table and the
    /// statement.
    fn guarded_create<'s, 'a>(
        &self,
        statement: &'s Statement<'a>,
    ) -> Option<(String, &'s Statement<'a>)> {
        let Statement::If(condition) = statement else {
            return None;
        };
        let IfBody::Statement(body) = &condition.body else {
            return None;
        };
        if !body.else_if_clauses.is_empty() || body.else_clause.is_some() {
            return None;
        }
        let Expression::UnaryPrefix(not) = condition.condition else {
            return None;
        };
        if !matches!(not.operator, UnaryPrefixOperator::Not(_)) {
            return None;
        }
        let Expression::Call(Call::StaticMethod(call)) = not.operand else {
            return None;
        };
        let has_table = matches!(&call.method, ClassLikeMemberSelector::Identifier(method)
            if method.value.eq_ignore_ascii_case(b"hasTable"));
        if !has_table || !self.facade(call.class, "schema") {
            return None;
        }
        let [table] = positional(&call.argument_list)?;
        let table = string_literal(table)?;
        let inner = match body.statement {
            Statement::Block(block) => match block.statements.as_slice() {
                [only] => only,
                _ => return None,
            },
            other => other,
        };
        match self.schema_call(inner) {
            Some((true, created, _)) if created == table => Some((table, inner)),
            _ => None,
        }
    }

    /// True for a chain of calls whose innermost receiver is `DB::table(..)` and that does
    /// not reach the connection, which runs any statement.
    fn on_db_table(&self, expression: &Expression<'_>) -> bool {
        match expression {
            Expression::Call(Call::Method(call)) => {
                !runs_any_statement(&call.method) && self.on_db_table(call.object)
            }
            Expression::Call(Call::StaticMethod(call)) => {
                self.facade(call.class, "db")
                    && matches!(&call.method, ClassLikeMemberSelector::Identifier(method)
                        if method.value.eq_ignore_ascii_case(b"table"))
            }
            _ => false,
        }
    }

    /// Creates or changes `table` by the body of `closure`.
    fn apply(
        &self,
        statement: &Statement<'_>,
        create: bool,
        table: String,
        closure: &Expression<'_>,
        tables: &mut Tables,
    ) {
        let mut columns = match (create, tables.get(&table)) {
            (true, None) => Vec::new(),
            (false, Some(Table::Known(columns))) => columns.clone(),
            (false, Some(Table::Unknown(_))) => return,
            (true, Some(_)) => {
                let reason = self.reason(statement, "Schema::create of a table that exists");
                tables.insert(table, Table::Unknown(reason));
                return;
            }
            (false, None) => {
                let reason = self.reason(
                    statement,
                    "Schema::table of a table that no migration creates",
                );
                tables.insert(table, Table::Unknown(reason));
                return;
            }
        };
        let result = self.body(closure, &mut columns);
        let folded = match result {
            Ok(()) => Table::Known(columns),
            Err(reason) => Table::Unknown(reason),
        };
        tables.insert(table, folded);
    }

    /// Applies the body of a `function (Blueprint $table) { .. }` closure to `columns`.
    fn body(&self, closure: &Expression<'_>, columns: &mut Vec<Column>) -> Result<(), String> {
        let Expression::Closure(closure) = closure else {
            return Err(self.reason(closure, "the second argument is not a closure"));
        };
        let form = closure.attribute_lists.is_empty()
            && closure.ampersand.is_none()
            && closure.use_clause.is_none()
            && closure
                .return_type_hint
                .as_ref()
                .is_none_or(|hint| matches!(hint.hint, Hint::Void(_)));
        let parameter = match closure.parameter_list.parameters.as_slice() {
            [parameter]
                if form
                    && parameter.modifiers.is_empty()
                    && parameter.attribute_lists.is_empty()
                    && parameter.ampersand.is_none()
                    && parameter.ellipsis.is_none()
                    && parameter.default_value.is_none()
                    && matches!(&parameter.hint, Some(Hint::Identifier(identifier))
                        if self.resolved(identifier) == b"illuminate\\database\\schema\\blueprint") =>
            {
                parameter.variable.name
            }
            _ => {
                return Err(self.reason(
                    closure,
                    "a closure other than function (Blueprint $table) with or without static and void",
                ));
            }
        };
        for statement in closure.body.statements.iter() {
            let Statement::Expression(expression) = statement else {
                return Err(
                    self.reason(statement, "a statement other than a call on the Blueprint")
                );
            };
            let Some(chain) = chain(expression.expression, parameter) else {
                return Err(
                    self.reason(statement, "a statement other than a call on the Blueprint")
                );
            };
            self.declaration(statement, &chain, columns)?;
        }
        Ok(())
    }

    /// Applies one chain `$table->m(..)->n(..)..` to `columns`.
    fn declaration(
        &self,
        statement: &Statement<'_>,
        chain: &[(&[u8], &ArgumentList<'_>)],
        columns: &mut Vec<Column>,
    ) -> Result<(), String> {
        let (method, arguments) = chain[0];
        let modifiers = &chain[1..];
        let unknown = |what: &str| Err(self.reason(statement, what));
        // A named argument binds what the tool reads by position when it names a
        // Blueprint method's `$column`, `$columns` or `$model` (`id(column: 'code_id')`);
        // nullable() goes through Fluent::__call, which takes its first value by any name.
        let plain = |list: &ArgumentList<'_>, read: &[&[u8]]| {
            list.arguments.iter().all(|argument| match argument {
                Argument::Positional(positional) => positional.ellipsis.is_none(),
                Argument::Named(named) => !read.is_empty() && !read.contains(&named.name.value),
            })
        };
        let nullable_named = modifiers
            .iter()
            .any(|(name, list)| name.eq_ignore_ascii_case(b"nullable") && !plain(list, &[]));
        if !plain(arguments, &[b"column", b"columns", b"model"]) || nullable_named {
            return unknown("a named or unpacked argument in a Blueprint call");
        }
        let lower = method.to_ascii_lowercase();
        let named = |default: Option<&str>| -> Option<String> {
            match positional_list(arguments).first() {
                Some(first) => string_literal(first),
                None => default.map(str::to_owned),
            }
        };
        // Table methods that change no column, and the drop of columns.
        match lower.as_slice() {
            b"unique" | b"index" | b"primary" | b"dropforeign" | b"dropunique" | b"dropindex" => {
                return if modifiers.is_empty() {
                    Ok(())
                } else {
                    unknown("a modifier on a table method")
                };
            }
            b"foreign" => {
                let fine = modifiers.iter().all(|(name, _)| {
                    [
                        &b"references"[..],
                        b"on",
                        b"ondelete",
                        b"onupdate",
                        b"cascadeondelete",
                        b"cascadeonupdate",
                        b"nullondelete",
                        b"restrictondelete",
                    ]
                    .contains(&name.to_ascii_lowercase().as_slice())
                });
                return if fine {
                    Ok(())
                } else {
                    unknown("a modifier of foreign() the tool does not read")
                };
            }
            b"dropcolumn" => {
                if !modifiers.is_empty() {
                    return unknown("a modifier on dropColumn()");
                }
                let names: Option<Vec<String>> = match positional_list(arguments).as_slice() {
                    [Expression::Array(array)] => array
                        .elements
                        .iter()
                        .map(|element| match element {
                            ArrayElement::Value(value) => string_literal(value.value),
                            _ => None,
                        })
                        .collect(),
                    [single] => string_literal(single).map(|name| vec![name]),
                    _ => None,
                };
                let Some(names) = names else {
                    return unknown("dropColumn() of a form the tool does not read");
                };
                for name in names {
                    let Some(at) = columns.iter().position(|c| c.name == name) else {
                        return unknown("dropColumn() of a column the table does not have");
                    };
                    columns.remove(at);
                }
                return Ok(());
            }
            _ => {}
        }
        // The columns a method declares, each with its name, kind and nullability.
        let declared: Vec<(Option<String>, Kind, bool)> = match lower.as_slice() {
            b"id" => vec![(named(Some("id")), Kind::Int, false)],
            b"foreignid"
            | b"biginteger"
            | b"unsignedbiginteger"
            | b"integer"
            | b"unsignedinteger"
            | b"smallinteger"
            | b"unsignedsmallinteger"
            | b"tinyinteger"
            | b"unsignedtinyinteger" => vec![(named(None), Kind::Int, false)],
            b"string" | b"char" | b"text" | b"mediumtext" | b"longtext" | b"enum" => {
                vec![(named(None), Kind::Text, false)]
            }
            b"uuid" => vec![(named(Some("uuid")), Kind::Text, false)],
            b"remembertoken" => vec![(Some("remember_token".to_owned()), Kind::Text, true)],
            b"boolean" | b"decimal" | b"float" | b"double" | b"timestamp" | b"datetime"
            | b"date" => vec![(named(None), Kind::Untyped, false)],
            b"timestamps" => vec![
                (Some("created_at".to_owned()), Kind::Untyped, true),
                (Some("updated_at".to_owned()), Kind::Untyped, true),
            ],
            b"softdeletes" => vec![(named(Some("deleted_at")), Kind::Untyped, true)],
            b"foreignidfor" => match self.foreign_id_for(arguments) {
                Some(name) => vec![(Some(name), Kind::Int, false)],
                None => {
                    return unknown(
                        "foreignIdFor() of a class that is not a model of the input with an int key",
                    );
                }
            },
            _ => return unknown("a Blueprint method the tool does not read"),
        };
        let foreign_id = matches!(lower.as_slice(), b"foreignid" | b"foreignidfor");
        let mut nullable_set: Option<bool> = None;
        let mut change = false;
        let mut foreign = false;
        for (name, arguments) in modifiers {
            let lower = name.to_ascii_lowercase();
            match lower.as_slice() {
                // After constrained(), a modifier belongs to the foreign key, as in Laravel.
                b"nullable" if !foreign => {
                    nullable_set = match positional_list(arguments).as_slice() {
                        [] => Some(true),
                        [Expression::Literal(Literal::True(_))] => Some(true),
                        [Expression::Literal(Literal::False(_))] => Some(false),
                        _ => {
                            return unknown("nullable() with an argument other than true or false");
                        }
                    };
                }
                // Only a foreign id column makes a foreign key of it; on another column
                // Laravel keeps the modifiers after it on the column.
                b"constrained" if foreign_id => foreign = true,
                b"constrained" => {
                    return unknown("constrained() after a column that is not a foreign id");
                }
                b"change" if !foreign => change = true,
                b"nullable"
                | b"default"
                | b"unsigned"
                | b"cascadeondelete"
                | b"nullondelete"
                | b"restrictondelete"
                | b"cascadeonupdate"
                | b"ondelete"
                | b"onupdate"
                | b"unique"
                | b"index"
                | b"primary"
                | b"after"
                | b"comment"
                | b"usecurrent"
                | b"usecurrentonupdate" => {}
                _ => return unknown("a column modifier the tool does not read"),
            }
        }
        for (name, kind, nullable) in declared {
            let Some(name) = name else {
                return unknown("a column name that is not a string literal");
            };
            let column = Column {
                name,
                kind,
                nullable: nullable_set.unwrap_or(nullable),
            };
            let existing = columns.iter().position(|c| c.name == column.name);
            match (change, existing) {
                (true, Some(at)) => columns[at] = column,
                (false, None) => columns.push(column),
                (true, None) => return unknown("change() of a column the table does not have"),
                (false, Some(_)) => return unknown("a column declared again without change()"),
            }
        }
        Ok(())
    }

    /// The column of `foreignIdFor(C::class)` or `foreignIdFor(C::class, '<c>')`, when `C`
    /// is a model of the input with an int key.
    fn foreign_id_for(&self, arguments: &ArgumentList<'_>) -> Option<String> {
        let list = positional_list(arguments);
        let (class, name) = match list.as_slice() {
            [class] => (class, None),
            [class, name] => (class, Some(string_literal(name)?)),
            _ => return None,
        };
        let Expression::Access(Access::ClassConstant(access)) = class else {
            return None;
        };
        let Expression::Identifier(identifier) = access.class else {
            return None;
        };
        let is_class = matches!(&access.constant, ClassLikeConstantSelector::Identifier(constant)
            if constant.value.eq_ignore_ascii_case(b"class"));
        if !is_class {
            return None;
        }
        let short = self.keyed.get(&self.resolved(identifier))?;
        Some(name.unwrap_or_else(|| format!("{}_id", laravel_snake(short))))
    }
}

/// Finds a call, `new`, partial application, `include` or `eval` in a migration that is not
/// one of the forms that change rows only (P2R-400).
struct RowsOnly<'r, 'n> {
    reader: &'r Reader<'n>,
    /// The lowercase names of the migration's own methods.
    own: Vec<Vec<u8>>,
}

impl RowsOnly<'_, '_> {
    fn allowed(&self, call: &Call<'_>) -> bool {
        match call {
            Call::StaticMethod(call) => {
                let ClassLikeMemberSelector::Identifier(method) = &call.method else {
                    return false;
                };
                let method = method.value.to_ascii_lowercase();
                let carbon = matches!(call.class, Expression::Identifier(identifier)
                    if [&b"carbon\\carbon"[..], b"carbon\\carbonimmutable", b"illuminate\\support\\carbon"]
                        .contains(&self.reader.resolved(identifier).as_slice()));
                (self.reader.facade(call.class, "db") && (method == b"table" || method == b"raw"))
                    || (carbon && method == b"now")
            }
            Call::Method(call) => {
                // down() is not checked, so a call of it is not.
                let own = matches!(call.object, Expression::Variable(Variable::Direct(variable))
                    if variable.name == b"$this")
                    && matches!(&call.method, ClassLikeMemberSelector::Identifier(method)
                        if self.own.contains(&method.value.to_ascii_lowercase())
                            && !method.value.eq_ignore_ascii_case(b"down"));
                own || self.chain(call)
            }
            Call::Function(call) => matches!(call.function, Expression::Identifier(identifier)
                if [&b"now"[..], b"date", b"count"]
                    .contains(&self.reader.resolved(identifier).as_slice())),
            Call::NullSafeMethod(_) => false,
        }
    }

    /// True for a call chained on `DB::table(..)`, `DB::raw(..)`, `now()` or `now()` of
    /// Carbon that does not reach the connection of a query, which runs any statement.
    fn chain(&self, call: &MethodCall<'_>) -> bool {
        !runs_any_statement(&call.method)
            && match call.object {
                Expression::Call(Call::Method(inner)) => self.chain(inner),
                Expression::Call(inner @ Call::StaticMethod(_)) => self.allowed(inner),
                Expression::Call(Call::Function(inner)) => {
                    matches!(inner.function, Expression::Identifier(identifier)
                        if self.reader.resolved(identifier) == b"now")
                }
                _ => false,
            }
    }
}

/// True for a method of a query that reaches its connection, which runs any statement
/// (`getProcessor()` runs its SQL through it), or one named by an expression.
fn runs_any_statement(method: &ClassLikeMemberSelector<'_>) -> bool {
    match method {
        ClassLikeMemberSelector::Identifier(method) => {
            method.value.eq_ignore_ascii_case(b"getConnection")
                || method.value.eq_ignore_ascii_case(b"getProcessor")
        }
        _ => true,
    }
}

/// Finds a closure or arrow function, which PHP 8.5 allows in a constant expression.
struct HasClosure;

impl<'ast, 'arena> Walker<'ast, 'arena, bool> for HasClosure {
    fn walk_in_closure(&self, _: &'ast Closure<'arena>, found: &mut bool) {
        *found = true;
    }

    fn walk_in_arrow_function(&self, _: &'ast ArrowFunction<'arena>, found: &mut bool) {
        *found = true;
    }
}

/// True for `$this`.
fn is_this(expression: &Expression<'_>) -> bool {
    matches!(expression, Expression::Variable(Variable::Direct(variable)) if variable.name == b"$this")
}

impl<'ast, 'arena> Walker<'ast, 'arena, bool> for RowsOnly<'_, '_> {
    fn walk_in_call(&self, call: &'ast Call<'arena>, rows: &mut bool) {
        if !self.allowed(call) {
            *rows = false;
        }
    }

    // `$this` as a receiver is checked by its call; anywhere else (`[$this, 'down']`, an
    // invokable) it may run a method without a call the tool sees.
    fn walk_method_call(&self, call: &'ast MethodCall<'arena>, rows: &mut bool) {
        if !is_this(call.object) {
            self.walk_expression(call.object, rows);
        }
        self.walk_class_like_member_selector(&call.method, rows);
        self.walk_argument_list(&call.argument_list, rows);
    }

    fn walk_property_access(&self, access: &'ast PropertyAccess<'arena>, rows: &mut bool) {
        if !is_this(access.object) {
            self.walk_expression(access.object, rows);
        }
        self.walk_class_like_member_selector(&access.property, rows);
    }

    fn walk_in_direct_variable(&self, variable: &'ast DirectVariable<'arena>, rows: &mut bool) {
        if variable.name == b"$this" {
            *rows = false;
        }
    }

    fn walk_in_instantiation(&self, _: &'ast Instantiation<'arena>, rows: &mut bool) {
        *rows = false;
    }

    fn walk_in_partial_application(&self, _: &'ast PartialApplication<'arena>, rows: &mut bool) {
        *rows = false;
    }

    fn walk_in_construct(&self, construct: &'ast Construct<'arena>, rows: &mut bool) {
        if !matches!(
            construct,
            Construct::Isset(_) | Construct::Empty(_) | Construct::Print(_)
        ) {
            *rows = false;
        }
    }
}

/// The calls of a chain on the variable `parameter`, innermost first, each with its method
/// name and arguments; `None` for any other expression.
fn chain<'e, 'a>(
    expression: &'e Expression<'a>,
    parameter: &[u8],
) -> Option<Vec<(&'e [u8], &'e ArgumentList<'a>)>> {
    let Expression::Call(Call::Method(call)) = expression else {
        return None;
    };
    let ClassLikeMemberSelector::Identifier(method) = &call.method else {
        return None;
    };
    let mut calls = match call.object {
        Expression::Variable(Variable::Direct(variable)) if variable.name == parameter => {
            Vec::new()
        }
        object => chain(object, parameter)?,
    };
    calls.push((method.value, &call.argument_list));
    Some(calls)
}

/// The values of the positional arguments, or none when one is named or unpacked.
fn positional_list<'l, 'a>(list: &'l ArgumentList<'a>) -> Vec<&'l Expression<'a>> {
    list.arguments
        .iter()
        .map_while(|argument| match argument {
            Argument::Positional(positional) if positional.ellipsis.is_none() => {
                Some(positional.value)
            }
            _ => None,
        })
        .collect()
}

/// Exactly `N` positional arguments.
fn positional<'l, 'a, const N: usize>(
    list: &'l ArgumentList<'a>,
) -> Option<[&'l Expression<'a>; N]> {
    if list.arguments.len() != N {
        return None;
    }
    positional_list(list).try_into().ok()
}

/// The text of a string literal without interpolation.
fn string_literal(expression: &Expression<'_>) -> Option<String> {
    match expression {
        Expression::Literal(Literal::String(string)) => {
            String::from_utf8(string.value?.to_vec()).ok()
        }
        _ => None,
    }
}
