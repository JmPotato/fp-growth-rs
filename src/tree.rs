//! `Tree` implements the tree data struct in FP-Growth algorithm.

use std::{
    cell::{Cell, RefCell},
    collections::HashMap,
    fmt::Debug,
    rc::{Rc, Weak},
    usize,
};

use crate::ItemType;

type RcNode<T> = Rc<Node<T>>;
type WeakRcNode<T> = Weak<Node<T>>;

/// `Node<T>` represents the single node in a tree.
#[derive(Debug)]
pub struct Node<T> {
    item: Option<T>,
    count: Cell<usize>,
    children: RefCell<Vec<RcNode<T>>>,
    // Use Weak reference here to prevent the reference cycle.
    parent: RefCell<WeakRcNode<T>>,
    // The node's neighbor is the one with the same value that is "to the right"
    // of it in the tree.
    neighbor: RefCell<WeakRcNode<T>>,
}

impl<T: ItemType> PartialEq for Node<T> {
    fn eq(&self, other: &Node<T>) -> bool {
        self.item == other.item && self.parent.borrow().upgrade() == other.parent.borrow().upgrade()
    }
}

impl<T: ItemType> Node<T> {
    /// Create a new Node with the given item and count.
    pub fn new(item: Option<T>, count: usize) -> Node<T> {
        Node {
            item,
            count: Cell::new(count),
            children: RefCell::new(vec![]),
            parent: Default::default(),
            neighbor: Default::default(),
        }
    }

    /// Create a new Rc<Node> with the given item and count.
    pub fn new_rc(item: Option<T>, count: usize) -> RcNode<T> {
        Rc::new(Self::new(item, count))
    }

    /// Add the given child Node as a child of this node.
    pub fn add_child(self: &Rc<Self>, child_node: RcNode<T>) {
        let mut children = self.children.borrow_mut();
        if !children.contains(&child_node) {
            *child_node.parent.borrow_mut() = Rc::downgrade(self);
            children.push(child_node);
        }
    }

    pub fn remove_child(self: &Rc<Self>, child_node: RcNode<T>) {
        let mut children = self.children.borrow_mut();
        // for (index, node) in children.clone().into_iter().enumerate() {
        //     if node == child_node {
        //         children.remove(index);
        //     }
        // }
        let index = children.iter().position(|x| *x == child_node).unwrap();
        children.remove(index);
    }

    /// Check whether this node contains a child node for the given item.
    /// If so, that node's reference is returned; otherwise, `None` is returned.
    pub fn search(&self, item: T) -> Option<RcNode<T>> {
        for node in self.children.borrow().iter() {
            if let Some(child_node_item) = node.item {
                if child_node_item == item {
                    return Some(Rc::clone(node));
                }
            }
        }
        None
    }

    /// Increment the count associated with this node's item.
    pub fn increment(&self, incr_count: usize) {
        let old_count = self.count.get();
        self.count.set(old_count + incr_count);
    }

    /// Print out the node.
    pub fn print(&self, depth: usize) {
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

    pub fn item(&self) -> Option<T> {
        self.item
    }

    /// Return the count value this node's item holds.
    pub fn count(&self) -> usize {
        self.count.get()
    }

    /// Return this node's neighbor node.
    pub fn neighbor(&self) -> Option<RcNode<T>> {
        self.neighbor.borrow().upgrade()
    }

    /// Return this node's parent node.
    pub fn parent(&self) -> Option<RcNode<T>> {
        self.parent.borrow().upgrade()
    }

    /// Check whether this node is a root node.
    pub fn is_root(&self) -> bool {
        self.item == None && self.count.get() == 0
    }

    /// Check whether this node is a leaf node.
    pub fn is_leaf(&self) -> bool {
        self.children.borrow().len() == 0
    }
}

type Route<T> = (RefCell<RcNode<T>>, RefCell<RcNode<T>>);

/// `Tree<T>` represents the main tree data struct will be used during the FP-Growth algorithm.
pub struct Tree<T> {
    root_node: RefCell<RcNode<T>>,
    // routes is a HashMap who maintains a mapping which satisfies item -> (Head node, tail node).
    routes: HashMap<T, Route<T>>,
}

impl<T: ItemType> Default for Tree<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: ItemType> Tree<T> {
    /// Create a new FP-Growth tree with an empty root node.
    pub fn new() -> Tree<T> {
        Tree {
            root_node: RefCell::new(Node::new_rc(None, 0)),
            routes: HashMap::new(),
        }
    }

    /// Generate a partial tree with the given paths.
    /// This function will be called during the algorithm.
    pub fn generate_partial_tree(paths: &[Vec<RcNode<T>>]) -> Tree<T> {
        let mut partial_tree = Tree::new();
        let mut leaf_item = None;
        for path in paths.iter() {
            // Get leaf_count from the leaf node.
            leaf_item = Some(path.last().unwrap().item.unwrap());
            let mut cur_node = Rc::clone(&partial_tree.root_node.borrow());
            for path_node in path.iter() {
                match cur_node.search(path_node.item.unwrap()) {
                    Some(child_node) => {
                        cur_node = child_node;
                    }
                    None => {
                        let next_node = Node::new_rc(path_node.item, {
                            let mut count = 0;
                            if path_node.item == leaf_item {
                                count = path_node.count.get();
                            }
                            count
                        });
                        cur_node.add_child(Rc::clone(&next_node));
                        partial_tree.update_route(Rc::clone(&next_node));
                        cur_node = next_node;
                    }
                }
            }
        }

        // Calculate the counts of the non-leaf nodes.
        for path in partial_tree.generate_prefix_path(leaf_item.unwrap()).iter() {
            let leaf_count = path.last().unwrap().count.get();
            for path_node in path[..path.len() - 1].iter() {
                path_node.increment(leaf_count);
            }
        }

        partial_tree
    }

    /// Iterate the transaction and add every item to the FP-Growth tree.
    pub fn add_transaction(&mut self, transaction: Vec<T>) {
        let mut cur_node = Rc::clone(&self.root_node.borrow());
        for &item in transaction.iter() {
            match cur_node.search(item) {
                // There is already a node in this tree for the current
                // transaction item; reuse it.
                Some(child_node) => {
                    child_node.increment(1);
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

    /// Update the route table that records the item and its node list.
    pub fn update_route(&mut self, node: RcNode<T>) {
        if let Some(item) = node.item {
            match self.routes.get(&item) {
                Some((_, tail)) => {
                    let old_tail = tail.replace(Rc::clone(&node));
                    *old_tail.neighbor.borrow_mut() = Rc::downgrade(&node);
                }
                None => {
                    self.routes
                        .insert(item, (RefCell::new(Rc::clone(&node)), RefCell::new(node)));
                }
            }
        }
    }

    /// Generate the prefix paths that end with the given item.
    pub fn generate_prefix_path(&self, item: T) -> Vec<Vec<RcNode<T>>> {
        let mut cur_end_node = Rc::clone(&self.routes.get(&item).unwrap().0.borrow());
        let mut paths = vec![];
        loop {
            let mut cur_node = Rc::clone(&cur_end_node);
            let mut path = vec![Rc::clone(&cur_node)];
            while let Some(parent_node) = cur_node.parent() {
                if parent_node.is_root() {
                    break;
                }
                path.push(Rc::clone(&parent_node));
                cur_node = parent_node;
            }
            path.reverse();
            paths.push(path);
            match cur_end_node.neighbor() {
                Some(neighbor_node) => cur_end_node = neighbor_node,
                None => break,
            }
        }
        paths
    }

    /// Get all nodes that holds the given item.
    pub fn get_all_nodes(&self, item: T) -> Vec<RcNode<T>> {
        match self.routes.get(&item) {
            None => vec![],
            Some((head_node, _)) => {
                let mut nodes = vec![Rc::clone(&head_node.borrow())];
                let mut cur_node = Rc::clone(&head_node.borrow());
                while let Some(neighbor_node) = cur_node.neighbor() {
                    nodes.push(Rc::clone(&neighbor_node));
                    cur_node = neighbor_node;
                }
                nodes
            }
        }
    }

    /// Get all nodes with the given item.
    pub fn get_all_items_nodes(&self) -> Vec<(T, Vec<RcNode<T>>)> {
        let mut items_nodes = vec![];
        for (item, _) in self.routes.iter() {
            items_nodes.push((*item, self.get_all_nodes(*item)));
        }
        items_nodes
    }

    #[allow(dead_code)]
    // [W.I.P] Prune the tree to reduce the search space.
    fn prune(&self) {
        let items_nodes = self.get_all_items_nodes();
        for (item, nodes) in items_nodes.iter() {
            if nodes.len() == 1 {
                continue;
            }
            // Find all paths this item belongs to.
            let mut all_paths = Vec::with_capacity(nodes.len());
            let mut leaf_node_count = Vec::with_capacity(nodes.len());
            for node in nodes.iter() {
                if !node.is_leaf() {
                    continue;
                }
                leaf_node_count.push(node.count());
                let mut path = vec![];
                let mut cur_node = Rc::clone(node);
                while !cur_node.is_root() {
                    path.push(Rc::clone(&cur_node));
                    let parent_node = cur_node.parent().unwrap();
                    cur_node = Rc::clone(&parent_node);
                }
                path.push(cur_node);
                path.reverse();
                all_paths.push(path);
            }
            if all_paths.len() < 2 {
                continue;
            }
            // Find the common ancestor for all paths.
            let mut common_ancestor_index = 0;
            let mut common_ancestor = None;
            for (index, node) in all_paths[0].iter().enumerate() {
                let mut is_ancestor = true;
                for path in all_paths.iter().skip(1) {
                    let cur_node = Rc::clone(&path[index]);
                    if cur_node != Rc::clone(node) {
                        is_ancestor = false;
                        break;
                    }
                }
                if !is_ancestor {
                    break;
                }
                common_ancestor_index = index;
                common_ancestor = Some(node);
            }
            // Prune nodes which start from the common ancestor.
            for (path_index, path) in all_paths.iter().enumerate() {
                for node in path.iter().skip(common_ancestor_index + 1) {
                    if node.count() <= leaf_node_count[path_index] {
                        let parent_node = node.parent().unwrap();
                        parent_node.remove_child(Rc::clone(node));
                        break;
                    }
                }
            }
            common_ancestor
                .unwrap()
                .add_child(Node::new_rc(Some(*item), leaf_node_count.iter().sum()));
        }
    }

    #[allow(dead_code)]
    /// Print out the tree.
    pub fn print(&self) {
        println!("Tree:");
        self.root_node.borrow().print(1);
        println!("Routes:");
        for (item, _) in self.routes.iter() {
            println!("Item: {:?}", *item);
            for node in self.get_all_nodes(*item).iter() {
                println!("{:?}", Rc::into_raw(Rc::clone(node)));
                println!("<{:?} {}>", node.item, node.count.get());
            }
        }
    }
}
