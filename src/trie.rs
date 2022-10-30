use std::collections::BTreeMap;
use std::fmt::{self, Debug, Display};

pub enum TNode<'a, T> {
    Empty,
    Leaf {
        content: &'a Option<T>,
        is_terminal: bool,
    },
    Node {
        content: &'a Option<T>,
        children: BTreeMap<char, TNode<'a, T>>,
        is_terminal: bool,
    },
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
            TNode::Leaf {
                content,
                is_terminal,
            } => {
                if let Some(c) = content {
                    return write!(f, "({})", c);
                }
                Ok(())
            }
            TNode::Node {
                content,
                children,
                is_terminal,
            } => {
                if let Some(c) = content {
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

impl<'a, T> TNode<'a, T> {
    fn to_leaf(&mut self, cont: &'a Option<T>) {
        *self = match self {
            TNode::Empty => TNode::Leaf {
                content: cont,
                is_terminal: true,
            },
            TNode::Node {
                content: _,
                children: _,
                is_terminal: _,
            } => TNode::Leaf {
                content: cont,
                is_terminal: true,
            },
            _ => panic!("Could not convert to Leaf"),
        }
    }
    fn to_empty(&mut self) {
        *self = TNode::Empty;
    }
    fn to_node(&mut self, c: char, cont: &'a Option<T>, is_term: bool) {
        *self = match self {
            TNode::Empty => TNode::Node {
                content: cont,
                children: BTreeMap::new(),
                is_terminal: is_term,
            },
            TNode::Leaf {
                content,
                is_terminal,
            } => TNode::Node {
                content,
                children: BTreeMap::from([(
                    c,
                    TNode::Leaf {
                        content: cont,
                        is_terminal: is_term,
                    },
                )]),
                is_terminal: *is_terminal,
            },
            _ => panic!("Could not convert to Node"),
        }
    }

    fn is_terminal(&self) -> bool {
        match self {
            TNode::Empty => false,
            TNode::Leaf {
                content: _,
                is_terminal,
            } => *is_terminal,
            TNode::Node { is_terminal, .. } => *is_terminal,
        }
    }

    pub fn add(&mut self, s: &str, content: &'a Option<T>) -> Result<&TNode<T>, KeyExists> {
        if s.is_empty() && self.is_terminal() {
            return Err(KeyExists);
        }
        let first_char = s.chars().next().unwrap();
        let rest = &s[first_char.len_utf8()..];

        match self {
            TNode::Empty => {
                self.to_leaf(content);
                return Ok(self);
            }
            TNode::Leaf { .. } => {
                self.to_node(first_char, content, true);
                return Ok(self);
            }
            TNode::Node {
                content, children, ..
            } => {
                if children.contains_key(&first_char) {
                    children.get_mut(&first_char).unwrap().add(rest, content)
                } else {
                    let new_node = TNode::Leaf {
                        content,
                        is_terminal: true,
                    };
                    children.insert(first_char, new_node);
                    return Ok(children.get(&first_char).unwrap());
                }
            }
        }
    }

    //  pub fn add(&mut self, s: &str, content: &Option<T>) -> Result<TNode<T>, KeyExists> {
    //      let first_char = s.chars().next().unwrap();
    //      let rest = &s[first_char.len_utf8()..];

    //      match self {
    //          TNode::Leaf {
    //              content: cont,
    //              is_terminal,
    //          } => {
    //              let mut new_node: TNode<T>;
    //              if rest.is_empty() {
    //                  new_node = TNode::Leaf {
    //                      content,
    //                      is_terminal: true,
    //                  };
    //              } else {
    //                  new_node = TNode::Leaf {
    //                      content: &None,
    //                      is_terminal: false,
    //                  };
    //                  new_node.add(rest, content);
    //              }
    //              *self = TNode::Node {
    //                  content: cont.as_deref_mut(),
    //                  is_terminal: *is_terminal,
    //                  children: BTreeMap::from([(first_char, new_node)]),
    //              };
    //              Ok(new_node)
    //          }
    //          TNode::Node {
    //              content: cont,
    //              children,
    //              is_terminal,
    //          } => {
    //              if children.contains_key(&first_char) {
    //                  let next_node = children.get(&first_char).unwrap();
    //                  return next_node.add(rest, content);
    //              } else {
    //                  let new_node = TNode::Leaf {
    //                      content,
    //                      is_terminal: rest.is_empty(),
    //                  };
    //                  children.insert(first_char, new_node);
    //                  return Ok(new_node);
    //              }
    //          }
    //      }
    //  }

    //  pub fn find<'a>(&'a mut self, s: &'a str, must_be_terminal: bool) -> Option<TNode<T>> {
    //      let lpo = LongestPrefOpts {
    //          must_be_terminal,
    //          must_match_fully: true,
    //      };
    //      self.longest_prefix_fn(s, None, "".to_owned(), lpo)
    //  }

    //  pub fn longest_prefix<'a>(
    //      &'a mut self,
    //      s: &'a str,
    //      must_be_terminal: bool,
    //  ) -> Option<TNode<T>> {
    //      let lpo = LongestPrefOpts {
    //          must_be_terminal,
    //          must_match_fully: false,
    //      };
    //      self.longest_prefix_fn(s, None, "".to_owned(), lpo)
    //  }

    //  fn longest_prefix_fn<'a>(
    //      &'a self,
    //      str_left: &'a str,
    //      last_terminal: Option<TNode<T>>,
    //      cur_pref: String,
    //      opts: LongestPrefOpts,
    //  ) -> Option<TNode<T>> {
    //      match self {
    //          TNode::Leaf {
    //              content,
    //              is_terminal,
    //          } => {
    //              let new_last_terminal = if *is_terminal {
    //                  Some(*self)
    //              } else {
    //                  last_terminal
    //              };
    //              if str_left.is_empty() {
    //                  return if opts.must_be_terminal && !is_terminal {
    //                      new_last_terminal
    //                  } else {
    //                      Some(*self)
    //                  };
    //              } else {
    //                  None
    //              }
    //          }
    //          TNode::Node {
    //              content,
    //              children,
    //              is_terminal,
    //          } => {
    //              let new_last_terminal = if *is_terminal {
    //                  Some(*self)
    //              } else {
    //                  last_terminal
    //              };
    //              if str_left.is_empty() {
    //                  return if opts.must_be_terminal && !is_terminal {
    //                      last_terminal
    //                  } else {
    //                      Some(*self)
    //                  };
    //              };

    //              let first_char = str_left.chars().next().unwrap();
    //              let rest = &str_left[first_char.len_utf8()..];
    //              if !children.contains_key(&first_char) {
    //                  return None;
    //              }
    //              let next_node = children.get(&first_char).unwrap();
    //              return next_node.longest_prefix_fn(str_left, new_last_terminal, cur_pref, opts);
    //          }
    //      }
    //  }

    //  fn pp(&self, print_content: bool) -> String {
    //      return self.pp_fn(0, print_content);
    //  }

    //  fn pp_fn(&self, indent: u8, print_content: bool) -> String {
    //      let mut res = String::from("");
    //      match &self {
    //          TNode::Leaf {
    //              content,
    //              is_terminal,
    //          } => {
    //              return "".to_owned();
    //          }
    //          TNode::Node {
    //              content,
    //              children,
    //              is_terminal,
    //          } => {
    //              let iter = children.iter();

    //              let child_count = children.len();

    //              for (k, v) in iter {
    //                  if *is_terminal || child_count > 1 {
    //                      res.push('\n');
    //                      res.push_str(&" ".repeat(indent.into()));
    //                  }

    //                  res.push_str(&k.to_string());

    //                  if print_content {
    //                      match *v {
    //                          TNode::Leaf {
    //                              content,
    //                              is_terminal,
    //                          } => {
    //                              res.push_str(&format!("  {}", v));
    //                          }
    //                          TNode::Node {
    //                              content,
    //                              children,
    //                              is_terminal,
    //                          } => {
    //                              res.push_str(&format!("  {}", v));
    //                          }
    //                      }
    //                  }

    //                  if print_content && *is_terminal {}

    //                  res.push_str(v.pp_fn(indent + 1, print_content).as_str());
    //              }
    //              res
    //          }
    //      }
    //  }
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
    //
    // #[test]
    // fn add_empty_string() {
    //     let mut t = TNode::Leaf {
    //         content: None,
    //         is_terminal: true,
    //     };
    //     t.add("", Some(1));
    //     println!("{}", t.pp(true));
    //     //assert_eq!(t.content, Some(1));
    //     assert_eq!(1, 2);
    // }
    //
    //#[test]
    //fn add_single_char_string() {
    //    let mut t = Trie::new(None);
    //    t.add("a", Some(1));
    //    t.add("ab", Some(1));
    //    t.add("c", Some(1));
    //    t.add("d", Some(1));
    //    assert_eq!(t.pp(false), "\na\n b\nc\nd")
    //}
    //
    //    #[test]
    //    fn show_content() {
    //        let mut t = Trie::new(None);
    //        t.add("a", Some(1));
    //        t.add("abc", Some(2));
    //        t.add("d", Some(3));
    //        t.add("e", Some(4));
    //        let s = t.pp(true);
    //        assert_eq!(s, "\na  (1)\n bc  (2)\nd  (3)\ne  (4)")
    //    }
    //
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
