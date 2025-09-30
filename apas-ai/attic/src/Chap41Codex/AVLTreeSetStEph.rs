//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chap41Codex: single-threaded ephemeral AVL tree set based on structural balancing.

pub mod AVLTreeSetStEph {
    use crate::Chap18::ArraySeqStEph::ArraySeqStEph::{ArraySeqStEphS, ArraySeqStEphTrait};
    use crate::Types::Types::*;

    type Link<T> = Option<Box<Node<T>>>;

    #[derive(Clone)]
    struct Node<T: StT + Ord> {
        value: T,
        left: Link<T>,
        right: Link<T>,
        height: i32,
        size: N,
    }

    impl<T: StT + Ord> Node<T> {
        fn new(value: T) -> Self {
            Node {
                value,
                left: None,
                right: None,
                height: 1,
                size: 1,
            }
        }
    }

    fn height<T: StT + Ord>(link: &Link<T>) -> i32 {
        link.as_ref().map_or(0, |node| node.height)
    }

    fn subtree_size<T: StT + Ord>(link: &Link<T>) -> N {
        link.as_ref().map_or(0, |node| node.size)
    }

    fn update_metadata<T: StT + Ord>(node: &mut Box<Node<T>>) {
        let left_height = height(&node.left);
        let right_height = height(&node.right);
        node.height = 1 + left_height.max(right_height);
        node.size = 1 + subtree_size(&node.left) + subtree_size(&node.right);
    }

    fn balance_factor<T: StT + Ord>(node: &Node<T>) -> i32 {
        height(&node.left) - height(&node.right)
    }

    fn rotate_left<T: StT + Ord>(mut root: Box<Node<T>>) -> Box<Node<T>> {
        let mut pivot = root.right.take().expect("rotate_left requires non-empty right child");
        root.right = pivot.left.take();
        update_metadata(&mut root);
        pivot.left = Some(root);
        update_metadata(&mut pivot);
        pivot
    }

    fn rotate_right<T: StT + Ord>(mut root: Box<Node<T>>) -> Box<Node<T>> {
        let mut pivot = root.left.take().expect("rotate_right requires non-empty left child");
        root.left = pivot.right.take();
        update_metadata(&mut root);
        pivot.right = Some(root);
        update_metadata(&mut pivot);
        pivot
    }

    fn rebalance<T: StT + Ord>(mut node: Box<Node<T>>) -> Box<Node<T>> {
        update_metadata(&mut node);
        let bf = balance_factor(&node);
        if bf > 1 {
            if let Some(left_child) = node.left.as_mut() {
                if balance_factor(left_child) < 0 {
                    let left_taken = node.left.take().unwrap();
                    node.left = Some(rotate_left(left_taken));
                }
            }
            return rotate_right(node);
        }
        if bf < -1 {
            if let Some(right_child) = node.right.as_mut() {
                if balance_factor(right_child) > 0 {
                    let right_taken = node.right.take().unwrap();
                    node.right = Some(rotate_right(right_taken));
                }
            }
            return rotate_left(node);
        }
        node
    }

    fn insert_link<T: StT + Ord>(link: Link<T>, value: T) -> Link<T> {
        match link {
            | None => Some(Box::new(Node::new(value))),
            | Some(mut node) => {
                match value.cmp(&node.value) {
                    | std::cmp::Ordering::Less => {
                        node.left = insert_link(node.left.take(), value);
                    }
                    | std::cmp::Ordering::Greater => {
                        node.right = insert_link(node.right.take(), value);
                    }
                    | std::cmp::Ordering::Equal => {
                        return Some(node);
                    }
                }
                Some(rebalance(node))
            }
        }
    }

    fn pop_min<T: StT + Ord>(mut node: Box<Node<T>>) -> (Link<T>, T) {
        match node.left.take() {
            | None => {
                let right = node.right.take();
                (right, node.value)
            }
            | Some(left_child) => {
                let (new_left, min_value) = pop_min(left_child);
                node.left = new_left;
                let balanced = rebalance(node);
                (Some(balanced), min_value)
            }
        }
    }

    fn delete_link<T: StT + Ord>(link: Link<T>, value: &T) -> Link<T> {
        match link {
            | None => None,
            | Some(mut node) => {
                match value.cmp(&node.value) {
                    | std::cmp::Ordering::Less => {
                        node.left = delete_link(node.left.take(), value);
                    }
                    | std::cmp::Ordering::Greater => {
                        node.right = delete_link(node.right.take(), value);
                    }
                    | std::cmp::Ordering::Equal => {
                        if node.left.is_none() {
                            return node.right;
                        }
                        if node.right.is_none() {
                            return node.left;
                        }
                        let right = node.right.take().expect("right child exists");
                        let (new_right, successor_value) = pop_min(right);
                        node.value = successor_value;
                        node.right = new_right;
                    }
                }
                Some(rebalance(node))
            }
        }
    }

    fn contains_link<T: StT + Ord>(link: &Link<T>, value: &T) -> B {
        match link {
            | None => false,
            | Some(node) => match value.cmp(&node.value) {
                | std::cmp::Ordering::Less => contains_link(&node.left, value),
                | std::cmp::Ordering::Greater => contains_link(&node.right, value),
                | std::cmp::Ordering::Equal => true,
            },
        }
    }

    fn inorder_apply<T: StT + Ord, F: FnMut(&T)>(link: &Link<T>, visit: &mut F) {
        if let Some(node) = link {
            inorder_apply(&node.left, visit);
            visit(&node.value);
            inorder_apply(&node.right, visit);
        }
    }

    fn filter_collect<T: StT + Ord, F: Pred<T>>(link: &Link<T>, predicate: &F, out: &mut AVLTreeSetStEph<T>) {
        if let Some(node) = link {
            filter_collect(&node.left, predicate, out);
            if predicate(&node.value) {
                out.insert(node.value.clone());
            }
            filter_collect(&node.right, predicate, out);
        }
    }

    fn intersection_collect<T: StT + Ord>(link: &Link<T>, other: &AVLTreeSetStEph<T>, out: &mut AVLTreeSetStEph<T>) {
        if let Some(node) = link {
            intersection_collect(&node.left, other, out);
            if other.find(&node.value) {
                out.insert(node.value.clone());
            }
            intersection_collect(&node.right, other, out);
        }
    }

    fn difference_collect<T: StT + Ord>(link: &Link<T>, other: &AVLTreeSetStEph<T>, out: &mut AVLTreeSetStEph<T>) {
        if let Some(node) = link {
            difference_collect(&node.left, other, out);
            if !other.find(&node.value) {
                out.insert(node.value.clone());
            }
            difference_collect(&node.right, other, out);
        }
    }

    fn union_collect<T: StT + Ord>(link: &Link<T>, out: &mut AVLTreeSetStEph<T>) {
        if let Some(node) = link {
            union_collect(&node.left, out);
            out.insert(node.value.clone());
            union_collect(&node.right, out);
        }
    }

    #[derive(Clone, Default)]
    pub struct AVLTreeSetStEph<T: StT + Ord> {
        root: Link<T>,
    }

    /// Trait (typeclass interface) for single-threaded AVL tree sets.
    pub trait AVLTreeSetStEphTrait<T: StT + Ord> {
        fn size(&self) -> N;
        fn to_seq(&self) -> ArraySeqStEphS<T>;
        fn empty() -> Self;
        fn singleton(value: T) -> Self;
        fn from_seq(seq: &ArraySeqStEphS<T>) -> Self;
        fn filter<F: Pred<T>>(&self, predicate: F) -> Self;
        fn intersection(&self, other: &Self) -> Self;
        fn difference(&self, other: &Self) -> Self;
        fn union(&self, other: &Self) -> Self;
        fn find(&self, value: &T) -> B;
        fn delete(&mut self, value: &T);
        fn insert(&mut self, value: T);
    }

    impl<T: StT + Ord> AVLTreeSetStEph<T> {
        pub(crate) fn inorder_vec(&self) -> Vec<T> {
            let mut values = Vec::with_capacity(self.size());
            inorder_apply(&self.root, &mut |value| values.push(value.clone()));
            values
        }
    }

    impl<T: StT + Ord> AVLTreeSetStEphTrait<T> for AVLTreeSetStEph<T> {
        fn size(&self) -> N {
            subtree_size(&self.root)
        }

        fn to_seq(&self) -> ArraySeqStEphS<T> {
            ArraySeqStEphS::from_vec(self.inorder_vec())
        }

        fn empty() -> Self {
            AVLTreeSetStEph { root: None }
        }

        fn singleton(value: T) -> Self {
            AVLTreeSetStEph {
                root: Some(Box::new(Node::new(value))),
            }
        }

        fn from_seq(seq: &ArraySeqStEphS<T>) -> Self {
            let mut set = Self::empty();
            for index in 0..seq.length() {
                set.insert(seq.nth(index).clone());
            }
            set
        }

        fn filter<F: Pred<T>>(&self, predicate: F) -> Self
        {
            let mut result = Self::empty();
            filter_collect(&self.root, &predicate, &mut result);
            result
        }

        fn intersection(&self, other: &Self) -> Self {
            let mut result = Self::empty();
            if self.size() <= other.size() {
                intersection_collect(&self.root, other, &mut result);
            } else {
                intersection_collect(&other.root, self, &mut result);
            }
            result
        }

        fn difference(&self, other: &Self) -> Self {
            let mut result = Self::empty();
            difference_collect(&self.root, other, &mut result);
            result
        }

        fn union(&self, other: &Self) -> Self {
            let mut result = self.clone();
            union_collect(&other.root, &mut result);
            result
        }

        fn find(&self, value: &T) -> B {
            contains_link(&self.root, value)
        }

        fn delete(&mut self, value: &T) {
            let root = self.root.take();
            self.root = delete_link(root, value);
        }

        fn insert(&mut self, value: T) {
            let root = self.root.take();
            self.root = insert_link(root, value);
        }
    }
}

