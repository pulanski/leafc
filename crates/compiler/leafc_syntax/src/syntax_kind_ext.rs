use std::mem;

use strum::EnumCount;

use crate::SyntaxKind;

impl From<u16> for SyntaxKind {
    #[inline(always)]
    fn from(kind: u16) -> SyntaxKind {
        debug_assert!(kind <= SyntaxKind::COUNT as u16);
        unsafe { mem::transmute(kind) }
    }
}

impl From<SyntaxKind> for u16 {
    #[inline(always)]
    fn from(kind: SyntaxKind) -> u16 {
        kind as u16
    }
}
