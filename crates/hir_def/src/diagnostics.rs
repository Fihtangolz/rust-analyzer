//! Diagnostics produced by `hir_def`.

use std::any::Any;

use hir_expand::diagnostics::Diagnostic;
use syntax::{ast, AstPtr, SyntaxNodePtr};

use hir_expand::{HirFileId, InFile};

#[derive(Debug)]
pub struct UnresolvedModule {
    pub file: HirFileId,
    pub decl: AstPtr<ast::Module>,
    pub candidate: String,
}

impl Diagnostic for UnresolvedModule {
    fn name(&self) -> &'static str {
        "unresolved-module"
    }
    fn message(&self) -> String {
        "unresolved module".to_string()
    }
    fn display_source(&self) -> InFile<SyntaxNodePtr> {
        InFile::new(self.file, self.decl.clone().into())
    }
    fn as_any(&self) -> &(dyn Any + Send + 'static) {
        self
    }
}
