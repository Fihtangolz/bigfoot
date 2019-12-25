use std::vec::Vec;

struct BPTree<Key, Val> {
    degree: usize,
    root: Node<Key, Val>
}

struct Pair<Key, Val> {
    key: Key,
    val: Val
}

struct Node<Key, Val> {
    val: Vec<Pair<Key, Val>>,
    children: Vec<*mut Self>,
}

impl BPTree<Key, Val> {
    pub fn new() -> Self {
        return Self{
            degree: 64,
        }
    }

    pub fn insert(&mut self, key: Key, item: Val) {

    }

    pub fn search(key: Key) -> &Node {}
    pub fn delete() -> bool {}
    pub fn clear() {}
}