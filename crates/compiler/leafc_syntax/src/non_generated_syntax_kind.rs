// // use crate::ast::generated::kinds::SyntaxKind; // TODO: should be this once syntaxgen is finished
// use crate::syntax_kind_nongen::SyntaxKind; // TODO: remove this once syntaxgen is finished
use leafc_lexer::TokenKind;
use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};
use rowan::{GreenNodeBuilder, Language};

/// The **kind** of a **syntax node** found in the **syntax tree**.
///
/// These can either be [**tokens**](TokenKind) generated by the [**lexer**][leafc_lexer]
/// (e.g. [`DOT`][TokenKind::DOT], [`IDENTIFIER`][TokenKind::IDENTIFIER], [`WHITESPACE`][TokenKind::WHITESPACE],
/// etc.) or **non-terminal AST nodes** used to represent components within the **grammar** (e.g. `EXPR`,
/// `TYPE`, `PATTERN`, etc.) and parse the **source code**. In general, syntax nodes are a **supset** of
/// the **tokens** generated by the **lexer** and form the basis of both leafc's **concrete syntax trees** and
/// **abstract syntax trees**.
#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive, ToPrimitive, Hash, Eq, PartialOrd, Ord)]
#[repr(u16)]
pub enum SyntaxKind {
    ////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    // Tokens
    ////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    WHITESPACE,
    ERROR,
    IDENTIFIER,
    COMMENT,
    DOC_COMMENT,
    ////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    // Literals
    ////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    RUNE,
    STRING,
    RAW_STRING,
    INTEGER,
    FLOAT,
    LIFETIME,
    ////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    /// Superscript literals
    ////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    INTEGER_SUP,
    FLOAT_SUP,
    ////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    // Mathematical constants
    ////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    PI,
    EULER,
    PHI,
    TAU,
    CATALAN,
    EULERGAMMA,
    INF,
    NAN,
    ////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    /// Keywords
    ////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    ABSTRACT_KW,
    ASYNC_KW,
    AWAIT_KW,
    EXTERN_KW,
    FINAL_KW,
    IS_KW,
    AND_KW,
    AS_KW,
    BREAK_KW,
    CONST_KW,
    CONTINUE_KW,
    DO_KW,
    DYN_KW,
    ELSE_KW,
    ENUM_KW,
    FALSE_KW,
    FN_KW,
    FOR_KW,
    IF_KW,
    IMPL_KW,
    IMPORT_KW,
    IN_KW,
    LET_KW,
    LOOP_KW,
    MATCH_KW,
    MISSING_KW,
    MOD_KW,
    MOVE_KW,
    MUT_KW,
    NOT_KW,
    OR_KW,
    PACKAGE_KW,
    PUB_KW,
    RETURN_KW,
    SELF_KW,
    STATIC_KW,
    STRUCT_KW,
    SUPER_KW,
    TRAIT_KW,
    TRUE_KW,
    TYPE_KW,
    UNSAFE_KW,
    USE_KW,
    WHERE_KW,
    WHILE_KW,
    YIELD_KW,
    ////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    // Punctuation
    ////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    PLUS,
    MINUS,
    STAR,
    SLASH,
    PERCENT,
    CARET,
    BANG,
    AMPERSAND,
    PIPE,
    DOUBLE_AMPERSAND,
    DOUBLE_PIPE,
    SHL,
    SHR,
    PLUS_EQ,
    MINUS_EQ,
    STAR_EQ,
    SLASH_EQ,
    PERCENT_EQ,
    CARET_EQ,
    AMPERSAND_EQ,
    PIPE_EQ,
    SHL_EQ,
    SHR_EQ,
    EQ,
    EQEQ,
    NE,
    GT,
    LT,
    GE,
    LE,
    AT,
    UNDERSCORE,
    DOT,
    DOTDOT,
    DOTDOTEQ,
    COMMA,
    SEMICOLON,
    COLON,
    PATHSEP,
    RARROW,
    FATARROW,
    HASH,
    DOLLAR,
    QMARK,
    TILDE,
    L_BRACKET,
    R_BRACKET,
    L_PAREN,
    R_PAREN,
    L_BRACE,
    R_BRACE,
    L_PAREN_SUPERSCRIPT,
    R_PAREN_SUPERSCRIPT,
    L_ARROW,
    DOUBLE_STAR,
}

impl From<TokenKind> for SyntaxKind {
    /// Converts a [**token kind**](TokenKind) into a [**syntax kind**](SyntaxKind).
    ///
    /// This is used to convert the **tokens** generated by the **lexer** into **syntax nodes** in
    /// the **parse tree**.
    fn from(token_kind: TokenKind) -> Self {
        match token_kind {
            TokenKind::WHITESPACE => SyntaxKind::WHITESPACE,
            TokenKind::ERROR => SyntaxKind::ERROR,
            TokenKind::IDENTIFIER => SyntaxKind::IDENTIFIER,
            TokenKind::COMMENT => SyntaxKind::COMMENT,
            TokenKind::DOC_COMMENT => SyntaxKind::DOC_COMMENT,
            TokenKind::RUNE => SyntaxKind::RUNE,
            TokenKind::STRING => SyntaxKind::STRING,
            TokenKind::RAW_STRING => SyntaxKind::RAW_STRING,
            TokenKind::INTEGER => SyntaxKind::INTEGER,
            TokenKind::FLOAT => SyntaxKind::FLOAT,
            TokenKind::LIFETIME => SyntaxKind::LIFETIME,
            TokenKind::INTEGER_SUP => SyntaxKind::INTEGER_SUP,
            TokenKind::FLOAT_SUP => SyntaxKind::FLOAT_SUP,
            TokenKind::PI => SyntaxKind::PI,
            TokenKind::EULER => SyntaxKind::EULER,
            TokenKind::PHI => SyntaxKind::PHI,
            TokenKind::TAU => SyntaxKind::TAU,
            TokenKind::CATALAN => SyntaxKind::CATALAN,
            TokenKind::EULERGAMMA => SyntaxKind::EULERGAMMA,
            TokenKind::INF => SyntaxKind::INF,
            TokenKind::NAN => SyntaxKind::NAN,
            TokenKind::ABSTRACT_KW => SyntaxKind::ABSTRACT_KW,
            TokenKind::ASYNC_KW => SyntaxKind::ASYNC_KW,
            TokenKind::AWAIT_KW => SyntaxKind::AWAIT_KW,
            TokenKind::EXTERN_KW => SyntaxKind::EXTERN_KW,
            TokenKind::FINAL_KW => SyntaxKind::FINAL_KW,
            TokenKind::IS_KW => SyntaxKind::IS_KW,
            TokenKind::AND_KW => SyntaxKind::AND_KW,
            TokenKind::AS_KW => SyntaxKind::AS_KW,
            TokenKind::BREAK_KW => SyntaxKind::BREAK_KW,
            TokenKind::CONST_KW => SyntaxKind::CONST_KW,
            TokenKind::CONTINUE_KW => SyntaxKind::CONTINUE_KW,
            TokenKind::DO_KW => SyntaxKind::DO_KW,
            TokenKind::DYN_KW => SyntaxKind::DYN_KW,
            TokenKind::ELSE_KW => SyntaxKind::ELSE_KW,
            TokenKind::ENUM_KW => SyntaxKind::ENUM_KW,
            TokenKind::FALSE_KW => SyntaxKind::FALSE_KW,
            TokenKind::FN_KW => SyntaxKind::FN_KW,
            TokenKind::FOR_KW => SyntaxKind::FOR_KW,
            TokenKind::IF_KW => SyntaxKind::IF_KW,
            TokenKind::IMPL_KW => SyntaxKind::IMPL_KW,
            TokenKind::IMPORT_KW => SyntaxKind::IMPORT_KW,
            TokenKind::IN_KW => SyntaxKind::IN_KW,
            TokenKind::LET_KW => SyntaxKind::LET_KW,
            TokenKind::LOOP_KW => SyntaxKind::LOOP_KW,
            TokenKind::MATCH_KW => SyntaxKind::MATCH_KW,
            TokenKind::MISSING_KW => SyntaxKind::MISSING_KW,
            TokenKind::MOD_KW => SyntaxKind::MOD_KW,
            TokenKind::MOVE_KW => SyntaxKind::MOVE_KW,
            TokenKind::MUT_KW => SyntaxKind::MUT_KW,
            TokenKind::NOT_KW => SyntaxKind::NOT_KW,
            TokenKind::OR_KW => SyntaxKind::OR_KW,
            TokenKind::PACKAGE_KW => todo!(),
            TokenKind::PUB_KW => todo!(),
            TokenKind::RETURN_KW => todo!(),
            TokenKind::SELF_VALUE_KW => todo!(),
            TokenKind::SELF_TYPE_KW => todo!(),
            TokenKind::STATIC_KW => todo!(),
            TokenKind::STRUCT_KW => todo!(),
            TokenKind::SUPER_KW => todo!(),
            TokenKind::TRAIT_KW => todo!(),
            TokenKind::TRUE_KW => todo!(),
            TokenKind::TYPE_KW => todo!(),
            TokenKind::UNSAFE_KW => todo!(),
            TokenKind::USE_KW => todo!(),
            TokenKind::WHERE_KW => todo!(),
            TokenKind::WHILE_KW => todo!(),
            TokenKind::YIELD_KW => todo!(),
            TokenKind::PLUS => todo!(),
            TokenKind::MINUS => todo!(),
            TokenKind::STAR => todo!(),
            TokenKind::SLASH => todo!(),
            TokenKind::PERCENT => todo!(),
            TokenKind::CARET => todo!(),
            TokenKind::BANG => todo!(),
            TokenKind::AMPERSAND => todo!(),
            TokenKind::PIPE => todo!(),
            TokenKind::DOUBLE_AMPERSAND => todo!(),
            TokenKind::DOUBLE_PIPE => todo!(),
            TokenKind::SHL => todo!(),
            TokenKind::SHR => todo!(),
            TokenKind::PLUS_EQ => todo!(),
            TokenKind::MINUS_EQ => todo!(),
            TokenKind::STAR_EQ => todo!(),
            TokenKind::SLASH_EQ => todo!(),
            TokenKind::PERCENT_EQ => todo!(),
            TokenKind::CARET_EQ => todo!(),
            TokenKind::AMPERSAND_EQ => todo!(),
            TokenKind::PIPE_EQ => todo!(),
            TokenKind::SHL_EQ => todo!(),
            TokenKind::SHR_EQ => todo!(),
            TokenKind::EQ => todo!(),
            TokenKind::EQEQ => todo!(),
            TokenKind::NE => todo!(),
            TokenKind::GT => todo!(),
            TokenKind::LT => todo!(),
            TokenKind::GE => todo!(),
            TokenKind::LE => todo!(),
            TokenKind::AT => todo!(),
            TokenKind::UNDERSCORE => todo!(),
            TokenKind::DOT => todo!(),
            TokenKind::DOTDOT => todo!(),
            TokenKind::DOTDOTEQ => todo!(),
            TokenKind::COMMA => todo!(),
            TokenKind::SEMICOLON => todo!(),
            TokenKind::COLON => todo!(),
            TokenKind::PATHSEP => todo!(),
            TokenKind::RARROW => todo!(),
            TokenKind::FATARROW => todo!(),
            TokenKind::HASH => todo!(),
            TokenKind::DOLLAR => todo!(),
            TokenKind::QMARK => todo!(),
            TokenKind::TILDE => todo!(),
            TokenKind::L_BRACKET => todo!(),
            TokenKind::R_BRACKET => todo!(),
            TokenKind::L_PAREN => todo!(),
            TokenKind::R_PAREN => todo!(),
            TokenKind::L_BRACE => todo!(),
            TokenKind::R_BRACE => todo!(),
            TokenKind::L_PAREN_SUPERSCRIPT => todo!(),
            TokenKind::R_PAREN_SUPERSCRIPT => todo!(),
            TokenKind::L_ARROW => todo!(),
            TokenKind::DOUBLE_STAR => todo!(),
        }
    }
}

// impl SyntaxKind {
//     pub fn is_keyword(self) -> bool {
//         matches!(
//             self,
//             SyntaxKind::AS_KW
//                 | SyntaxKind::BREAK_KW
//                 | SyntaxKind::CONTINUE_KW
//                 | SyntaxKind::DO_KW
//                 | SyntaxKind::DYN_KW
//                 | SyntaxKind::ELSE_KW
//                 | SyntaxKind::ENUM_KW
//                 | SyntaxKind::FALSE_KW
//                 | SyntaxKind::FN_KW
//                 | SyntaxKind::FOR_KW
//                 | SyntaxKind::IF_KW
//                 | SyntaxKind::IMPL_KW
//                 | SyntaxKind::IMPORT_KW
//                 | SyntaxKind::IN_KW
//                 | SyntaxKind::LET_KW
//                 | SyntaxKind::LOOP_KW
//                 | SyntaxKind::MATCH_KW
//                 | SyntaxKind::MISSING_KW
//                 | SyntaxKind::MOD_KW
//                 | SyntaxKind::MOVE_KW
//                 | SyntaxKind::MUT_KW
//                 | SyntaxKind::NOT_KW
//                 | SyntaxKind::OR_KW
//                 | SyntaxKind::PACKAGE_KW
//                 | SyntaxKind::PUB_KW
//                 | SyntaxKind::RETURN_KW
//                 | SyntaxKind::SELF_VALUE_KW
//                 | SyntaxKind::SELF_TYPE_KW
//                 | SyntaxKind::STATIC_KW
//                 | SyntaxKind::STRUCT_KW
//                 | SyntaxKind::SUPER_KW
//                 | SyntaxKind::TRAIT_KW
//                 | SyntaxKind::TRUE_KW
//                 | SyntaxKind::TYPE_KW
//                 | SyntaxKind::UNSAFE_KW
//                 | SyntaxKind::USE_KW
//                 | SyntaxKind::WHERE_KW
//                 | SyntaxKind::WHILE_KW
//                 | SyntaxKind::YIELD_KW
//         )
//     }
// }
