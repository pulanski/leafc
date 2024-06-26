//! Generated by `syntax_gen`, do not edit by hand.

#![allow(non_snake_case)]
use crate::{
    ast::{
        self,
        support,
        AstChildren,
        AstNode,
    },
    SyntaxKind::{
        self,
        *,
    },
    SyntaxNode,
    SyntaxToken,
    T,
};
///Node defs
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Name {
    pub(crate) syntax: SyntaxNode,
}
impl Name {
    pub fn ident_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![ident])
    }
    pub fn self_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![self_value])
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NameRef {
    pub(crate) syntax: SyntaxNode,
}
impl NameRef {
    pub fn ident_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![ident])
    }
    pub fn self_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![self_value])
    }
    pub fn super_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![super])
    }
    pub fn pkg_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![package])
    }
    pub fn Self_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![self_type])
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Path {
    pub(crate) syntax: SyntaxNode,
}
impl Path {
    pub fn qualifier(&self) -> Option<Path> {
        support::child(&self.syntax)
    }
    pub fn coloncolon_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![::])
    }
    pub fn segment(&self) -> Option<PathSegment> {
        support::child(&self.syntax)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PathSegment {
    pub(crate) syntax: SyntaxNode,
}
impl PathSegment {
    pub fn coloncolon_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![::])
    }
    pub fn name_ref(&self) -> Option<NameRef> {
        support::child(&self.syntax)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TokenTree {
    pub(crate) syntax: SyntaxNode,
}
impl TokenTree {
    pub fn l_paren_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T!['('])
    }
    pub fn r_paren_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![')'])
    }
    pub fn l_curly_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T!['{'])
    }
    pub fn r_curly_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T!['}'])
    }
    pub fn l_brack_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T!['['])
    }
    pub fn r_brack_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![']'])
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Attr {
    pub(crate) syntax: SyntaxNode,
}
impl Attr {
    pub fn pound_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![#])
    }
    pub fn excl_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![!])
    }
    pub fn l_brack_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T!['['])
    }
    pub fn meta(&self) -> Option<Meta> {
        support::child(&self.syntax)
    }
    pub fn r_brack_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![']'])
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Meta {
    pub(crate) syntax: SyntaxNode,
}
impl Meta {
    pub fn path(&self) -> Option<Path> {
        support::child(&self.syntax)
    }
    pub fn token_tree(&self) -> Option<TokenTree> {
        support::child(&self.syntax)
    }
    pub fn eq_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![=])
    }
    pub fn expr(&self) -> Option<Expr> {
        support::child(&self.syntax)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SourceFile {
    pub(crate) syntax: SyntaxNode,
}
impl ast::HasAttrs for SourceFile {}
impl ast::HasModuleItem for SourceFile {}
impl ast::HasDocComments for SourceFile {}
impl SourceFile {}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Module {
    pub(crate) syntax: SyntaxNode,
}
impl ast::HasAttrs for Module {}
impl ast::HasName for Module {}
impl ast::HasVisibility for Module {}
impl ast::HasDocComments for Module {}
impl Module {
    pub fn mod_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![mod])
    }
    pub fn item_list(&self) -> Option<ItemList> {
        support::child(&self.syntax)
    }
    pub fn semicolon_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![;])
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Use {
    pub(crate) syntax: SyntaxNode,
}
impl ast::HasAttrs for Use {}
impl ast::HasVisibility for Use {}
impl ast::HasDocComments for Use {}
impl Use {
    pub fn use_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![use])
    }
    pub fn use_tree(&self) -> Option<UseTree> {
        support::child(&self.syntax)
    }
    pub fn semicolon_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![;])
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Visibility {
    pub(crate) syntax: SyntaxNode,
}
impl Visibility {
    pub fn pub_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![pub])
    }
    pub fn l_paren_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T!['('])
    }
    pub fn in_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![in])
    }
    pub fn path(&self) -> Option<Path> {
        support::child(&self.syntax)
    }
    pub fn r_paren_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![')'])
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ItemList {
    pub(crate) syntax: SyntaxNode,
}
impl ast::HasAttrs for ItemList {}
impl ast::HasModuleItem for ItemList {}
impl ItemList {
    pub fn l_curly_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T!['{'])
    }
    pub fn r_curly_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T!['}'])
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Rename {
    pub(crate) syntax: SyntaxNode,
}
impl ast::HasName for Rename {}
impl Rename {
    pub fn as_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![as])
    }
    pub fn underscore_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![_])
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UseTree {
    pub(crate) syntax: SyntaxNode,
}
impl UseTree {
    pub fn path(&self) -> Option<Path> {
        support::child(&self.syntax)
    }
    pub fn coloncolon_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![::])
    }
    pub fn star_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![*])
    }
    pub fn use_tree_list(&self) -> Option<UseTreeList> {
        support::child(&self.syntax)
    }
    pub fn rename(&self) -> Option<Rename> {
        support::child(&self.syntax)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UseTreeList {
    pub(crate) syntax: SyntaxNode,
}
impl UseTreeList {
    pub fn l_curly_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T!['{'])
    }
    pub fn use_trees(&self) -> AstChildren<UseTree> {
        support::children(&self.syntax)
    }
    pub fn r_curly_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T!['}'])
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RecordFieldList {
    pub(crate) syntax: SyntaxNode,
}
impl RecordFieldList {
    pub fn l_curly_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T!['{'])
    }
    pub fn fields(&self) -> AstChildren<RecordField> {
        support::children(&self.syntax)
    }
    pub fn r_curly_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T!['}'])
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RecordField {
    pub(crate) syntax: SyntaxNode,
}
impl ast::HasAttrs for RecordField {}
impl ast::HasName for RecordField {}
impl ast::HasVisibility for RecordField {}
impl ast::HasDocComments for RecordField {}
impl RecordField {
    pub fn colon_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![:])
    }
    pub fn ty(&self) -> Option<Type> {
        support::child(&self.syntax)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TupleFieldList {
    pub(crate) syntax: SyntaxNode,
}
impl TupleFieldList {
    pub fn l_paren_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T!['('])
    }
    pub fn fields(&self) -> AstChildren<TupleField> {
        support::children(&self.syntax)
    }
    pub fn r_paren_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![')'])
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TupleField {
    pub(crate) syntax: SyntaxNode,
}
impl ast::HasAttrs for TupleField {}
impl ast::HasVisibility for TupleField {}
impl ast::HasDocComments for TupleField {}
impl TupleField {
    pub fn ty(&self) -> Option<Type> {
        support::child(&self.syntax)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ExprStmt {
    pub(crate) syntax: SyntaxNode,
}
impl ExprStmt {
    pub fn expr(&self) -> Option<Expr> {
        support::child(&self.syntax)
    }
    pub fn semicolon_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![;])
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FieldExpr {
    pub(crate) syntax: SyntaxNode,
}
impl ast::HasAttrs for FieldExpr {}
impl FieldExpr {
    pub fn expr(&self) -> Option<Expr> {
        support::child(&self.syntax)
    }
    pub fn dot_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![.])
    }
    pub fn name_ref(&self) -> Option<NameRef> {
        support::child(&self.syntax)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RefExpr {
    pub(crate) syntax: SyntaxNode,
}
impl ast::HasAttrs for RefExpr {}
impl RefExpr {
    pub fn amp_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![&])
    }
    pub fn mut_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![mut])
    }
    pub fn const_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![const])
    }
    pub fn expr(&self) -> Option<Expr> {
        support::child(&self.syntax)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NeverType {
    pub(crate) syntax: SyntaxNode,
}
impl NeverType {
    pub fn excl_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![!])
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ParenType {
    pub(crate) syntax: SyntaxNode,
}
impl ParenType {
    pub fn l_paren_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T!['('])
    }
    pub fn ty(&self) -> Option<Type> {
        support::child(&self.syntax)
    }
    pub fn r_paren_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![')'])
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PathType {
    pub(crate) syntax: SyntaxNode,
}
impl PathType {
    pub fn path(&self) -> Option<Path> {
        support::child(&self.syntax)
    }
}
///Enum defs
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Expr {
    FieldExpr(FieldExpr),
    RefExpr(RefExpr),
}
impl ast::HasAttrs for Expr {}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Item {
    Module(Module),
    Use(Use),
}
impl ast::HasAttrs for Item {}
impl ast::HasDocComments for Item {}
impl ast::HasVisibility for Item {}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Type {
    NeverType(NeverType),
    ParenType(ParenType),
    PathType(PathType),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FieldList {
    RecordFieldList(RecordFieldList),
    TupleFieldList(TupleFieldList),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Stmt {
    ExprStmt(ExprStmt),
    Item(Item),
}
///Any node defs
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AnyHasAttrs {
    pub(crate) syntax: SyntaxNode,
}
impl ast::HasAttrs for AnyHasAttrs {}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AnyHasDocComments {
    pub(crate) syntax: SyntaxNode,
}
impl ast::HasDocComments for AnyHasDocComments {}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AnyHasModuleItem {
    pub(crate) syntax: SyntaxNode,
}
impl ast::HasModuleItem for AnyHasModuleItem {}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AnyHasName {
    pub(crate) syntax: SyntaxNode,
}
impl ast::HasName for AnyHasName {}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AnyHasVisibility {
    pub(crate) syntax: SyntaxNode,
}
impl ast::HasVisibility for AnyHasVisibility {}
///Node boilerplate
impl AstNode for Name {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == NAME
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for NameRef {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == NAME_REF
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for Path {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == PATH
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for PathSegment {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == PATH_SEGMENT
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for TokenTree {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == TOKEN_TREE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for Attr {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == ATTR
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for Meta {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == META
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for SourceFile {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SOURCE_FILE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for Module {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == MODULE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for Use {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == USE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for Visibility {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == VISIBILITY
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for ItemList {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == ITEM_LIST
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for Rename {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == RENAME
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for UseTree {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == USE_TREE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for UseTreeList {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == USE_TREE_LIST
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for RecordFieldList {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == RECORD_FIELD_LIST
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for RecordField {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == RECORD_FIELD
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for TupleFieldList {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == TUPLE_FIELD_LIST
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for TupleField {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == TUPLE_FIELD
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for ExprStmt {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == EXPR_STMT
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for FieldExpr {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == FIELD_EXPR
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for RefExpr {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == REF_EXPR
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for NeverType {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == NEVER_TYPE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for ParenType {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == PAREN_TYPE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AstNode for PathType {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == PATH_TYPE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
///Enum boilerplate
impl From<FieldExpr> for Expr {
    fn from(node: FieldExpr) -> Expr {
        Expr::FieldExpr(node)
    }
}
impl From<RefExpr> for Expr {
    fn from(node: RefExpr) -> Expr {
        Expr::RefExpr(node)
    }
}
impl AstNode for Expr {
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(kind, FIELD_EXPR | REF_EXPR)
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            FIELD_EXPR => Expr::FieldExpr(FieldExpr { syntax }),
            REF_EXPR => Expr::RefExpr(RefExpr { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Expr::FieldExpr(it) => &it.syntax,
            Expr::RefExpr(it) => &it.syntax,
        }
    }
}
impl From<Module> for Item {
    fn from(node: Module) -> Item {
        Item::Module(node)
    }
}
impl From<Use> for Item {
    fn from(node: Use) -> Item {
        Item::Use(node)
    }
}
impl AstNode for Item {
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(kind, MODULE | USE)
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            MODULE => Item::Module(Module { syntax }),
            USE => Item::Use(Use { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Item::Module(it) => &it.syntax,
            Item::Use(it) => &it.syntax,
        }
    }
}
impl From<NeverType> for Type {
    fn from(node: NeverType) -> Type {
        Type::NeverType(node)
    }
}
impl From<ParenType> for Type {
    fn from(node: ParenType) -> Type {
        Type::ParenType(node)
    }
}
impl From<PathType> for Type {
    fn from(node: PathType) -> Type {
        Type::PathType(node)
    }
}
impl AstNode for Type {
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(kind, NEVER_TYPE | PAREN_TYPE | PATH_TYPE)
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            NEVER_TYPE => Type::NeverType(NeverType { syntax }),
            PAREN_TYPE => Type::ParenType(ParenType { syntax }),
            PATH_TYPE => Type::PathType(PathType { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Type::NeverType(it) => &it.syntax,
            Type::ParenType(it) => &it.syntax,
            Type::PathType(it) => &it.syntax,
        }
    }
}
impl From<RecordFieldList> for FieldList {
    fn from(node: RecordFieldList) -> FieldList {
        FieldList::RecordFieldList(node)
    }
}
impl From<TupleFieldList> for FieldList {
    fn from(node: TupleFieldList) -> FieldList {
        FieldList::TupleFieldList(node)
    }
}
impl AstNode for FieldList {
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(kind, RECORD_FIELD_LIST | TUPLE_FIELD_LIST)
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            RECORD_FIELD_LIST => FieldList::RecordFieldList(RecordFieldList { syntax }),
            TUPLE_FIELD_LIST => FieldList::TupleFieldList(TupleFieldList { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            FieldList::RecordFieldList(it) => &it.syntax,
            FieldList::TupleFieldList(it) => &it.syntax,
        }
    }
}
impl From<ExprStmt> for Stmt {
    fn from(node: ExprStmt) -> Stmt {
        Stmt::ExprStmt(node)
    }
}
impl From<Item> for Stmt {
    fn from(node: Item) -> Stmt {
        Stmt::Item(node)
    }
}
///Any node boilerplate
impl AnyHasAttrs {
    #[inline]
    pub fn new<T: ast::HasAttrs>(node: T) -> AnyHasAttrs {
        AnyHasAttrs { syntax: node.syntax().clone() }
    }
}
impl AstNode for AnyHasAttrs {
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(
            kind,
            SOURCE_FILE |
                MODULE |
                USE |
                ITEM_LIST |
                RECORD_FIELD |
                TUPLE_FIELD |
                FIELD_EXPR |
                REF_EXPR
        )
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        Self::can_cast(syntax.kind()).then_some(AnyHasAttrs { syntax })
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AnyHasDocComments {
    #[inline]
    pub fn new<T: ast::HasDocComments>(node: T) -> AnyHasDocComments {
        AnyHasDocComments { syntax: node.syntax().clone() }
    }
}
impl AstNode for AnyHasDocComments {
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(kind, SOURCE_FILE | MODULE | USE | RECORD_FIELD | TUPLE_FIELD)
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        Self::can_cast(syntax.kind()).then_some(AnyHasDocComments { syntax })
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AnyHasModuleItem {
    #[inline]
    pub fn new<T: ast::HasModuleItem>(node: T) -> AnyHasModuleItem {
        AnyHasModuleItem { syntax: node.syntax().clone() }
    }
}
impl AstNode for AnyHasModuleItem {
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(kind, SOURCE_FILE | ITEM_LIST)
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        Self::can_cast(syntax.kind()).then_some(AnyHasModuleItem { syntax })
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AnyHasName {
    #[inline]
    pub fn new<T: ast::HasName>(node: T) -> AnyHasName {
        AnyHasName { syntax: node.syntax().clone() }
    }
}
impl AstNode for AnyHasName {
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(kind, MODULE | RENAME | RECORD_FIELD)
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        Self::can_cast(syntax.kind()).then_some(AnyHasName { syntax })
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
impl AnyHasVisibility {
    #[inline]
    pub fn new<T: ast::HasVisibility>(node: T) -> AnyHasVisibility {
        AnyHasVisibility { syntax: node.syntax().clone() }
    }
}
impl AstNode for AnyHasVisibility {
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(kind, MODULE | USE | RECORD_FIELD | TUPLE_FIELD)
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        Self::can_cast(syntax.kind()).then_some(AnyHasVisibility { syntax })
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
}
///Display impls
impl std::fmt::Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for FieldList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for Stmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for Name {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for NameRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for PathSegment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TokenTree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for Attr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for Meta {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for SourceFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for Module {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for Use {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for Visibility {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for ItemList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for Rename {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for UseTree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for UseTreeList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for RecordFieldList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for RecordField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TupleFieldList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TupleField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for ExprStmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for FieldExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for RefExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for NeverType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for ParenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for PathType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
