use trie_generic::Trie;

fn main() {
    let mut t = Trie::<i32>::new(None);

    t.add("https://google.com", Some(1));
    t.add("http://wikipedia.org", Some(1));
    t.add("https://imdb.com", Some(1));

    println!("{}", t.pp());
}
