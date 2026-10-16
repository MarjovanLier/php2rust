//! Laravel's collections as lists of their elements, read in order (P2R-440 to P2R-443).

use mago_syntax::cst::*;

use super::Converter;
use crate::printer::RExpr;
use crate::types::{COLLECTIONS, RustType};

/// The problem of any use of a collection value that section 45 does not convert (P2R-443).
pub(super) const COLLECTION_USE: &str = "a collection converts only in a foreach without a key, in count(), and in the methods count(), isEmpty(), isNotEmpty() and first() without arguments (P2R-442, P2R-443)";

impl Converter<'_> {
    /// True when an included file, never the input, declares Laravel's
    /// `Illuminate\Support\Collection`, and the Eloquent collection too when anything
    /// declares it (P2R-340, P2R-440).
    pub(super) fn included_collections(&self) -> bool {
        let included = |name: &[u8]| {
            self.codebase.get_class_like(name).map(|class| {
                !self
                    .files
                    .iter()
                    .any(|analysed| analysed.file == class.span.file_id)
            })
        };
        included(COLLECTIONS[0]) == Some(true) && included(COLLECTIONS[1]) != Some(false)
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
