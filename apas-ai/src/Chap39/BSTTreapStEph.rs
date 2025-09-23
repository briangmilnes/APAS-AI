//! Ephemeral Treap (randomized heap-ordered BST) with `find` support.

pub mod BSTTreapStEph {
    use crate::Chap19::ArraySeqStPer::ArraySeqStPer::*;
    use crate::Chap19::ArraySeqStPer::ArraySeqStPer::*;
    use crate::Types::Types::*;
    use rand::{rng, Rng};

    type Link<T> = Option<Box<Node<T>>>;

    #[derive(Clone)]
    struct Node<T: StT + Ord> {
        key: T,
        priority: u64,
        size: N,
        left: Link<T>,
        right: Link<T>,
    }

    impl<T: StT + Ord> Node<T> {
        fn new(key: T, priority: u64) -> Self {
            Node {
                key,
                priority,
                size: 1,
                left: None,
                right: None,
            }
        }
    }

    pub struct BSTTreapStEph<T: StT + Ord> {
        root: Link<T>,
    }

    pub type BSTreeTreap<T> = BSTTreapStEph<T>;

    pub trait BSTTreapStEphTrait<T: StT + Ord> {
        fn new() -> Self;
        fn size(&self) -> N;
        fn is_empty(&self) -> B;
        fn height(&self) -> N;
        fn insert(&mut self, value: T);
        fn find(&self, target: &T) -> Option<&T>;
        fn contains(&self, target: &T) -> B;
        fn minimum(&self) -> Option<&T>;
        fn maximum(&self) -> Option<&T>;
        fn in_order(&self) -> ArrayStPerS<T>;
        fn pre_order(&self) -> ArrayStPerS<T>;
    }

    impl<T: StT + Ord> Default for BSTTreapStEph<T> {
        fn default() -> Self { Self::new() }
    }

    impl<T: StT + Ord> BSTTreapStEph<T> {
        pub fn new() -> Self { BSTTreapStEph { root: None } }

        pub fn size(&self) -> N { Self::size_link(&self.root) }

        pub fn is_empty(&self) -> B {
            if self.size() == 0 {
                B::True
            } else {
                B::False
            }
        }

        pub fn height(&self) -> N {
            fn height_rec<T: StT + Ord>(link: &Link<T>) -> N {
                match link {
                    | None => 0,
                    | Some(node) => 1 + height_rec(&node.left).max(height_rec(&node.right)),
                }
            }
            height_rec(&self.root)
        }

        pub fn insert(&mut self, value: T) {
            let mut r = rng();
            Self::insert_link(&mut self.root, value, &mut r);
        }

        pub fn find(&self, target: &T) -> Option<&T> { Self::find_link(&self.root, target) }

        pub fn contains(&self, target: &T) -> B {
            if self.find(target).is_some() {
                B::True
            } else {
                B::False
            }
        }

        pub fn minimum(&self) -> Option<&T> { Self::min_link(&self.root) }

        pub fn maximum(&self) -> Option<&T> { Self::max_link(&self.root) }

        pub fn in_order(&self) -> ArrayStPerS<T> {
            let mut out = Vec::with_capacity(self.size());
            Self::in_order_collect(&self.root, &mut out);
            ArrayStPerS::from_vec(out)
        }

        pub fn pre_order(&self) -> ArrayStPerS<T> {
            let mut out = Vec::with_capacity(self.size());
            Self::pre_order_collect(&self.root, &mut out);
            ArrayStPerS::from_vec(out)
        }

        fn size_link(link: &Link<T>) -> N { link.as_ref().map_or(0, |n| n.size) }

        fn update(node: &mut Node<T>) { node.size = 1 + Self::size_link(&node.left) + Self::size_link(&node.right); }

        fn rotate_left(link: &mut Link<T>) {
            if let Some(mut x) = link.take() {
                if let Some(mut y) = x.right.take() {
                    x.right = y.left.take();
                    Self::update(&mut x);
                    Self::update(&mut y);
                    y.left = Some(x);
                    *link = Some(y);
                } else {
                    *link = Some(x);
                }
            }
        }

        fn rotate_right(link: &mut Link<T>) {
            if let Some(mut x) = link.take() {
                if let Some(mut y) = x.left.take() {
                    x.left = y.right.take();
                    Self::update(&mut x);
                    Self::update(&mut y);
                    y.right = Some(x);
                    *link = Some(y);
                } else {
                    *link = Some(x);
                }
            }
        }

        fn insert_link(link: &mut Link<T>, value: T, rng: &mut impl Rng) {
            if let Some(node) = link.as_mut() {
                if value < node.key {
                    Self::insert_link(&mut node.left, value, rng);
                    if node.left.as_ref().map_or(false, |left| left.priority < node.priority) {
                        Self::rotate_right(link);
                    }
                } else if value > node.key {
                    Self::insert_link(&mut node.right, value, rng);
                    if node
                        .right
                        .as_ref()
                        .map_or(false, |right| right.priority < node.priority)
                    {
                        Self::rotate_left(link);
                    }
                }
                if let Some(node) = link.as_mut() {
                    Self::update(node);
                }
            } else {
                *link = Some(Box::new(Node::new(value, rng.random())));
            }
        }

        fn find_link<'a>(link: &'a Link<T>, target: &T) -> Option<&'a T> {
            match link {
                | None => None,
                | Some(node) => {
                    if target == &node.key {
                        Some(&node.key)
                    } else if target < &node.key {
                        Self::find_link(&node.left, target)
                    } else {
                        Self::find_link(&node.right, target)
                    }
                }
            }
        }

        fn min_link<'a>(link: &'a Link<T>) -> Option<&'a T> {
            match link {
                | None => None,
                | Some(node) => match node.left {
                    | None => Some(&node.key),
                    | Some(_) => Self::min_link(&node.left),
                },
            }
        }

        fn max_link<'a>(link: &'a Link<T>) -> Option<&'a T> {
            match link {
                | None => None,
                | Some(node) => match node.right {
                    | None => Some(&node.key),
                    | Some(_) => Self::max_link(&node.right),
                },
            }
        }

        fn in_order_collect(link: &Link<T>, out: &mut Vec<T>) {
            if let Some(node) = link {
                Self::in_order_collect(&node.left, out);
                out.push(node.key.clone());
                Self::in_order_collect(&node.right, out);
            }
        }

        fn pre_order_collect(link: &Link<T>, out: &mut Vec<T>) {
            if let Some(node) = link {
                out.push(node.key.clone());
                Self::pre_order_collect(&node.left, out);
                Self::pre_order_collect(&node.right, out);
            }
        }
    }

    impl<T: StT + Ord> BSTTreapStEphTrait<T> for BSTTreapStEph<T> {
        fn new() -> Self { BSTTreapStEph::new() }

        fn size(&self) -> N { BSTTreapStEph::size(self) }

        fn is_empty(&self) -> B { BSTTreapStEph::is_empty(self) }

        fn height(&self) -> N { BSTTreapStEph::height(self) }

        fn insert(&mut self, value: T) { BSTTreapStEph::insert(self, value) }

        fn find(&self, target: &T) -> Option<&T> { BSTTreapStEph::find(self, target) }

        fn contains(&self, target: &T) -> B { BSTTreapStEph::contains(self, target) }

        fn minimum(&self) -> Option<&T> { BSTTreapStEph::minimum(self) }

        fn maximum(&self) -> Option<&T> { BSTTreapStEph::maximum(self) }

        fn in_order(&self) -> ArrayStPerS<T> { BSTTreapStEph::in_order(self) }

        fn pre_order(&self) -> ArrayStPerS<T> { BSTTreapStEph::pre_order(self) }
    }

    #[macro_export]
    macro_rules! BSTTreapStEphLit {
        () => {
            < $crate::BSTTreapStEph::BSTTreapStEph::BSTTreapStEph<_> as $crate::BSTTreapStEph::BSTTreapStEph::BSTTreapStEphTrait<_> >::new()
        };
        ( $( $x:expr ),* $(,)? ) => {{
            let mut __tree = < $crate::BSTTreapStEph::BSTTreapStEph::BSTTreapStEph<_> as $crate::BSTTreapStEph::BSTTreapStEph::BSTTreapStEphTrait<_> >::new();
            $( __tree.insert($x); )*
            __tree
        }};
    }

    #[allow(dead_code)]
    fn _BSTTreapStEphLit_type_checks() {
        let _ = BSTTreapStEphLit![1, 2, 3]; // non-empty infers (e.g., i32)
        let _: BSTTreapStEph<i32> = BSTTreapStEphLit![]; // empty form requires explicit type
    }
}
