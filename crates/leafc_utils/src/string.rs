use smartstring::alias::String;

/// Removes all occurrences of a substring from a string.
///
/// # Examples
///
/// ```
/// use leafc_utils::string::remove_substr;
/// use smartstring::alias::String;
///
/// let mut s = String::from("hello world");
/// remove_substr(&mut s, "world");
/// assert_eq!(s, "hello ");
/// ```
pub fn remove_substr(from: &mut String, substring: &str) {
    let mut start = 0;
    while let Some(pos) = from[start..].find(substring) {
        from.replace_range(start + pos..start + pos + substring.len(), "");
        start += pos;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_remove_substring() {
        let mut from = String::from("hello world");
        remove_substr(&mut from, "world");
        assert_eq!(from, "hello ");

        // remove all occurrences
        let mut from = String::from("hello world hello world");
        remove_substr(&mut from, "hello");
        assert_eq!(from, " world  world");
    }
}
