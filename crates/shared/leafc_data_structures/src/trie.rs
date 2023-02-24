use typed_builder::TypedBuilder;

use crate::collections::HashMap;

/// A **node** in a **trie**.
///
/// A trie is a tree-like data structure that stores a dynamic set or
/// associative array where the keys are usually strings. Unlike a binary
/// search tree, no node in the tree stores the key associated with that
/// node; instead, its position in the tree defines the key with which it
/// is associated. All the descendants of a node have a common prefix of
/// the string associated with that node, and the root is associated with
/// the empty string. Values are not necessarily associated with every
/// node. Rather, values tend only to be associated with leaves, and
/// with some inner nodes that correspond to keys of interest.
/// https://en.wikipedia.org/wiki/Trie
///
/// # Examples
///
/// ```
/// use leafc_data_structures::TrieNode;
///
/// // Create a new Trie Node.
/// let mut root = TrieNode::new('a');
///
/// // Trie Nodes can have children.
/// let child_b = root.add_child('b');
/// let child_c = root.add_child('c');
///
/// // Trie Nodes have an optional parent.
/// assert_eq!(child_b.parent(), Some(&root));
///
/// // From the root, we can get the children.
/// assert_eq!(root.get_child('b'), Some(&child_b));
/// assert_eq!(root.get_child('c'), Some(&child_c));
/// assert_eq!(root.get_child('d'), None);
///
/// // We can also get all the children.
/// // root.children() <- returns an iterator over the children.
///
/// // We can signify that a node is the end of a word.
/// root.set_end(true);
///
/// // And we can remove a child
/// root.remove_child('b');
///
/// assert!(root.get_child('b').is_none());
///
/// // Or all the children.
/// root.clear_children();
///
/// assert!(root.get_children().is_empty());
/// ```
#[derive(Debug, Clone, TypedBuilder)]
// #[builder(setter(into))]
pub struct TrieNode {
    /// The **children** of the **node**.
    children: HashMap<char, TrieNode>,
    /// The **parent** of the **node**.
    /// This is `None` if the **node** is the **root**.
    parent:   Option<Box<TrieNode>>,
    /// The **key** of the **node**.
    key:      Option<char>,
}

impl PartialEq for TrieNode {
    /// Two [`TrieNode`]s are equal if they have the same **key** and
    /// **parent**.
    /// The **children** are not considered.
    ///
    /// # Examples
    ///
    /// ```
    /// use leafc_data_structures::TrieNode;
    ///
    /// let node_a = TrieNode::new('a');
    /// let node_b = TrieNode::new('b');
    ///
    /// assert_eq!(node_a, node_b);
    /// ```
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key && self.parent == other.parent
    }
}

impl TrieNode {
    /// Create a new [`TrieNode`] with the given **key**, `ch`.
    /// The **key** is the **character** that the **node** represents.
    ///
    /// # Examples
    ///
    /// ```
    /// use leafc_data_structures::TrieNode;
    ///
    /// let node = TrieNode::new('a');
    ///
    /// assert_eq!(node.key(), Some('a'));
    /// ```
    pub fn new(ch: char) -> Self {
        Self { children: HashMap::default(), parent: None, key: Some(ch) }
    }

    /// Inserts a **child node** with the given **key**, `ch` into the
    /// [`TrieNode`].
    ///
    /// # Examples
    ///
    /// ```
    /// use leafc_data_structures::TrieNode;
    ///
    /// // Create a new Trie Node.
    /// let mut root = TrieNode::new('a');
    ///
    /// // Add a child node with the key 'b'.
    /// let child_b = root.add_child('b');
    ///
    /// // Add a child node with the key 'c'.
    /// let child_c = root.add_child('c');
    ///
    /// // From the root, we can get the children.
    /// assert!(root.get_child('b').is_some());
    /// assert!(root.get_child('c').is_some());
    /// assert!(root.get_child('d').is_none());
    /// ```
    pub fn add_child<'a>(&mut self, ch: char) -> &'a mut TrieNode {
        let node = TrieNode::new(ch);
        self.children.insert(ch, node.clone());

        Box::leak(Box::new(node))
    }

    /// Removes the **child node** with the given **key**, `ch` from the
    /// [`TrieNode`].
    ///
    /// # Examples
    ///
    /// ```
    /// use leafc_data_structures::TrieNode;
    ///
    /// // Create a new Trie Node.
    /// let mut root = TrieNode::new('a');
    ///
    /// // Add a child node with the key 'b'.
    /// let child_b = root.add_child('b');
    ///
    /// // Remove the child node with the key 'b'.
    /// root.remove_child('b');
    ///
    /// // From the root, we can get the children.
    /// assert!(root.get_child('b').is_none());
    /// ```
    pub fn remove_child(&mut self, ch: char) {
        self.children.remove(&ch);
    }

    /// Removes all the **child nodes** from the [`TrieNode`].
    ///
    /// # Examples
    ///
    /// ```
    /// use leafc_data_structures::TrieNode;
    ///
    /// // Create a new Trie Node.
    /// let mut root = TrieNode::new('a');
    ///
    /// // Create a few child nodes.
    /// let child_b = root.add_child('b');
    /// let child_c = root.add_child('c');
    /// let child_d = root.add_child('d');
    ///
    /// // Remove all the child nodes.
    /// root.clear_children();
    ///
    /// // From the root, we can get the children.
    /// assert!(
    ///     root.get_child('b').is_none() &&
    ///         root.get_child('c').is_none() &&
    ///         root.get_child('d').is_none()
    /// );
    /// ```
    pub fn clear_children(&mut self) {
        self.children.clear();
    }

    /// Returns a **mutable reference** to the **child node** with the given
    /// **key**, `ch`.
    ///
    /// # Examples
    ///
    /// ```
    /// use leafc_data_structures::TrieNode;
    ///
    /// // Create a new Trie Node.
    /// let mut root = TrieNode::new('a');
    ///
    /// // Add a child node with the key 'b'.
    /// let child_b = root.add_child('b');
    ///
    /// // Add a child node with the key 'c'.
    /// let child_c = root.add_child('c');
    ///
    /// // From the root, we can get the children.
    /// let mut child_b = root.get_child_mut('b').unwrap();
    ///
    /// // Change the key of the child node.
    /// child_b.set_key('d');
    ///
    /// // From the root, we can get the children.
    /// assert!(root.get_child('b').is_none());
    /// assert!(root.get_child('d').is_some());
    /// ```
    pub fn get_child_mut(&mut self, ch: char) -> Option<&mut TrieNode> {
        Some(Box::leak(Box::new(self.children.get_mut(&ch)?.clone())))
    }

    /// Returns a **mutable reference** to the **parent node** of the
    /// [`TrieNode`].
    ///
    /// # Examples
    ///
    /// ```
    /// use leafc_data_structures::TrieNode;
    ///
    /// // Create a new Trie Node.
    /// let mut root = TrieNode::new('a');
    ///
    /// // Add a child node with the key 'b'.
    /// let child_b = root.add_child('b');
    ///
    /// // B is the child of A.
    /// assert_eq!(child_b.parent()?.key(), Some('a'));
    /// ```
    pub fn parent(&self) -> Option<&TrieNode> {
        Some(Box::leak(Box::new(self.parent.clone()?)))
    }

    /// Returns a reference to the **child node** with the given **key**,
    /// `ch`.
    ///
    /// # Examples
    ///
    /// ```
    /// use leafc_data_structures::TrieNode;
    ///
    /// // Create a new Trie Node.
    /// let mut root = TrieNode::new('a');
    ///
    /// // Add a child node with the key 'b'.
    /// let child_b = root.add_child('b');
    ///
    /// // Add a child node with the key 'c'.
    /// let child_c = root.add_child('c');
    ///
    /// // From the root, we can get the children.
    /// assert!(root.get_child('b').is_some());
    /// assert!(root.get_child('c').is_some());
    /// ```
    pub fn get_child(&self, ch: char) -> Option<&TrieNode> {
        let node = self.children.get(&ch)?;
        Some(Box::leak(Box::new(node.clone())))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trie_node() {
        let mut root = TrieNode::new('a');

        let child_b = root.add_child('b');
        let child_c = root.add_child('c');

        assert_eq!(child_b.parent(), Some(&root));
        assert_eq!(child_c.parent(), Some(&root));

        assert!(root.get_child('b').is_some());
        assert!(root.get_child('c').is_some());

        assert!(root.get_child_mut('b').is_some());

        // assert_eq!(root.children(), vec![TrieNode::from('b'),
        // TrieNode::from('c')]);

        // root.set_end(true);

        // root.remove_child('b');

        // assert!(root.get_child('b').is_none());

        // root.remove_children();

        // assert!(root.children().is_empty());
    }
}
