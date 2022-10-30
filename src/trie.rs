use std::collections::BTreeMap;
use std::fmt::{self, Debug, Display};

#[derive(Debug)]
pub struct Leaf<'a, T> {
    content: &'a Option<T>,
    is_terminal: bool,
}

#[derive(Debug)]
pub struct Node<'a, T: Debug + Display> {
    content: &'a Option<T>,
    children: BTreeMap<char, TNode<'a, T>>,
    is_terminal: bool,
}

#[derive(Debug)]
pub enum TNode<'a, T: Display + Debug> {
    Empty,
    Leaf(Leaf<'a, T>),
    Node(Node<'a, T>),
}

pub struct LongestPrefFlags {
    is_terminal: bool,
    full_match: bool,
}

struct LongestPrefOpts {
    must_be_terminal: bool,
    must_match_fully: bool,
}

type LongestPrefResult = Option<(Vec<char>, LongestPrefFlags)>;

#[derive(Debug, Clone)]
pub struct KeyExists;

impl fmt::Display for KeyExists {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Cannot add same key twice")
    }
}

impl<'a, T: Display + Debug> fmt::Display for TNode<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            TNode::Empty => {
                write!(f, "(empty)")
            }
            TNode::Leaf(leaf) => {
                if let Some(c) = leaf.content {
                    return write!(f, "({})", c);
                }
                Ok(())
            }
            TNode::Node(node) => {
                if let Some(c) = node.content {
                    return write!(f, "({})", c);
                }
                Ok(())
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct KeyNotFound;

impl fmt::Display for KeyNotFound {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Key not found")
    }
}

impl<'a, T: Display + Debug> TNode<'a, T> {
    fn to_leaf(&mut self, cont: &'a Option<T>) {
        *self = match self {
            TNode::Empty => TNode::Leaf(Leaf {
                content: cont,
                is_terminal: false,
            }),
            TNode::Node(node) => TNode::Leaf(Leaf {
                content: node.content,
                is_terminal: node.is_terminal,
            }),
            _ => panic!("Could not convert to Leaf"),
        }
    }
    fn to_empty(&mut self) {
        *self = TNode::Empty;
    }
    fn to_node(&mut self) {
        *self = match self {
            TNode::Leaf(leaf) => TNode::Node(Node {
                content: leaf.content,
                children: BTreeMap::from([]),
                is_terminal: leaf.is_terminal,
            }),
            TNode::Empty => TNode::Node(Node {
                content: &None,
                children: BTreeMap::from([]),
                is_terminal: false,
            }),
            _ => panic!("Could not convert to Node"),
        }
    }

    fn is_terminal(&self) -> bool {
        match self {
            TNode::Empty => false,
            TNode::Leaf(leaf) => leaf.is_terminal,
            TNode::Node(node) => node.is_terminal,
        }
    }

    fn content(&self) -> &Option<T> {
        match self {
            TNode::Leaf(leaf) => leaf.content,
            TNode::Node(node) => node.content,
            TNode::Empty => panic!("Cannot call .content() for Empty"),
        }
    }

    pub fn add(&mut self, s: &str, cont: &'a Option<T>) -> Result<&TNode<T>, KeyExists> {
        if s.is_empty() {
            if self.is_terminal() {
                return Err(KeyExists);
            } else {
                match self {
                    TNode::Node(node) => {
                        node.content = cont;
                        node.is_terminal = true;
                        return Ok(self);
                    }
                    TNode::Leaf(_) => {
                        *self = TNode::Leaf(Leaf {
                            content: cont,
                            is_terminal: true,
                        });
                        return Ok(self);
                    }
                    TNode::Empty => {
                        *self = TNode::Leaf(Leaf {
                            content: cont,
                            is_terminal: true,
                        });
                        return Ok(self);
                    }
                };
            };
        }
        let first_char = s.chars().next().unwrap();
        let rest = &s[first_char.len_utf8()..];

        match self {
            TNode::Empty | TNode::Leaf { .. } => {
                self.to_node();
                self.add(s, cont)
            }
            TNode::Node(node) => {
                if node.children.contains_key(&first_char) {
                    node.children.get_mut(&first_char).unwrap().add(rest, cont)
                } else {
                    let new_node = TNode::Empty;

                    node.children
                        .entry(first_char)
                        .or_insert(new_node)
                        .add(rest, cont)
                }
            }
        }
    }

    pub fn find(&mut self, s: &str, must_be_terminal: bool) -> Option<&TNode<T>> {
        let lpo = LongestPrefOpts {
            must_be_terminal,
            must_match_fully: true,
        };
        self.longest_prefix_fn(s, None, "".to_owned(), lpo)
    }

    pub fn longest_prefix(&'a mut self, s: &'a str, must_be_terminal: bool) -> Option<&TNode<T>> {
        let lpo = LongestPrefOpts {
            must_be_terminal,
            must_match_fully: false,
        };
        self.longest_prefix_fn(s, None, "".to_owned(), lpo)
    }

    fn longest_prefix_fn(
        &self,
        str_left: &str,
        last_terminal: Option<&'a TNode<T>>,
        cur_pref: String,
        opts: LongestPrefOpts,
    ) -> Option<&TNode<T>> {
        match self {
            TNode::Empty => None,
            TNode::Leaf(leaf) => {
                let new_last_terminal = if leaf.is_terminal {
                    Some(self)
                } else {
                    last_terminal
                };
                if str_left.is_empty() {
                    return if opts.must_be_terminal && !leaf.is_terminal {
                        new_last_terminal
                    } else {
                        Some(self)
                    };
                } else {
                    None
                }
            }
            TNode::Node(node) => {
                let new_last_terminal = if node.is_terminal {
                    Some(self)
                } else {
                    last_terminal
                };
                if str_left.is_empty() {
                    return if opts.must_be_terminal && !node.is_terminal {
                        last_terminal
                    } else {
                        Some(self)
                    };
                };

                let first_char = str_left.chars().next().unwrap();
                let rest = &str_left[first_char.len_utf8()..];
                if !node.children.contains_key(&first_char) {
                    return None;
                }
                let next_node = node.children.get(&first_char).unwrap();
                return next_node.longest_prefix_fn(rest, new_last_terminal, cur_pref, opts);
            }
        }
    }

    pub fn pp(&self, print_content: bool) -> String {
        return self.pp_fn(0, print_content);
    }

    fn pp_fn(&self, indent: u8, print_content: bool) -> String {
        let mut res = String::from("");
        match &self {
            TNode::Empty => {
                res.push_str("[empty]\n");
                res
            }
            TNode::Leaf { .. } => {
                if print_content {
                    res.push_str(format!("  {}", self).as_str());
                }
                res.push('\n');
                res
            }
            TNode::Node(node) => {
                let iter = node.children.iter();

                let child_count = node.children.len();

                for (i, (k, v)) in iter.enumerate() {
                    if node.is_terminal || child_count > 1 {
                        if indent != 0 {
                            res.push('\n');
                        }
                        res.push_str(&" ".repeat(indent.into()));
                    }

                    res.push_str(&k.to_string());
                    res.push_str(v.pp_fn(indent + 1, print_content).as_str());
                }
                res
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use super::*;

    //    #[test]
    //    fn pretty_print() {
    //        let mut t: Trie<u8> = Trie {
    //            root: RefCell::new(TNode {
    //                is_terminal: false,
    //                content: None,
    //                children: BTreeMap::from([
    //                    (
    //                        'a',
    //                        Box::from(TNode {
    //                            is_terminal: true,
    //                            content: None,
    //                            children: BTreeMap::from([(
    //                                'b',
    //                                Box::from(TNode {
    //                                    is_terminal: false,
    //                                    content: None,
    //                                    children: BTreeMap::from([(
    //                                        'c',
    //                                        Box::from(TNode {
    //                                            is_terminal: true,
    //                                            content: None,
    //                                            children: BTreeMap::new(),
    //                                        }),
    //                                    )]),
    //                                }),
    //                            )]),
    //                        }),
    //                    ),
    //                    (
    //                        'd',
    //                        Box::from(TNode {
    //                            is_terminal: true,
    //                            content: None,
    //                            children: BTreeMap::new(),
    //                        }),
    //                    ),
    //                    (
    //                        'e',
    //                        Box::from(TNode {
    //                            is_terminal: true,
    //                            content: None,
    //                            children: BTreeMap::new(),
    //                        }),
    //                    ),
    //                ]),
    //            }),
    //        };
    //        assert_eq!(t.pp(false), "\na\n bc\nd\ne")
    //    }

    #[test]
    fn add_to_empty_trie() {
        let mut t = TNode::Empty;
        t.add("a", &Some(1)).unwrap();
        match t {
            TNode::Node(node) => {
                assert_eq!(node.content, &None);
                assert_eq!(node.is_terminal, false);
                let subt = node.children.get(&'a').unwrap();
                assert_eq!(subt.content(), &Some(1));
                assert_eq!(subt.is_terminal(), true);
            }
            _ => panic!("t should be TNode::Node"),
        }
    }

    //#[test]
    //fn add_single_char_string() {
    //    let mut t = TNode::Empty;
    //    t.add("a", &Some(1)).unwrap();
    //    t.add("ab", &Some(1)).unwrap();
    //    t.add("c", &Some(1)).unwrap();
    //    t.add("d", &Some(1)).unwrap();
    //    assert_eq!(t.pp(false), "\na\n b\nc\nd")
    //}

    #[test]
    fn show_content() {
        let mut t = TNode::Empty;
        assert_eq!(t.pp(true), "[empty]\n");

        t.add("a", &Some(1)).unwrap();
        assert_eq!(t.pp(true), "a  (1)\n");

        t.add("abc", &Some(2)).unwrap();
        assert_eq!(t.pp(true), "a\n bc  (2)\n");

        t.add("d", &Some(3)).unwrap();
        assert_eq!(t.pp(true), "a\n bc  (2)\nd  (3)\n");

        t.add("e", &Some(4)).unwrap();
        assert_eq!(t.pp(true), "a\n bc  (2)\nd  (3)\ne  (4)\n");
    }

    //    #[test]
    //    fn longest_prefix() {
    //        let mut t = Trie::new(None);
    //        t.add("this is words", Some(1));
    //        t.add("this is more", Some(1));
    //        t.add("this is more words", Some(1));
    //        let pref = t.longest_prefix("this is more wo", false).unwrap().0;
    //        let expected: Vec<char> = "this is more wo".chars().collect();
    //        assert_eq!(pref, expected);
    //    }
    //    #[test]
    //    fn longest_prefix_terminal() {
    //        let mut t = Trie::new(None);
    //        t.add("this is words", Some(1));
    //        t.add("this is more", Some(1));
    //        t.add("this is more words", Some(1));
    //        let pref = t.longest_prefix("this is more wo", true).unwrap().0;
    //        let expected: Vec<char> = "this is more".chars().collect();
    //        assert_eq!(pref, expected);
    //    }
    //    #[test]
    //    fn longest_prefix_fail() {
    //        let mut t = Trie::new(None);
    //        t.add("this is words", Some(1));
    //        t.add("this is more", Some(1));
    //        t.add("this is more words", Some(1));
    //        let pref = t.longest_prefix("this is", true);
    //        assert!(pref.is_none());
    //    }
    //    #[test]
    //    fn find() {
    //        let mut t = Trie::new(None);
    //        t.add("this is words", Some(1));
    //        t.add("this is more", Some(1));
    //        t.add("this is even more", Some(1));
    //        let pref = t.find("this is more", false).unwrap().0;
    //        let expected: Vec<char> = "this is more".chars().collect();
    //        assert_eq!(pref, expected);
    //    }
    //    #[test]
    //    fn find_terminal() {
    //        let mut t = Trie::new(None);
    //        t.add("this is words", Some(1));
    //        t.add("this is more", Some(1));
    //        t.add("this is even more", Some(1));
    //        let pref = t.find("this is more", true).unwrap().0;
    //        let expected: Vec<char> = "this is more".chars().collect();
    //        assert_eq!(pref, expected);
    //    }
    //    #[test]
    //    fn find_terminal_fail() {
    //        let mut t = Trie::new(None);
    //        t.add("this is words", Some(1));
    //        t.add("this is more", Some(1));
    //        t.add("this is even more", Some(1));
    //        let pref = t.find("this is more wo", true);
    //        assert!(pref.is_none())
    //    }
    //    #[test]
    //    fn remove() {
    //        let mut t = Trie::new(None);
    //        t.add("ab", Some(1));
    //        t.add("abc", Some(2));
    //        t.remove("abc", false);
    //        println!("{}", t.pp(true));
    //        let expected = "ab";
    //        assert_eq!(t.pp(false), expected);
    //    }
    //    #[test]
    //    fn remove_non_terminal() {
    //        let mut t = Trie::new(None);
    //        t.add("a", Some(1));
    //        t.add("abc", Some(2));
    //        t.remove("abc", false);
    //        println!("{}", t.pp(true));
    //        let expected = "a";
    //        assert_eq!(t.pp(false), expected);
    //    }
    //    #[test]
    //    fn remove_subtree() {
    //        let mut t = Trie::new(None);
    //        t.add("a", Some(1));
    //        t.add("abc", Some(2));
    //        t.remove("ab", true);
    //        println!("{}", t.pp(true));
    //        let expected = "a";
    //        assert_eq!(t.pp(false), expected);
    //    }
    //    #[test]
    //    fn remove_non_existing() {
    //        let mut t = Trie::new(None);
    //        t.add("a", Some(1));
    //        t.add("abc", Some(2));
    //        let expected = t.pp(false);
    //        t.remove("xyz", true);
    //        println!("{}", t.pp(true));
    //        assert_eq!(t.pp(false), expected);
    //    }
}
