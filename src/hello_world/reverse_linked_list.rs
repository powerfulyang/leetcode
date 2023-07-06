#[allow(dead_code)]
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Node {
    value: i32,
    next: Option<Box<Node>>,
}

#[allow(dead_code)]
impl Node {
    fn new(value: i32) -> Node {
        Node { value, next: None }
    }
}

#[allow(dead_code)]
pub fn reverse_linked_list(mut head: Option<Box<Node>>) -> Option<Box<Node>> {
    let mut prev: Option<Box<Node>> = None;
    while let Some(mut boxed_node) = head {
        head = boxed_node.next.take();
        boxed_node.next = prev;
        prev = Some(boxed_node);
    }
    prev
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_take() {
        let mut x = Some(2);
        let y = x.take();
        assert_eq!(x, None);
        assert_eq!(y, Some(2));

        let mut x: Option<u32> = None;
        let y = x.take();
        assert_eq!(x, None);
        assert_eq!(y, None);
    }

    #[test]
    fn test() {
        let mut list = Node::new(1);
        list.next = Some(Box::new(Node::new(2)));
        list.next.as_mut().unwrap().next = Some(Box::new(Node::new(3)));
        list.next.as_mut().unwrap().next.as_mut().unwrap().next = Some(Box::new(Node::new(4)));
        list.next
            .as_mut()
            .unwrap()
            .next
            .as_mut()
            .unwrap()
            .next
            .as_mut()
            .unwrap()
            .next = Some(Box::new(Node::new(5)));
        let reversed_list = reverse_linked_list(Option::from(Box::from(list.clone())));
        assert_eq!(reversed_list.as_ref().unwrap().value, 5);
        assert_eq!(
            reversed_list.as_ref().unwrap().next.as_ref().unwrap().value,
            4
        );
        let reversed_list = reverse_linked_list_recursive(Option::from(Box::from(list.clone())));
        assert_eq!(
            reversed_list
                .as_ref()
                .unwrap()
                .next
                .as_ref()
                .unwrap()
                .next
                .as_ref()
                .unwrap()
                .value,
            3
        );
        assert_eq!(
            reversed_list
                .as_ref()
                .unwrap()
                .next
                .as_ref()
                .unwrap()
                .next
                .as_ref()
                .unwrap()
                .next
                .as_ref()
                .unwrap()
                .value,
            2
        );
        assert_eq!(
            reversed_list
                .as_ref()
                .unwrap()
                .next
                .as_ref()
                .unwrap()
                .next
                .as_ref()
                .unwrap()
                .next
                .as_ref()
                .unwrap()
                .next
                .as_ref()
                .unwrap()
                .value,
            1
        );
    }
}

#[allow(dead_code)]
fn reverse_linked_list_recursive(head: Option<Box<Node>>) -> Option<Box<Node>> {
    fn reverse_linked_list_recursive_helper(
        mut head: Option<Box<Node>>,
        prev: Option<Box<Node>>,
    ) -> Option<Box<Node>> {
        if let Some(mut boxed_node) = head {
            head = boxed_node.next.take();
            boxed_node.next = prev;
            reverse_linked_list_recursive_helper(head, Some(boxed_node))
        } else {
            prev
        }
    }
    reverse_linked_list_recursive_helper(head, None)
}
