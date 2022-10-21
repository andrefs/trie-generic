pub mod trie {
    use std::cell::Cell;
    use std::collections::BTreeMap;
    use std::string::String;

    pub struct TNode<T> {
        pub is_terminal: bool,
        pub content: Option<T>,
        pub children: BTreeMap<char, TNode<T>>,
    }

    impl<T> TNode<T> {
        pub fn new(content: Option<T>) -> TNode<T> {
            TNode {
                is_terminal: false,
                content,
                children: BTreeMap::new(),
            }
        }
    }

    pub struct Trie<T> {
        pub root: Cell<TNode<T>>,
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

    impl<T> Trie<T> {
        pub fn new(content: Option<T>) -> Trie<T> {
            Trie {
                root: Cell::new(TNode::new(content)),
            }
        }

        pub fn add(&mut self, s: &str, content: Option<T>) {
            add_fn(self.root.get_mut(), s, content)
        }

        pub fn pp(&mut self) -> String {
            pp_fn(self.root.get_mut(), 0)
        }

        pub fn find<'a>(&'a mut self, s: &'a str, must_be_terminal: bool) -> LongestPrefResult {
            let lpo = LongestPrefOpts {
                must_be_terminal,
                must_match_fully: true,
            };
            longest_prefix_fn(self.root.get_mut(), s, None, "".to_owned(), lpo)
        }

        pub fn longest_prefix<'a>(
            &'a mut self,
            s: &'a str,
            must_be_terminal: bool,
        ) -> LongestPrefResult {
            let lpo = LongestPrefOpts {
                must_be_terminal,
                must_match_fully: false,
            };
            longest_prefix_fn(self.root.get_mut(), s, None, "".to_owned(), lpo)
        }
    }

    fn longest_prefix_fn<'a, T>(
        cur_node: &'a TNode<T>,
        str_left: &'a str,
        last_terminal: Option<Vec<char>>,
        cur_pref: String,
        opts: LongestPrefOpts,
    ) -> LongestPrefResult {
        eprintln!("XXXXX '{cur_pref}' '{str_left}'");

        let mut new_last_terminal: Option<Vec<char>> = last_terminal;
        if cur_node.is_terminal {
            new_last_terminal = Some(cur_pref.chars().collect());
        }

        // end of searched string (matches fully)
        if str_left.is_empty() {
            eprintln!("XXXXX 1");
            if opts.must_be_terminal && !cur_node.is_terminal {
                return match new_last_terminal {
                    None => None,
                    Some(t) => Some((
                        t,
                        LongestPrefFlags {
                            is_terminal: true,
                            full_match: false,
                        },
                    )),
                };
            }
            eprintln!("XXXXX 3");
            return Some((
                cur_pref.chars().collect(),
                LongestPrefFlags {
                    is_terminal: cur_node.is_terminal,
                    full_match: true,
                },
            ));
        }
        eprintln!("XXXXX 3");

        let mut chars = str_left.chars();
        let ch = chars.next().unwrap();
        let new_str = chars.as_str();

        if cur_node.children.is_empty() || !cur_node.children.contains_key(&ch) {
            if opts.must_match_fully {
                return None;
            }
            if opts.must_be_terminal && !cur_node.is_terminal {
                return match new_last_terminal {
                    None => None,
                    Some(t) => Some((
                        t,
                        LongestPrefFlags {
                            is_terminal: true,
                            full_match: false,
                        },
                    )),
                };
            }
            return Some((
                cur_pref.chars().collect(),
                LongestPrefFlags {
                    is_terminal: cur_node.is_terminal,
                    full_match: false,
                },
            ));
        }
        return if !opts.must_be_terminal || cur_node.is_terminal {
            longest_prefix_fn(
                &cur_node.children[&ch],
                new_str,
                new_last_terminal,
                format!("{}{}", cur_pref, ch),
                opts,
            )
        } else {
            longest_prefix_fn(
                &cur_node.children[&ch],
                new_str,
                new_last_terminal,
                format!("{}{}", cur_pref, ch),
                opts,
            )
        };
    }

    fn pp_fn<T>(node: &TNode<T>, indent: u8) -> String {
        let mut res = String::from("");
        let iter = node.children.iter();

        if node.children.len() == 1 {
            let (k, v) = iter.last().unwrap();
            if node.is_terminal {
                res.push('\n');
                res.push_str(&" ".repeat(indent.into()));
                res.push_str(&k.to_string());
            } else {
                res.push_str(&k.to_string())
            }
            res.push_str(&pp_fn(v, indent + 1));
        } else {
            for (k, v) in iter {
                res.push('\n');
                res.push_str(&" ".repeat(indent.into()));
                res.push_str(&k.to_string());
                res.push_str(&pp_fn(v, indent + 1));
            }
        }
        res
    }

    fn add_fn<T>(node: &mut TNode<T>, s: &str, content: Option<T>) {
        if s.is_empty() {
            node.is_terminal = true;
            node.content = content;
            return;
        }
        let mut chars = s.chars();
        let c = chars.next().unwrap();
        let rest = chars.as_str();

        node.children.entry(c).or_insert_with(|| TNode::new(None));
        let subtrie = node.children.get_mut(&c).expect("char must exist");
        add_fn(subtrie, rest, content);
    }
}

#[cfg(test)]
mod tests {
    use std::cell::Cell;
    use std::collections::BTreeMap;

    use super::*;
    use trie::*;

    #[test]
    fn create_tnode() {
        let n = TNode::new(Some(1));
        assert_eq!(n.content, Some(1));
        assert!(!n.is_terminal);
        assert!(n.children.is_empty());
    }

    #[test]
    fn pretty_print() {
        let mut t: Trie<u8> = Trie {
            root: Cell::new(TNode {
                is_terminal: false,
                content: None,
                children: BTreeMap::from([
                    (
                        'a',
                        TNode {
                            is_terminal: true,
                            content: None,
                            children: BTreeMap::from([(
                                'b',
                                TNode {
                                    is_terminal: true,
                                    content: None,
                                    children: BTreeMap::new(),
                                },
                            )]),
                        },
                    ),
                    (
                        'c',
                        TNode {
                            is_terminal: true,
                            content: None,
                            children: BTreeMap::new(),
                        },
                    ),
                    (
                        'd',
                        TNode {
                            is_terminal: true,
                            content: None,
                            children: BTreeMap::new(),
                        },
                    ),
                ]),
            }),
        };
        assert_eq!(t.pp(), "\na\n b\nc\nd")
    }

    #[test]
    fn add_empty_string() {
        let mut t = Trie::new(None);
        t.add("", Some(1));
        assert_eq!(t.root.get_mut().content, Some(1));
    }

    #[test]
    fn add_single_char_string() {
        let mut t = Trie::new(None);
        t.add("a", Some(1));
        t.add("ab", Some(1));
        t.add("c", Some(1));
        t.add("d", Some(1));
        println!("{}", t.pp());
        assert_eq!(t.pp(), "\na\n b\nc\nd")
    }

    #[test]
    fn longest_prefix() {
        let mut t = Trie::new(None);
        t.add("this is words", Some(1));
        t.add("this is more", Some(1));
        t.add("this is more words", Some(1));
        let pref = t.longest_prefix("this is more wo", false).unwrap().0;
        let expected: Vec<char> = "this is more wo".chars().collect();
        assert_eq!(pref, expected);
    }
    #[test]
    fn longest_prefix_terminal() {
        let mut t = Trie::new(None);
        t.add("this is words", Some(1));
        t.add("this is more", Some(1));
        t.add("this is more words", Some(1));
        eprintln!("{}", t.pp());
        let pref = t.longest_prefix("this is more wo", true).unwrap().0;
        let expected: Vec<char> = "this is more".chars().collect();
        assert_eq!(pref, expected);
    }
    #[test]
    fn longest_prefix_fail() {
        let mut t = Trie::new(None);
        t.add("this is words", Some(1));
        t.add("this is more", Some(1));
        t.add("this is more words", Some(1));
        eprintln!("{}", t.pp());
        let pref = t.longest_prefix("this is", true);
        assert!(pref.is_none());
    }
    #[test]
    fn find() {
        let mut t = Trie::new(None);
        t.add("this is words", Some(1));
        t.add("this is more", Some(1));
        t.add("this is even more", Some(1));
        let pref = t.find("this is more", false).unwrap().0;
        let expected: Vec<char> = "this is more".chars().collect();
        assert_eq!(pref, expected);
    }
    #[test]
    fn find_terminal() {
        let mut t = Trie::new(None);
        t.add("this is words", Some(1));
        t.add("this is more", Some(1));
        t.add("this is even more", Some(1));
        let pref = t.find("this is more", true).unwrap().0;
        let expected: Vec<char> = "this is more".chars().collect();
        assert_eq!(pref, expected);
    }
    #[test]
    fn find_terminal_fail() {
        let mut t = Trie::new(None);
        t.add("this is words", Some(1));
        t.add("this is more", Some(1));
        t.add("this is even more", Some(1));
        let pref = t.find("this is more wo", true);
        assert!(pref.is_none())
    }
}
