//! Laravel's collections as lists of their elements, read in order (P2R-440 to P2R-443).

use mago_syntax::cst::*;

use super::Converter;
use crate::printer::RExpr;
use crate::types::{self, COLLECTIONS, RustType};

/// The problem of any use of a collection value that section 45 does not convert (P2R-443).
pub(super) const COLLECTION_USE: &str = "a collection converts only in a foreach without a key, in count(), and in the methods count(), isEmpty(), isNotEmpty() and first() without arguments (P2R-442, P2R-443)";

impl Converter<'_> {
    /// The lowercase names of Laravel's collection classes that an included file, never
    /// the input, declares. Eloquent's counts only beside the base class it inherits its
    /// methods from, which the input may otherwise declare itself (P2R-340, P2R-440).
    pub(super) fn included_collections(&self) -> Vec<&'static [u8]> {
        let included = |name: &[u8]| {
            self.codebase.get_class_like(name).is_some_and(|class| {
                !self
                    .files
                    .iter()
                    .any(|analysed| analysed.file == class.span.file_id)
            })
        };
        match (included(COLLECTIONS[0]), included(COLLECTIONS[1])) {
            (true, true) => COLLECTIONS.to_vec(),
            (true, false) => vec![COLLECTIONS[0]],
            (false, _) => Vec::new(),
        }
    }

    /// True when `call` is `isEmpty()`, `isNotEmpty()` or `first()` on a collection value,
    /// whose Rust forms rustc warns about when nothing reads them (P2R-443).
    pub(super) fn collection_read(&self, call: &MethodCall<'_>) -> bool {
        let reads = matches!(&call.method, ClassLikeMemberSelector::Identifier(name)
            if [&b"isempty"[..], b"isnotempty", b"first"].contains(&name.value.to_ascii_lowercase().as_slice()));
        reads
            && self
                .expression_type(call.object)
                .and_then(|union| types::rust_type(union, &self.type_names).ok())
                .is_some_and(|ty| matches!(ty, RustType::Collection(_)))
    }

    /// `$c->count()`, `$c->isEmpty()`, `$c->isNotEmpty()` and `$c->first()` on a collection
    /// of `element`, each without arguments (P2R-442).
    pub(super) fn collection_method(
        &mut self,
        call: &MethodCall<'_>,
        node: &Expression<'_>,
        object: RExpr,
        element: &RustType,
    ) -> Option<(RExpr, RustType)> {
        let name = match &call.method {
            ClassLikeMemberSelector::Identifier(name) => name.value.to_ascii_lowercase(),
            _ => Vec::new(),
        };
        if !call.argument_list.arguments.is_empty() {
            self.record_refused_arguments(&call.argument_list);
            self.refuse(node, COLLECTION_USE);
            return None;
        }
        let read = |method: &str| {
            RExpr::MethodCall(Box::new(object.clone()), method.to_owned(), Vec::new())
        };
        match name.as_slice() {
            b"count" => Some((RExpr::Len(Box::new(object)), RustType::I64)),
            b"isempty" => Some((read("is_empty"), RustType::Bool)),
            b"isnotempty" => Some((RExpr::Not(Box::new(read("is_empty"))), RustType::Bool)),
            // `Option<PhpDateTime>` is `DateTime|false`, never `DateTime|null` (P2R-330).
            b"first" if *element == RustType::DateTime => {
                self.refuse(
                    node,
                    "first() of a collection of DateTime does not convert, because Option<PhpDateTime> stands for DateTime|false (P2R-442)",
                );
                None
            }
            // Laravel's `first()` gives the first element, or null for an empty collection.
            b"first" => Some((
                RExpr::Method(Box::new(read("first")), "cloned"),
                RustType::Option(Box::new(element.clone())),
            )),
            _ => {
                self.refuse(node, COLLECTION_USE);
                None
            }
        }
    }
}
