struct Node {
    value: String,
    children: Vec<usize>,
}

struct Arena {
    nodes: Vec<Node>,
}

impl Arena {
    fn new() -> Self {
        Arena { nodes: Vec::new() }
    }

    fn alloc(&mut self, node: Node) -> usize {
        let id = self.nodes.len();
        self.nodes.push(node);
        id
    }

    fn get(&self, id: usize) -> &Node {
        &self.nodes[id]
    }
}

fn main() {
    let mut arena = Arena::new();

    let child1 = arena.alloc(Node { value: "child1".into(), children: vec![] });
    let child2 = arena.alloc(Node { value: "child2".into(), children: vec![] });
    let root = arena.alloc(Node {
        value: "root".into(),
        children: vec![child1, child2],
    });

    println!("Root: {}", arena.get(root).value);
}