#[derive(Debug)]
struct Node {
    val: i32,
    next: Option<Box<Node>>, // ссылка на следующий элемент
}

impl Node {
    // Конструктор
    fn new(val: i32) -> Self {
        Self {
            val,
            next: None,
        }
    }

    // Добавление элемента в конец списка
    fn append(&mut self, val: i32) {
        match self.next {
            Some(ref mut next_node) => {
                // если следующий элемент есть — идем дальше
                next_node.append(val);
            }
            None => {
                // если следующего нет — создаем новый
                self.next = Some(Box::new(Node::new(val)));
            }
        }
    }

    // Печать всего списка
    fn print(&self) {
        println!("{}", self.val);

        if let Some(ref next_node) = self.next {
            next_node.print();
        }
    }
}

fn main() {
    // создаем первый элемент (голову списка)
    let mut head = Node::new(10);

    // добавляем ещё 4 числа
    head.append(20);
    head.append(30);
    head.append(40);
    head.append(50);

    // выводим список
    head.print();
}