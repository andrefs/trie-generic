pub mod trie {
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
        pub root: TNode<T>,
    }

    impl<T> Trie<T> {
        pub fn new(content: Option<T>) -> Trie<T> {
            Trie {
                root: TNode::new(content),
            }
        }

        //pub fn add(&mut self, s: &str, content: Option<T>) {
        //    add_fn(s, content, &mut self.root);
        //}

        pub fn pp(& self) -> String {
            pp_fn(&self.root, 0)
        }


    }

    fn pp_fn<T>(node: &TNode<T>, indent: u8) -> String {
        let mut res = String::from("");
        let iter = node.children.iter();

        if node.children.len() == 1 {
            let (k,v) = iter.last().unwrap();
            if node.is_terminal {
                res.push('\n');
                res.push_str(&" ".repeat(indent.into()));
                res.push_str(&k.to_string());
            } else {
                res.push_str(&k.to_string())
            }
            res.push_str(&pp_fn(v, indent+1));
        } else {
            for (k, v) in iter {
                res.push('\n');
                res.push_str(&" ".repeat(indent.into()));
                res.push_str(&k.to_string());
                res.push_str(&pp_fn(v, indent+1));
            }
        }
        res
    }

    //fn add_fn<T>(s: &str, content: Option<T>, node: &mut TNode<T>) {
    //    if s.is_empty() {
    //        node.is_terminal = true;
    //        node.content = content;
    //        return;
    //    }
    //    let mut chars = s.chars();
    //    let c = chars.next().unwrap();
    //    let rest = chars.as_str();

    //    node.children.entry(c).or_insert(TNode::new(None));
    //    //let mut subtrie = node.children.get(&c).expect("char must exist");
    //    //add_fn(rest, content, subtrie);
    //}

}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use super::*;
    use trie::*;

    #[test]
    fn create_tnode(){
        let n = TNode::new(Some(1));
        assert_eq!(n.content, Some(1));
        assert!(!n.is_terminal);
        assert!(n.children.is_empty());
    }

    #[test]
    fn pretty_print(){
        let t: Trie<u8> = Trie {
            root: TNode {
                is_terminal: false,
                content: None,
                children: BTreeMap::from([
                    ('a', TNode {
                        is_terminal: false,
                        content: None,
                        children: BTreeMap::from([
                            ('b', TNode {
                                is_terminal: true,
                                content: None,
                                children: BTreeMap::new()
                            })
                        ])
                    }),
                    ('c', TNode {
                        is_terminal: true,
                        content: None,
                        children: BTreeMap::new()
                    }),
                    ('d', TNode {
                        is_terminal: true,
                        content: None,
                        children: BTreeMap::new()
                    }),
                ])
            }
        };
        assert_eq!(t.pp(), "\nab\nc\nd")
    }

    // #[test]
    // fn add_empty_string(){
    //     let mut t = Trie::new(None);
    //     t.add("", Some(1));
    //     assert_eq!(t.root.content, Some(1));
    // }

    // #[test]
    // fn add_single_char_string(){
    //     let mut t = Trie::new(None);
    //     t.add("a", Some(1));
    //     // assert_eq!(t.root["a"].content, Some(1));
    //     panic!()
    // }
}





    //     fn pp(&self){
    //         pp_fn(&self.root, 0);
    //     }
    // }


    // fn pp_fn<T>(node: &TNode<T>, indent: u8){

    // }
