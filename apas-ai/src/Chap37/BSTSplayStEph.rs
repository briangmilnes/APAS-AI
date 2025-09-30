//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Ephemeral Splay Tree placeholder (standard BST semantics) with public methods.

pub mod BSTSplayStEph {
    use crate::Chap19::ArraySeqStPer::ArraySeqStPer::*;
    use crate::Chap19::ArraySeqStPer::ArraySeqStPer::*;
    use crate::Types::Types::*;

    type Link<T> = Option<Box<Node<T>>>;

    #[derive(Clone, Debug)]
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
    pub struct BSTSplayStEph<T: StT + Ord> {
        root: Link<T>,
    }

    pub type BSTreeSplay<T> = BSTSplayStEph<T>;

    pub trait BSTSplayStEphTrait<T: StT + Ord> {
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

    impl<T: StT + Ord> Default for BSTSplayStEph<T> {
        fn default() -> Self {
            Self::new()
        }
    }

    impl<T: StT + Ord> BSTSplayStEph<T> {
        // Private helper methods only - no public delegation

        fn size_link(link: &Link<T>) -> N {
            link.as_ref().map_or(0, |n| n.size)
        }

        fn update(node: &mut Node<T>) {
            node.size = 1 + Self::size_link(&node.left) + Self::size_link(&node.right);
        }

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

    impl<T: StT + Ord> BSTSplayStEphTrait<T> for BSTSplayStEph<T> {
        fn new() -> Self {
            BSTSplayStEph { root: None }
        }

        fn size(&self) -> N {
            Self::size_link(&self.root)
        }

        fn is_empty(&self) -> B {
            if self.size() == 0 { true } else { false }
        }

        fn height(&self) -> N {
            fn height_rec<T: StT + Ord>(link: &Link<T>) -> N {
                match link {
                    | None => 0,
                    | Some(node) => 1 + height_rec(&node.left).max(height_rec(&node.right)),
                }
            }
            height_rec(&self.root)
        }

        fn insert(&mut self, value: T) {
            Self::insert_link(&mut self.root, value);
        }

        fn find(&self, target: &T) -> Option<&T> {
            Self::find_link(&self.root, target)
        }

        fn contains(&self, target: &T) -> B {
            self.find(target).is_some()
        }

        fn minimum(&self) -> Option<&T> {
            Self::min_link(&self.root)
        }

        fn maximum(&self) -> Option<&T> {
            Self::max_link(&self.root)
        }

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
    macro_rules! BSTSplayStEphLit {
        () => {
            < $crate::Chap37::BSTSplayStEph::BSTSplayStEph::BSTSplayStEph<_> as $crate::Chap37::BSTSplayStEph::BSTSplayStEph::BSTSplayStEphTrait<_> >::new()
        };
        ( $( $x:expr ),* $(,)? ) => {{
            let mut __tree = < $crate::Chap37::BSTSplayStEph::BSTSplayStEph::BSTSplayStEph<_> as $crate::Chap37::BSTSplayStEph::BSTSplayStEph::BSTSplayStEphTrait<_> >::new();
            $( __tree.insert($x); )*
            __tree
        }};
    }

    #[allow(dead_code)]
    fn _BSTSplayStEphLit_type_checks() {
        let _ = BSTSplayStEphLit![1, 2, 3]; // non-empty infers (e.g., i32)
        let _: BSTSplayStEph<i32> = BSTSplayStEphLit![]; // empty form requires explicit type
    }
}
