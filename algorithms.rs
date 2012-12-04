use core::option;

type NodeLink<T> = Option<Node<T>>;

// A node in a singly-linked list.
pub enum Node<T> {
  // I don't understand this syntax.
  Node_(@{
    mut data: T,
    mut next: NodeLink<T>
  })
}

pure fn Node<T>(initial_value: T) -> Node<T> {
  let node = Node_(@{
    mut data: initial_value,
    mut next: None
  });

  return node;
}

fn append_to_node<T>(node: Node<T>, next_value: T) -> Node<T> {
  let next = Node(next_value);
  node.next = Some(next);

  return node;
}



fn prepend_to_node<T>(node: Node<T>, previous_value: T) -> Node<T> {
  let first = Node(previous_value);
  first.next = Some(node);

  return first;
}


/*
fn find_in_list<T>(start: Node<T>, element: T, comparator: fn(&T) -> bool ) -> Option<Node<T>> {

  if (comparator(&start.value)) {
    return Some(start);
  }

  match start.next {
    @None => return None,
    @Some(node) => return find_in_list(node, element, comparator)
  };
}
*/

#[test]
fn test_algorithms() {
  info!("hello");

}

fn main() {

}
