use trie_generic::TNode;

fn main() {
    let mut t = TNode::<i32>::Empty;

    t.add("https://google.com", &Some(1)).unwrap();
    t.add("http://wikipedia.org", &Some(2)).unwrap();
    t.add("https://imdb.com", &Some(3)).unwrap();

    //println!("{}", t.pp(true));
}
