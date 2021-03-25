use std::{
    cell::{Cell, RefCell},
    collections::HashMap,
    fmt::Debug,
    hash::Hash,
    rc::{Rc, Weak},
};

type RcNode<T> = Rc<Node<T>>;
type WeakRcNode<T> = Weak<Node<T>>;

// Node represents the single node in a tree.
#[derive(Debug)]
struct Node<T> {
    item: Option<T>,
    count: Cell<usize>,
    children: RefCell<Vec<RcNode<T>>>,
    // Use Weak reference here to prevent the reference cycle.
    parent: RefCell<WeakRcNode<T>>,
    // The node's neighbor is the one with the same value that is "to the right"
    // of it in the tree.
    neighbor: RefCell<WeakRcNode<T>>,
}

impl<T> PartialEq for Node<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Node<T>) -> bool {
        self.item == other.item && self.parent.borrow().upgrade() == other.parent.borrow().upgrade()
    }
}

impl<T> Node<T>
where
    T: PartialEq + Copy + Debug,
{
    // Create a new Node with the given item and count.
    fn new(item: Option<T>, count: usize) -> Node<T> {
        Node {
            item,
            count: Cell::new(count),
            children: RefCell::new(vec![]),
            parent: Default::default(),
            neighbor: Default::default(),
        }
    }

    // Create a new Rc<Node> with the given item and count.
    fn new_rc(item: Option<T>, count: usize) -> RcNode<T> {
        Rc::new(Self::new(item, count))
    }

    // Add the given child Node as a child of this node.
    fn add_child(self: &Rc<Self>, child_node: RcNode<T>) {
        let mut children = self.children.borrow_mut();
        if !children.contains(&Rc::clone(&child_node)) {
            *child_node.parent.borrow_mut() = Rc::downgrade(&Rc::clone(self));
            children.push(Rc::clone(&child_node))
        }
    }

    // Check whether this node contains a child node for the given item.
    // If so, that node's reference is returned; otherwise, `None` is returned.
    fn search(&self, item: T) -> Option<RcNode<T>> {
        for node in self.children.borrow().iter() {
            if let Some(child_node_item) = node.item {
                if child_node_item == item {
                    return Some(Rc::clone(node));
                }
            }
        }
        None
    }

    // Check whether exists a child Node holds the given item.
    fn child_contains(&self, item: T) -> bool {
        for node in self.children.borrow().iter() {
            if let Some(child_node_item) = node.item {
                if child_node_item == item {
                    return true;
                }
            }
        }
        false
    }

    // Increment the count associated with this node's item.
    fn increment(&self) {
        let old_count = self.count.get();
        self.count.set(old_count + 1);
    }

    fn print(&self, depth: usize) {
        let padding = " ".repeat(depth);
        let node_info;
        match self.is_root() {
            true => node_info = "<(root)>".to_string(),
            false => node_info = format!("<{:?} {} (node)>", self.item, self.count.get()),
        }
        println!("{}{}", padding, node_info);
        for child in self.children.borrow().iter() {
            child.print(depth + 1);
        }
    }

    fn neighbor(&self) -> Option<RcNode<T>> {
        self.neighbor.borrow().upgrade()
    }

    fn is_root(&self) -> bool {
        self.item == None && self.count.get() == 0
    }

    fn is_leaf(&self) -> bool {
        self.children.borrow().len() == 0
    }
}

// Tree represents the main tree data struct will be used during the FP-Growth algorithm.
pub struct Tree<T> {
    root_node: RefCell<RcNode<T>>,
    // routes is a HashMap who maintains a mapping which satisfies item -> (Head node, tail node).
    routes: HashMap<T, (RefCell<RcNode<T>>, RefCell<RcNode<T>>)>,
}

impl<T> Tree<T>
where
    T: Eq + Copy + Hash + Debug,
{
    // Create a new FP-Growth tree with an empty root node.
    pub fn new() -> Tree<T> {
        Tree {
            root_node: RefCell::new(Node::new_rc(None, 0)),
            routes: HashMap::new(),
        }
    }

    // Iterate the transaction and add every item to the FP-Growth tree.
    pub fn add_transaction(&mut self, transaction: Vec<T>) {
        let mut cur_node = Rc::clone(&self.root_node.borrow());
        for item in transaction.into_iter() {
            match cur_node.search(item) {
                // There is already a node in this tree for the current
                // transaction item; reuse it.
                Some(child_node) => {
                    child_node.increment();
                    cur_node = child_node;
                }
                None => {
                    let next_node = Node::new_rc(Some(item), 1);
                    cur_node.add_child(Rc::clone(&next_node));
                    self.update_route(Rc::clone(&next_node));
                    cur_node = next_node;
                }
            }
        }
    }

    fn update_route(&mut self, node: RcNode<T>) {
        if let Some(item) = node.item {
            match self.routes.get(&item) {
                Some((_, tail)) => {
                    tail.replace_with(|old_tail| {
                        *old_tail.neighbor.borrow_mut() = Rc::downgrade(&Rc::clone(&node));
                        node
                    });
                }
                None => {
                    self.routes.insert(
                        item,
                        (
                            RefCell::new(Rc::clone(&node)),
                            RefCell::new(Rc::clone(&node)),
                        ),
                    );
                }
            }
        }
    }

    pub fn print(&self) {
        println!("Tree:");
        self.root_node.borrow().print(1);
        println!("Routes:");
        for (item, route) in self.routes.iter() {
            println!("Item: {:?}", *item);
            let mut cur_node = Rc::clone(&route.0.borrow());
            while let Some(neighbor_node) = cur_node.neighbor() {
                println!("<{:?} {}>", neighbor_node.item, neighbor_node.count.get());
                cur_node = neighbor_node;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::tree::{Node, Tree};
    use std::rc::Rc;

    #[test]
    fn test_node() {
        let root_node = Node::<i32>::new_rc(None, 0);
        let child_node_1 = Rc::new(Node::<i32>::new(Some(1), 1));
        let child_node_2 = Rc::new(Node::<i32>::new(Some(2), 2));

        root_node.add_child(Rc::clone(&child_node_1));
        child_node_1.add_child(Rc::clone(&child_node_2));

        assert!(root_node.is_root());
        assert!(!root_node.is_leaf());
        assert!(root_node.child_contains(1));
        assert!(!root_node.child_contains(2));
        assert_eq!(root_node.search(1), Some(Rc::clone(&child_node_1)));
        assert_eq!(root_node.search(2), None);
        assert_eq!(root_node.item, None);

        assert!(!child_node_1.is_root());
        assert!(!child_node_1.is_leaf());
        assert!(!child_node_1.child_contains(1));
        assert!(child_node_1.child_contains(2));
        assert_eq!(child_node_1.search(1), None);
        assert_eq!(child_node_1.search(2), Some(Rc::clone(&child_node_2)));
        assert_eq!(child_node_1.item, Some(1));

        assert!(!child_node_2.is_root());
        assert!(child_node_2.is_leaf());
        assert!(!child_node_2.child_contains(1));
        assert!(!child_node_2.child_contains(2));
        assert_eq!(child_node_2.search(1), None);
        assert_eq!(child_node_2.search(2), None);
        assert_eq!(child_node_2.item, Some(2));
    }

    #[test]
    fn test_tree() {
        let mut tree = Tree::<&str>::new();
        let transactions = vec![
            vec!["a", "c", "e", "b", "f"],
            vec!["a", "c", "g"],
            vec!["e"],
            vec!["a", "c", "e", "g", "d"],
            vec!["a", "c", "e", "g"],
            vec!["e"],
            vec!["a", "c", "e", "b", "f"],
            vec!["a", "c", "d"],
            vec!["a", "c", "e", "g"],
            vec!["a", "c", "e", "g"],
        ];
        for transaction in transactions.into_iter() {
            tree.add_transaction(transaction);
        }
        tree.print();
    }
}
