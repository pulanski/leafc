use std::fmt;

use smol_str::SmolStr;

use super::Token;

impl Token {
    #[allow(clippy::wrong_self_convention)]
    fn to_string(&self) -> SmolStr {
        let str = match self {
            Self::ERROR => "Error",

            Self::IDENT => "Ident",

            Self::Comment => "Comment",
            Self::DocComment => "DocComment",
            Self::WHITESPACE => " ",

            Self::String => "str",
            Self::Rune => "rune",
            Self::INTEGER => "int",
            Self::Float => "float",
            Self::Bool => "bool",

            Self::Let => "let",
            Self::RETURN_KW => "return",
            Self::Continue => "continue",
            Self::Break => "break",
            Self::Package => "pkg",
            Self::Exposed => "exposed",
            Self::Empty => "empty",
            Self::Then => "then",
            Self::For => "for",
            Self::Or => "or",
            Self::And => "and",
            Self::Type => "type",
            Self::Loop => "loop",
            Self::While => "while",
            Self::If => "if",
            Self::Else => "else",
            Self::FN_KW => "fn",
            Self::Enum => "enum",
            Self::Trait => "trait",
            Self::Import => "import",
            Self::Exposing => "exposing",
            Self::Export => "export",
            Self::As => "as",
            Self::Library => "lib",
            Self::End => "end",
            Self::In => "in",
            Self::Is => "is",
            Self::Match => "match",
            Self::Where => "where",
            Self::Const => "comptime",
            Self::Extend => "extend",
            Self::With => "with",
            Self::Alias => "alias",
            Self::Mut => "mut",
            Self::Ref => "ref",
            Self::Extern => "extern",
            Self::Unsafe => "unsafe",

            Self::Equal => "=",
            Self::AddAssign => "+=",
            Self::SubAssign => "-=",
            Self::MultAssign => "*=",
            Self::DivAssign => "/=",
            Self::ModAssign => "%=",
            Self::PowAssign => "**=",
            Self::ShlAssign => "<<=",
            Self::ShrAssign => ">>=",
            Self::OrAssign => "|=",
            Self::AndAssign => "&=",
            Self::XorAssign => "^=",

            Self::IsEqual => "==",
            Self::IsNotEqual => "!=",
            Self::GreaterThanEqual => ">=",
            Self::LessThanEqual => "<=",
            Self::LeftCaret => "<",
            Self::RightCaret => ">",

            Self::Bang => "!",

            Self::Plus => "+",
            Self::Minus => "-",
            Self::Divide => "/",
            Self::Star => "*",
            Self::Modulo => "%",
            Self::DoubleStar => "**",
            Self::Pipe => "|",
            Self::Caret => "^",
            Self::Ampersand => "&",
            Self::Shl => "<<",
            Self::Shr => ">>",

            Self::LeftBrace => "[",
            Self::RightBrace => "]",
            Self::L_PAREN => "(",
            Self::R_PAREN => ")",
            Self::L_BRACE => "{",
            Self::R_BRACE => "}",
            Self::THIN_ARROW => "->",
            Self::LeftArrow => "<-",
            Self::RightRocket => "=>",

            Self::AtSign => "@",
            Self::Comma => ",",
            Self::SEMICOLON => ";",
            Self::Colon => ":",
            Self::Dot => ".",
            Self::DoubleDot => "..",
        };

        str.into()
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.to_string().as_str())
    }
}
