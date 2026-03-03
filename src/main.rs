#[derive(Debug)]
struct Node {
    val: i32,
    next: Option<Box<Node>>,
}

impl Node {
    fn new(val: i32) -> Self {
        Self { val, next: None }
    }
}

// Создать список из массива чисел (для удобства ввода/теста)
fn from_vec(values: Vec<i32>) -> Option<Box<Node>> {
    let mut head: Option<Box<Node>> = None;
    for v in values.into_iter().rev() {
        let mut n = Box::new(Node::new(v));
        n.next = head;
        head = Some(n);
    }
    head
}

// Превратить список обратно в Vec (чтобы красиво вывести)
fn to_vec(mut head: &Option<Box<Node>>) -> Vec<i32> {
    let mut out = Vec::new();
    while let Some(node) = head.as_ref() {
        out.push(node.val);
        head = &node.next;
    }
    out
}

// Разделить список на две половины: (left, right)
fn split(mut head: Option<Box<Node>>) -> (Option<Box<Node>>, Option<Box<Node>>) {
    // slow/fast pointers (как в С)
    let mut slow = &mut head;
    let mut fast = &mut head;

    while fast.as_ref().and_then(|n| n.next.as_ref()).is_some() {
        fast = &mut fast.as_mut().unwrap().next;
        if fast.as_ref().and_then(|n| n.next.as_ref()).is_some() {
            fast = &mut fast.as_mut().unwrap().next;
            slow = &mut slow.as_mut().unwrap().next;
        }
    }

    // slow сейчас примерно в середине, отрезаем правую часть
    let right = slow.as_mut().and_then(|n| n.next.take());
    (head, right)
}

// Слить два отсортированных списка в один
fn merge(a: Option<Box<Node>>, b: Option<Box<Node>>) -> Option<Box<Node>> {
    match (a, b) {
        (None, x) => x,
        (x, None) => x,
        (Some(mut na), Some(mut nb)) => {
            if na.val <= nb.val {
                let next = na.next.take();
                na.next = merge(next, Some(nb));
                Some(na)
            } else {
                let next = nb.next.take();
                nb.next = merge(Some(na), next);
                Some(nb)
            }
        }
    }
}

// Merge sort для списка
fn merge_sort(head: Option<Box<Node>>) -> Option<Box<Node>> {
    // 0 или 1 элемент — уже отсортировано
    if head.as_ref().and_then(|n| n.next.as_ref()).is_none() {
        return head;
    }
    let (left, right) = split(head);
    let left_sorted = merge_sort(left);
    let right_sorted = merge_sort(right);
    merge(left_sorted, right_sorted)
}

fn main() {
    // Пример входных данных (можешь заменить на свои)
    let head = from_vec(vec![5, 1, 9, 3, 3, -2, 10, 0]);

    println!("До:  {:?}", to_vec(&head));

    let sorted = merge_sort(head);

    println!("После: {:?}", to_vec(&sorted));
}