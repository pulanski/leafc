use allocative::Allocative;
use derive_more::{
    Add,
    AddAssign,
    Display,
    Mul,
    MulAssign,
    Sub,
    SubAssign,
};
use derive_new::new;
use dupe::Dupe;
use std::ops::{
    Add,
    AddAssign,
};

/// A small, `Copy`, value representing a **position** into a piece of **source
/// text**.
///
/// In the context of a file, this corresponds to the **number of bytes** from
/// the **start of the file**.
#[derive(
    Copy,
    Clone,
    Dupe,
    Hash,
    Eq,
    PartialEq,
    PartialOrd,
    Ord,
    Debug,
    Default,
    Allocative,
    new,
    Add,
    Sub,
    Mul,
    AddAssign,
    SubAssign,
    MulAssign,
    Display,
)]
#[allow(clippy::module_name_repetitions)]
#[display(fmt = "{_0}")]
pub struct TextPosition(usize);

impl Add<usize> for TextPosition {
    type Output = Self;

    fn add(self, rhs: usize) -> Self::Output {
        Self(self.0 + rhs)
    }
}

impl AddAssign<usize> for TextPosition {
    fn add_assign(&mut self, rhs: usize) {
        self.0 += rhs;
    }
}

impl From<usize> for TextPosition {
    fn from(value: usize) -> Self {
        Self(value)
    }
}

impl From<TextPosition> for usize {
    fn from(val: TextPosition) -> Self {
        val.0
    }
}

#[test]
fn test_text_position() {
    let a = TextPosition::new(0);
    let b = TextPosition::new(1);

    assert!(a < b);
    assert!(a <= b);
    assert!(a <= a);
    assert!(b > a);
    assert!(b >= a);
    assert!(b >= b);

    let mut c = a + b;

    assert_eq!(c, TextPosition::from(1));
    assert_eq!("0 + 1 = 1", format!("{a} + {b} = {c}"));

    c += 1;
    assert_eq!(c, TextPosition::from(2));
}
