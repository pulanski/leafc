use self::node_id::NodeId;
use leafc_lexer::lexer::TokenStream;
use leafc_utils::location::Span;

pub mod generated;

pub mod node_id;

// pub struct Pat {
//     pub id: NodeId,
//     pub kind: PatKind,
// }

pub struct Ty {
    pub id: NodeId,
    pub kind: TyKind,
    pub span: Span,
    pub tokens: TokenStream,
}

pub enum TyKind {
    Tup(Vec<Ty>),

    Slice(Box<Ty>),

    Array(Box<Ty>, u64),
    // Ptr(Box<Ty>),

    // Ref(Box<Ty>),
    // Path(Path),
}
