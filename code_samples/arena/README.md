<img src="../../rusting-crab-logo.png" height=300>

[**Cleuton Sampaio**](https://linkedin.com/in/cleutonsampaio)

[**Veja no GitHub**](https://https://github.com/cleuton/rustingcrab/tree/main/code_samples/arena)

# Arena

Arena é uma forma de agrupar todas as alocações de objetos numa área única (como um `Vec`), referenciá‑los por índices em vez de usar ponteiros individuais e só liberar toda a memória de uma vez, o que reduz overhead, evita fragmentação e simplifica referências em estruturas interligadas.

Sim, já ouvi falar do padrão **Arena** em Rust! Ele é bastante útil em contextos onde você precisa alocar muitos objetos inter-relacionados e quer otimizar o gerenciamento de memória e o desempenho.

## Por que Arena?

Todos os objetos são alocados em uma única região de memória (a "arena") e liberados todos de uma vez, ao invés de liberar cada objeto individualmente. Isso evita a fragmentação de memória e reduz a sobrecarga do gerenciamento de memória.

## Relação com Rust

Rust permite controle preciso sobre alocação e tempo de vida, e o padrão Arena se beneficia disso para:

- Evitar o uso excessivo de `Rc<T>` ou `Box<T>`.
- Reduzir indireções e custos de _drop_.
- Facilitar estruturas de dados interconectadas (como árvores ou grafos) com referências seguras usando `&T` ou `NodeId`.

## Exemplo simples

Aqui vai um exemplo simples, baseado em uma árvore sintática abstrata (AST) para uma linguagem simples, usando uma arena:

```rust
struct Node {
    value: String,
    children: Vec<usize>, // índices dos filhos na arena
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
```

## Benefícios

- **Desempenho**: Alocação e desalocação rápidas.
- **Segurança**: Evita ciclos de referência e uso de `Rc<RefCell<T>>`.
- **Simplicidade**: Estruturas de dados complexas com referências seguras.
- **Drop em lote**: Tudo é liberado ao mesmo tempo.

## Quando usar Arena?

- Quando você está construindo ASTs, grafos, ou estruturas hierárquicas.
- Quando precisa de desempenho e controle de memória.
- Quando quer evitar o _ownership hell_ com referências seguras.

