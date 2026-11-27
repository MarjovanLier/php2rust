//! The laravel-data package as the included files declare it: its class `Data`, read once
//! per run, which every Data class of the input asks about (P2R-350, P2R-380).

use std::collections::HashSet;

use mago_codex::metadata::CodebaseMetadata;
use mago_codex::metadata::class_like::ClassLikeMetadata;
use mago_database::file::FileId;

use super::classes::IMPLICIT_METHODS;

/// The lowercase key of Spatie's `Data`; Mago finds a class-like and its methods in any
/// letter case (P2R-350).
pub(super) const DATA: &[u8] = b"spatie\\laraveldata\\data";

/// What the included `Data` is, read from Mago's codebase.
pub(super) struct DataPackage<'c> {
    codebase: &'c CodebaseMetadata,
    /// `Data` when an included file declares it; one the input declares is a class of the
    /// input, and Mago's stubs do not count (P2R-340, P2R-350).
    data: Option<&'c ClassLikeMetadata>,
    /// Why the included `Data` is not the shape of version 4.18.0, in the order the stops
    /// print, each with its message (P2R-350).
    pub(super) problems: Vec<String>,
}

impl<'c> DataPackage<'c> {
    /// Reads `Data` from a codebase that Mago has populated, so its inherited methods and
    /// interfaces are known.
    pub(super) fn read(codebase: &'c CodebaseMetadata, included: &HashSet<FileId>) -> Self {
        let data = codebase
            .get_class_like(DATA)
            .filter(|data| included.contains(&data.span.file_id));
        let mut problems = Vec::new();
        if let Some(data) = data {
            // The override check needs every method of Data, also those of its traits.
            if data
                .used_traits
                .iter()
                .chain(data.all_parent_interfaces.iter())
                .any(|name| !codebase.class_like_exists(name.as_bytes()))
            {
                problems.push("Spatie\\LaravelData\\Data names a trait or interface that no included file declares, so its methods are not all known; include the whole package (P2R-350)".to_owned());
            }
            // Data 4.18 has no parent, whose methods the override check would also need.
            if data.direct_parent_class.is_some() {
                problems.push(
                    "Spatie\\LaravelData\\Data extends another class, which version 4.18.0 does not (P2R-350)".to_owned(),
                );
            }
            // PHP calls these by itself, so a Data that has one is not the 4.18 shape.
            if let Some(magic) = IMPLICIT_METHODS
                .iter()
                .find(|name| codebase.method_exists(DATA, name.as_bytes()))
            {
                problems.push(format!(
                    "Spatie\\LaravelData\\Data has the method {magic}, which PHP calls by itself (P2R-350)"
                ));
            }
        }
        Self {
            codebase,
            data,
            problems,
        }
    }

    /// True when an included file declares `Data` (P2R-350).
    pub(super) fn included(&self) -> bool {
        self.data.is_some()
    }

    /// True when `Data` has a method named `name`, in any letter case, which a Data class
    /// may not override (P2R-350).
    pub(super) fn has_method(&self, name: &[u8]) -> bool {
        self.codebase.method_exists(DATA, name)
    }

    /// True when `Data`, a trait of it or an interface of it has a real method named
    /// `name`, in any letter case, rather than only an `@method` tag or nothing, so a hook
    /// of that name would override it (P2R-350, P2R-380).
    pub(super) fn real_method(&self, name: &[u8]) -> bool {
        let Some(data) = self.data else {
            return false;
        };
        let real = |owner: &[u8]| {
            self.codebase
                .get_method(owner, name)
                .is_some_and(|found| !found.flags.is_magic_method())
        };
        let renames = |class: &ClassLikeMetadata| {
            class.trait_alias_map.iter().any(|(original, alias)| {
                [original, alias]
                    .iter()
                    .any(|word| word.as_bytes().eq_ignore_ascii_case(name))
            })
        };
        self.codebase
            .get_declaring_method(DATA, name)
            .is_some_and(|found| !found.flags.is_magic_method())
            // Mago takes Data's own @method tag over a real method of its trait or interface,
            // also over a trait method that Data or one of its traits renames to the name.
            || renames(data)
            || data.used_traits.iter().any(|owner| {
                self.codebase
                    .get_class_like(owner.as_bytes())
                    .is_some_and(renames)
            })
            || data
                .used_traits
                .iter()
                .chain(data.all_parent_interfaces.iter())
                .any(|owner| real(owner.as_bytes()))
    }
}

#[cfg(test)]
mod tests {
    use std::borrow::Cow;

    use mago_allocator::LocalArena;
    use mago_codex::populator::populate_codebase;
    use mago_codex::reference::SymbolReferences;
    use mago_codex::scanner::scan_program;
    use mago_database::file::{File, FileType};
    use mago_names::resolver::NameResolver;
    use mago_php_version::PHPVersion;
    use mago_syntax::parser::parse_file;

    use super::*;

    /// The codebase of `source`, scanned and populated as the converter sees it, with the
    /// file in the included set when `included`.
    fn populated(source: &str, included: bool) -> (CodebaseMetadata, HashSet<FileId>) {
        let file = File::new(
            Cow::Borrowed(b"data.php"),
            FileType::Vendored,
            None,
            Cow::Owned(source.as_bytes().to_vec()),
        );
        let arena = LocalArena::new();
        let program = parse_file(&arena, &file);
        let names = NameResolver::new(&arena).resolve(program);
        let mut codebase = CodebaseMetadata::new();
        codebase.extend(scan_program(
            &arena,
            &file,
            program,
            &names,
            PHPVersion::PHP85,
        ));
        populate_codebase(
            &mut codebase,
            &mut SymbolReferences::new(),
            Default::default(),
            Default::default(),
        );
        let files = if included {
            HashSet::from([file.id])
        } else {
            HashSet::new()
        };
        (codebase, files)
    }

    /// The shape problems of the included `Data` that `body` declares, in order.
    fn problems(body: &str) -> Vec<String> {
        let (codebase, included) = populated(
            &format!("<?php\nnamespace Spatie\\LaravelData;\n{body}\n"),
            true,
        );
        let package = DataPackage::read(&codebase, &included);
        assert!(package.included());
        package.problems
    }

    #[test]
    fn a_data_of_the_4_18_shape_has_no_problems() {
        assert!(problems(
            "interface Contract {}\ntrait Base {}\nabstract class Data implements Contract { use Base; }"
        )
        .is_empty());
    }

    #[test]
    fn a_trait_no_file_declares_is_a_problem() {
        assert_eq!(
            problems("abstract class Data { use Missing; }"),
            [
                "Spatie\\LaravelData\\Data names a trait or interface that no included file declares, so its methods are not all known; include the whole package (P2R-350)"
            ]
        );
    }

    #[test]
    fn a_parent_class_is_a_problem() {
        assert_eq!(
            problems("class Base {}\nabstract class Data extends Base {}"),
            [
                "Spatie\\LaravelData\\Data extends another class, which version 4.18.0 does not (P2R-350)"
            ]
        );
    }

    #[test]
    fn a_method_php_calls_by_itself_is_a_problem() {
        assert_eq!(
            problems(
                "abstract class Data { public function __get(string $name): mixed { return null; } }"
            ),
            ["Spatie\\LaravelData\\Data has the method __get, which PHP calls by itself (P2R-350)"]
        );
    }

    /// Every problem stops, not only the first, in the order the stops print.
    #[test]
    fn each_problem_counts_in_order() {
        let found = problems(
            "class Base {}\nabstract class Data extends Base { use Missing; public function __call(string $name, array $arguments): mixed { return null; } }",
        );
        assert_eq!(found.len(), 3, "{found:?}");
        assert!(found[0].contains("names a trait or interface"));
        assert!(found[1].contains("extends another class"));
        assert!(found[2].contains("has the method __call"));
    }

    #[test]
    fn no_data_means_no_package() {
        let (codebase, included) = populated("<?php\nclass Other {}\n", true);
        let package = DataPackage::read(&codebase, &included);
        assert!(!package.included());
        assert!(package.problems.is_empty());
        assert!(!package.real_method(b"rules"));
    }

    /// A `Data` the input declares is a class of the input, not the package (P2R-340).
    #[test]
    fn a_data_outside_the_included_files_is_no_package() {
        let (codebase, included) = populated(
            "<?php\nnamespace Spatie\\LaravelData;\nabstract class Data { use Missing; }\n",
            false,
        );
        let package = DataPackage::read(&codebase, &included);
        assert!(!package.included());
        assert!(package.problems.is_empty());
    }

    #[test]
    fn real_method_reads_data_its_traits_aliases_and_interfaces() {
        let (codebase, included) = populated(
            "<?php\nnamespace Spatie\\LaravelData;\n\
             interface Contract { public static function fromContract(): void; }\n\
             trait Base { public static function rules(): array { return []; } public static function original(): void {} public static function prepareForPipeline(): void {} }\n\
             trait Outer { use Base { original as renamedByTrait; } }\n\
             /**\n * @method static void tagged()\n * @method static void prepareForPipeline()\n */\n\
             abstract class Data implements Contract { use Base { original as renamedByData; } use Outer; public static function own(): void {} }\n",
            true,
        );
        let package = DataPackage::read(&codebase, &included);
        let real = |name: &str| package.real_method(name.as_bytes());
        // Data's own method, and a trait method in another letter case.
        assert!(real("own"));
        assert!(real("RULES"));
        // A trait method that Data or one of its traits renames, under either name.
        assert!(real("renamedByData"));
        assert!(real("renamedByTrait"));
        assert!(real("original"));
        // An interface method.
        assert!(real("fromContract"));
        // Data's @method tag hides the trait's real method from Data, not from the trait.
        assert!(real("prepareForPipeline"));
        // A name only an @method tag gives, and a name nothing gives.
        assert!(!real("tagged"));
        assert!(!real("Tagged"));
        assert!(!real("messages"));
    }
}
