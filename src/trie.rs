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

struct FindResults<'a, T: Display + Debug> {
    node: Option<&'a TNode<'a, T>>,
    prefix: String,
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
    fn to_leaf(&mut self) {
        *self = match self {
            TNode::Empty => TNode::Leaf(Leaf {
                content: &None,
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

    fn is_childless(&self) -> bool {
        match self {
            TNode::Empty => true,
            TNode::Leaf(_) => true,
            TNode::Node(node) => node.children.is_empty(),
        }
    }

    fn is_empty(&self) -> bool {
        match self {
            TNode::Empty => true,
            TNode::Leaf(leaf) => leaf.content.is_none(),
            TNode::Node(node) => node.content.is_none() && node.children.is_empty(),
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

    pub fn contains_key(&self, s: &str) -> bool {
        self.find(s, true).is_some()
    }

    pub fn find(&self, s: &str, must_be_terminal: bool) -> Option<&TNode<T>> {
        let lpo = LongestPrefOpts {
            must_be_terminal,
            must_match_fully: true,
        };
        let last_term = FindResults {
            node: None,
            prefix: "".to_owned(),
        };
        self.longest_prefix_fn(s, "", last_term, lpo).node
    }

    pub fn longest_prefix(&'a mut self, s: &'a str, must_be_terminal: bool) -> String {
        let lpo = LongestPrefOpts {
            must_be_terminal,
            must_match_fully: false,
        };
        let last_term = FindResults {
            node: None,
            prefix: "".to_owned(),
        };
        self.longest_prefix_fn(s, "", last_term, lpo).prefix
    }

    fn longest_prefix_fn(
        &self,
        str_left: &str,
        str_acc: &str,
        last_terminal: FindResults<'a, T>,
        opts: LongestPrefOpts,
    ) -> FindResults<T> {
        match self {
            TNode::Empty => FindResults {
                node: None,
                prefix: "".to_owned(),
            },
            TNode::Leaf(leaf) => {
                let new_last_terminal = if leaf.is_terminal {
                    FindResults {
                        node: Some(self),
                        prefix: str_acc.to_owned(),
                    }
                } else {
                    last_terminal
                };
                if str_left.is_empty() {
                    return if opts.must_be_terminal {
                        new_last_terminal
                    } else {
                        FindResults {
                            node: Some(self),
                            prefix: str_acc.to_owned(),
                        }
                    };
                } else {
                    FindResults {
                        node: None,
                        prefix: "".to_owned(),
                    }
                }
            }
            TNode::Node(node) => {
                let new_last_terminal = if node.is_terminal {
                    FindResults {
                        node: Some(self),
                        prefix: str_acc.to_owned(),
                    }
                } else {
                    last_terminal
                };
                if str_left.is_empty() {
                    return if opts.must_be_terminal {
                        new_last_terminal
                    } else {
                        FindResults {
                            node: Some(self),
                            prefix: str_acc.to_owned(),
                        }
                    };
                };

                let first_char = str_left.chars().next().unwrap();
                let rest = &str_left[first_char.len_utf8()..];
                if !node.children.contains_key(&first_char) {
                    if opts.must_match_fully {
                        return FindResults {
                            node: None,
                            prefix: "".to_owned(),
                        };
                    } else {
                        return FindResults {
                            node: Some(self),
                            prefix: str_acc.to_owned(),
                        };
                    }
                }
                let next_node = node.children.get(&first_char).unwrap();
                let mut new_str_acc = str_acc.to_owned();
                new_str_acc.push(first_char);
                return next_node.longest_prefix_fn(
                    rest,
                    new_str_acc.as_str(),
                    new_last_terminal,
                    opts,
                );
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

                for (k, v) in iter {
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

    fn remove(&mut self, str_left: &'a str, remove_subtree: bool) -> bool {
        self.remove_fn(str_left, remove_subtree).1
    }

    fn remove_fn(&mut self, str_left: &'a str, remove_subtree: bool) -> (bool, bool) {
        let first_char = str_left.chars().next().unwrap();
        let rest = &str_left[first_char.len_utf8()..];

        match self {
            TNode::Empty | TNode::Leaf(_) => {
                return (false, false);
            }
            TNode::Node(node) => {
                if !node.children.contains_key(&first_char) {
                    return (false, false);
                }

                if rest.is_empty() {
                    match node.children.get_mut(&first_char).unwrap() {
                        TNode::Leaf(_) => {
                            let removed = node.children.remove(&first_char).is_some();
                            let bubble_up = removed && !node.is_terminal;
                            return (bubble_up, removed);
                        }
                        TNode::Empty => {
                            panic!("Something wrong")
                        }
                        TNode::Node(sub_node) => {
                            if remove_subtree {
                                let removed = node.children.remove(&first_char).is_some();
                                let bubble_up = removed && !node.is_terminal;
                                return (bubble_up, removed);
                            }
                            if !sub_node.is_terminal {
                                return (false, false);
                            }
                            sub_node.is_terminal = false;
                            return (true, true);
                        }
                    }
                } else {
                    let (bubble_up, removed) = node
                        .children
                        .get_mut(&first_char)
                        .unwrap()
                        .remove_fn(rest, remove_subtree);
                    let child = node.children.get_mut(&first_char).unwrap();
                    if removed && child.is_childless() {
                        child.to_leaf();
                    }
                    if bubble_up {
                        let removed = node.children.remove(&first_char).is_some();
                        let bubble_up = removed && !node.is_terminal;
                        return (bubble_up, removed);
                    }
                    return (false, removed);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use super::*;

    #[test]
    fn pretty_print() {
        let t: TNode<u8> = TNode::Node(Node {
            is_terminal: false,
            content: &None,
            children: BTreeMap::from([
                (
                    'a',
                    TNode::Node(Node {
                        is_terminal: true,
                        content: &None,
                        children: BTreeMap::from([(
                            'b',
                            TNode::Node(Node {
                                is_terminal: false,
                                content: &None,
                                children: BTreeMap::from([(
                                    'c',
                                    TNode::Leaf(Leaf {
                                        is_terminal: true,
                                        content: &None,
                                    }),
                                )]),
                            }),
                        )]),
                    }),
                ),
                (
                    'd',
                    TNode::Leaf(Leaf {
                        is_terminal: true,
                        content: &None,
                    }),
                ),
                (
                    'e',
                    TNode::Leaf(Leaf {
                        is_terminal: true,
                        content: &None,
                    }),
                ),
            ]),
        });
        assert_eq!(t.pp(false), "a\n bc\nd\ne\n")
    }

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

    #[test]
    fn add_single_char_string() {
        let mut t = TNode::Empty;
        t.add("a", &Some(1)).unwrap();
        t.add("ab", &Some(1)).unwrap();
        t.add("c", &Some(1)).unwrap();
        t.add("d", &Some(1)).unwrap();
        assert_eq!(t.pp(false), "a\n b\nc\nd\n")
    }

    #[test]
    fn contains_key() {
        let mut t = TNode::Empty;
        t.add("a", &Some(1)).unwrap();
        assert!(t.contains_key("a"));

        t.add("abc", &Some(2)).unwrap();
        assert!(!t.contains_key("b"));
        assert!(t.contains_key("abc"));
    }

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

    #[test]
    fn longest_prefix() {
        let mut t = TNode::Empty;
        t.add("this is words", &Some(1)).unwrap();
        t.add("this is more", &Some(1)).unwrap();
        t.add("this is more words", &Some(1)).unwrap();
        let res = t.longest_prefix("this is more wo", false);
        let expected: Vec<char> = "this is more wo".chars().collect();
        assert_eq!(res.chars().collect::<Vec<_>>(), expected);
    }

    #[test]
    fn longest_prefix_no_full_match() {
        let mut t = TNode::Empty;
        t.add("this is words", &Some(1)).unwrap();
        t.add("this is more", &Some(1)).unwrap();
        t.add("this is more words", &Some(1)).unwrap();
        let res = t.longest_prefix("this is weeks", false);
        let expected: Vec<char> = "this is w".chars().collect();
        assert_eq!(res.chars().collect::<Vec<_>>(), expected);
    }

    #[test]
    fn longest_prefix_terminal() {
        let mut t = TNode::Empty;
        t.add("this is words", &Some(1)).unwrap();
        t.add("this is more", &Some(1)).unwrap();
        t.add("this is more words", &Some(1)).unwrap();
        let res = t.longest_prefix("this is more wo", true);
        let expected: Vec<char> = "this is more".chars().collect();
        assert_eq!(res.chars().collect::<Vec<_>>(), expected);
    }

    #[test]
    fn longest_prefix_fail() {
        let mut t = TNode::Empty;
        t.add("this is words", &Some(1)).unwrap();
        t.add("this is more", &Some(1)).unwrap();
        t.add("this is more words", &Some(1)).unwrap();
        let res = t.longest_prefix("this is", true);
        assert!(res.is_empty());
    }

    #[test]
    fn find() {
        let mut t = TNode::Empty;
        t.add("this is words", &Some(1)).unwrap();
        t.add("this is more", &Some(2)).unwrap();
        t.add("this is even more", &Some(3)).unwrap();
        let res = t.find("this is more", false).unwrap();
        //let expected: Vec<char> = "this is more".chars().collect();
        assert_eq!(res.content().unwrap(), 2)
    }
    #[test]
    fn find_terminal() {
        let mut t = TNode::Empty;
        t.add("this is words", &Some(1)).unwrap();
        t.add("this is more", &Some(2)).unwrap();
        t.add("this is even more", &Some(3)).unwrap();
        let res = t.find("this is more", true).unwrap();
        //let expected: Vec<char> = "this is more".chars().collect();
        assert_eq!(res.content().unwrap(), 2);
    }
    #[test]
    fn find_terminal_fail() {
        let mut t = TNode::Empty;
        t.add("this is words", &Some(1)).unwrap();
        t.add("this is more", &Some(1)).unwrap();
        t.add("this is even more", &Some(1)).unwrap();
        let pref = t.find("this is more wo", true);
        assert!(pref.is_none())
    }

    #[test]
    fn remove() {
        let mut t = TNode::Empty;
        t.add("a", &Some(1)).unwrap();
        t.add("abc", &Some(2)).unwrap();
        t.add("abcd", &Some(3)).unwrap();

        assert!(!t.remove("ab", false));
        assert!(t.contains_key("a"));
        assert!(t.contains_key("abc"));
        assert!(t.contains_key("abcd"));

        assert!(t.remove("abc", true));
        assert!(t.contains_key("a"));
        assert!(!t.contains_key("abc"));
        assert!(!t.contains_key("abcd"));

        assert!(t.remove("a", false));
        assert!(t.is_empty());
    }

    #[test]
    fn remove_non_terminal() {
        let mut t = TNode::Empty;
        t.add("a", &Some(1)).unwrap();
        t.add("abc", &Some(2)).unwrap();
        t.remove("abc", false);
        println!("{}", t.pp(true));
        let expected = "a\n";
        assert_eq!(t.pp(false), expected);
    }
    #[test]
    fn remove_subtree() {
        let mut t = TNode::Empty;
        t.add("a", &Some(1)).unwrap();
        t.add("abc", &Some(2)).unwrap();
        t.remove("ab", true);
        println!("{}", t.pp(true));
        let expected = "a\n";
        assert_eq!(t.pp(false), expected);
    }
    #[test]
    fn remove_non_existing() {
        let mut t = TNode::Empty;
        t.add("a", &Some(1)).unwrap();
        t.add("abc", &Some(2)).unwrap();
        let expected = t.pp(false);
        t.remove("xyz", true);
        println!("{}", t.pp(true));
        assert_eq!(t.pp(false), expected);
    }
}
