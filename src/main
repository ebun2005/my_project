#[derive(Debug)]
struct Node {
    val: i32,
}

impl Node {
    // Конструктор
    fn new(val: i32) -> Self {
        Self { val }
    }

    // Метод для печати значения
    fn print(&self) {
        println!("{}", self.val);
    }
}

fn main() {
    // Создаем вектор (массив динамический) для хранения Node
    let mut nodes: Vec<Node> = Vec::new();

    // Добавляем 5 чисел
    nodes.push(Node::new(10));
    nodes.push(Node::new(20));
    nodes.push(Node::new(30));
    nodes.push(Node::new(40));
    nodes.push(Node::new(50));
    let node1= &nodes[3] ;
    // Выводим все элементы
    node1.print();
    for node in &nodes {
        node.print();
    }
}