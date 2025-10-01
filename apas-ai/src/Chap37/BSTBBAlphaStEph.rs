//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Ephemeral weight-balanced (BB[α]) binary search tree with `find` support.

pub mod BSTBBAlphaStEph {
    use crate::Chap19::ArraySeqStPer::ArraySeqStPer::*;
    use crate::Chap19::ArraySeqStPer::ArraySeqStPer::*;
    use crate::Types::Types::*;

    const ALPHA: f64 = 0.75;

    type Link<T> = Option<Box<Node<T>>>;

    #[derive(Debug, Clone)]
    struct Node<T: StT + Ord> {
        key: T,
        size: N,
        left: Link<T>,
        right: Link<T>,
    }

    impl<T: StT + Ord> Node<T> {
        fn new(key: T) -> Self {
            Node {
                key,
                size: 1,
                left: None,
                right: None,
            }
        }
    }

    #[derive(Debug, Clone)]
    pub struct BSTBBAlphaStEph<T: StT + Ord> {
        root: Link<T>,
    }

    pub type BSTreeBBAlpha<T> = BSTBBAlphaStEph<T>;

    pub trait BSTBBAlphaStEphTrait<T: StT + Ord> {
        fn new() -> Self;
        fn size(&self) -> N;
        fn is_empty(&self) -> B;
        fn height(&self) -> N;
        fn insert(&mut self, value: T);
        fn find(&self, target: &T) -> Option<&T>;
        fn contains(&self, target: &T) -> B;
        fn minimum(&self) -> Option<&T>;
        fn maximum(&self) -> Option<&T>;
        fn in_order(&self) -> ArraySeqStPerS<T>;
        fn pre_order(&self) -> ArraySeqStPerS<T>;
    }

    impl<T: StT + Ord> Default for BSTBBAlphaStEph<T> {
        fn default() -> Self { Self::new() }
    }

    impl<T: StT + Ord> BSTBBAlphaStEph<T> {
        // Private helper methods only - no public delegation

        pub fn is_empty(&self) -> B { if self.size() == 0 { true } else { false } }

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
            let inserted = Self::insert_link(&mut self.root, value);
            if inserted {
                let total = Self::size_link(&self.root);
                Self::rebalance_if_needed(&mut self.root, total);
            }
        }

        pub fn find(&self, target: &T) -> Option<&T> { Self::find_link(&self.root, target) }

        pub fn contains(&self, target: &T) -> B { if self.find(target).is_some() { true } else { false } }

        pub fn minimum(&self) -> Option<&T> { Self::min_link(&self.root) }

        pub fn maximum(&self) -> Option<&T> { Self::max_link(&self.root) }

        pub fn in_order(&self) -> ArraySeqStPerS<T> {
            let mut out = Vec::with_capacity(self.size());
            Self::in_order_collect(&self.root, &mut out);
            ArraySeqStPerS::from_vec(out)
        }

        pub fn pre_order(&self) -> ArraySeqStPerS<T> {
            let mut out = Vec::with_capacity(self.size());
            Self::pre_order_collect(&self.root, &mut out);
            ArraySeqStPerS::from_vec(out)
        }

        fn size_link(link: &Link<T>) -> N { link.as_ref().map_or(0, |n| n.size) }

        fn update(node: &mut Node<T>) { node.size = 1 + Self::size_link(&node.left) + Self::size_link(&node.right); }

        fn insert_link(link: &mut Link<T>, value: T) -> bool {
            match link {
                | Some(node) => {
                    let inserted = if value < node.key {
                        Self::insert_link(&mut node.left, value)
                    } else if value > node.key {
                        Self::insert_link(&mut node.right, value)
                    } else {
                        false
                    };
                    if inserted {
                        Self::update(node);
                    }
                    inserted
                }
                | None => {
                    *link = Some(Box::new(Node::new(value)));
                    true
                }
            }
        }

        fn needs_rebuild(node: &Node<T>) -> bool {
            let total = node.size as f64;
            let left = Self::size_link(&node.left) as f64;
            let right = Self::size_link(&node.right) as f64;
            left > ALPHA * total || right > ALPHA * total
        }

        fn rebalance_if_needed(link: &mut Link<T>, total_size: N) {
            if let Some(node) = link.as_ref() {
                if Self::needs_rebuild(node) {
                    let mut values = Vec::with_capacity(total_size);
                    Self::collect_values(&Some(node.clone()), &mut values);
                    *link = Self::build_balanced(&values);
                }
            }
        }

        fn collect_values(link: &Link<T>, out: &mut Vec<T>) {
            if let Some(node) = link {
                Self::collect_values(&node.left, out);
                out.push(node.key.clone());
                Self::collect_values(&node.right, out);
            }
        }

        fn build_balanced(values: &[T]) -> Link<T> {
            if values.is_empty() {
                return None;
            }
            let mid = values.len() / 2;
            let mut node = Box::new(Node::new(values[mid].clone()));
            node.left = Self::build_balanced(&values[..mid]);
            node.right = Self::build_balanced(&values[mid + 1..]);
            Self::update(&mut node);
            Some(node)
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

    impl<T: StT + Ord> BSTBBAlphaStEphTrait<T> for BSTBBAlphaStEph<T> {
        fn new() -> Self { BSTBBAlphaStEph { root: None } }

        fn size(&self) -> N { Self::size_link(&self.root) }

        fn is_empty(&self) -> B { if self.size() == 0 { true } else { false } }

        fn height(&self) -> N {
            fn height_rec<T: StT + Ord>(link: &Link<T>) -> N {
                match link {
                    | None => 0,
                    | Some(node) => 1 + height_rec(&node.left).max(height_rec(&node.right)),
                }
            }
            height_rec(&self.root)
        }

        fn insert(&mut self, value: T) { Self::insert_link(&mut self.root, value); }

        fn find(&self, target: &T) -> Option<&T> { Self::find_link(&self.root, target) }

        fn contains(&self, target: &T) -> B { self.find(target).is_some() }

        fn minimum(&self) -> Option<&T> { Self::min_link(&self.root) }

        fn maximum(&self) -> Option<&T> { Self::max_link(&self.root) }

        fn in_order(&self) -> ArraySeqStPerS<T> {
            let mut out = Vec::with_capacity(self.size());
            Self::in_order_collect(&self.root, &mut out);
            ArraySeqStPerS::from_vec(out)
        }

        fn pre_order(&self) -> ArraySeqStPerS<T> {
            let mut out = Vec::with_capacity(self.size());
            Self::pre_order_collect(&self.root, &mut out);
            ArraySeqStPerS::from_vec(out)
        }
    }

    #[macro_export]
    macro_rules! BSTBBAlphaStEphLit {
        () => {
            < $crate::Chap37::BSTBBAlphaStEph::BSTBBAlphaStEph::BSTBBAlphaStEph<_> as $crate::Chap37::BSTBBAlphaStEph::BSTBBAlphaStEph::BSTBBAlphaStEphTrait<_> >::new()
        };
        ( $( $x:expr ),* $(,)? ) => {{
            let mut __tree = < $crate::Chap37::BSTBBAlphaStEph::BSTBBAlphaStEph::BSTBBAlphaStEph<_> as $crate::Chap37::BSTBBAlphaStEph::BSTBBAlphaStEph::BSTBBAlphaStEphTrait<_> >::new();
            $( __tree.insert($x); )*
            __tree
        }};
    }

    #[allow(dead_code)]
    fn _BSTBBAlphaStEphLit_type_checks() {
        let _ = BSTBBAlphaStEphLit![1, 2, 3]; // non-empty infers (e.g., i32)
        let _: BSTBBAlphaStEph<i32> = BSTBBAlphaStEphLit![]; // empty form requires explicit type
    }
}
