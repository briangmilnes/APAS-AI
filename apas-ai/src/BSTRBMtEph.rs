//! Ephemeral Red-Black balanced binary search tree with interior locking for multi-threaded access.

pub mod BSTRBMtEph {
    use std::sync::{Arc, RwLock};

    use crate::ArraySeqStPer::ArraySeqStPer::*;
    use crate::ArraySeqStPerChap18::ArraySeqStPerChap18::*;
    use crate::Types::Types::*;

    #[derive(Clone, Copy, PartialEq, Eq)]
    enum Color {
        Red,
        Black,
    }

    type Link<T> = Option<Box<Node<T>>>;

    #[derive(Clone)]
    struct Node<T: StTInMtT + Ord> {
        key: T,
        color: Color,
        size: N,
        left: Link<T>,
        right: Link<T>,
    }

    impl<T: StTInMtT + Ord> Node<T> {
        fn new(key: T) -> Self {
            Node {
                key,
                color: Color::Red,
                size: 1,
                left: None,
                right: None,
            }
        }
    }

    #[derive(Clone)]
    pub struct BSTRBMtEph<T: StTInMtT + Ord> {
        root: Arc<RwLock<Link<T>>>,
    }

    pub type BSTreeRB<T> = BSTRBMtEph<T>;

    pub trait BSTRBMtEphTrait<T: StTInMtT + Ord>: Sized {
        fn new() -> Self;
        fn insert(&self, value: T);
        fn find(&self, target: &T) -> Option<T>;
        fn contains(&self, target: &T) -> B;
        fn size(&self) -> N;
        fn is_empty(&self) -> B;
        fn height(&self) -> N;
        fn minimum(&self) -> Option<T>;
        fn maximum(&self) -> Option<T>;
        fn in_order(&self) -> ArrayStPerS<T>;
        fn pre_order(&self) -> ArrayStPerS<T>;
    }

    impl<T: StTInMtT + Ord> Default for BSTRBMtEph<T> {
        fn default() -> Self { Self::new() }
    }

    impl<T: StTInMtT + Ord> BSTRBMtEph<T> {
        pub fn new() -> Self {
            BSTRBMtEph {
                root: Arc::new(RwLock::new(None)),
            }
        }

        pub fn size(&self) -> N {
            let guard = self.root.read().unwrap();
            Self::size_link(&*guard)
        }

        pub fn is_empty(&self) -> B {
            if self.size() == 0 {
                B::True
            } else {
                B::False
            }
        }

        pub fn height(&self) -> N {
            fn height_rec<T: StTInMtT + Ord>(link: &Link<T>) -> N {
                match link {
                    | None => 0,
                    | Some(node) => 1 + height_rec(&node.left).max(height_rec(&node.right)),
                }
            }

            let guard = self.root.read().unwrap();
            height_rec(&*guard)
        }

        pub fn insert(&self, value: T) {
            let mut guard = self.root.write().unwrap();
            Self::insert_link(&mut *guard, value);
            if let Some(node) = guard.as_mut() {
                node.color = Color::Black;
            }
        }

        pub fn find(&self, target: &T) -> Option<T> {
            let guard = self.root.read().unwrap();
            Self::find_link(&*guard, target).cloned()
        }

        pub fn contains(&self, target: &T) -> B {
            if self.find(target).is_some() {
                B::True
            } else {
                B::False
            }
        }

        pub fn minimum(&self) -> Option<T> {
            let guard = self.root.read().unwrap();
            Self::min_link(&*guard).cloned()
        }

        pub fn maximum(&self) -> Option<T> {
            let guard = self.root.read().unwrap();
            Self::max_link(&*guard).cloned()
        }

        pub fn in_order(&self) -> ArrayStPerS<T> {
            let guard = self.root.read().unwrap();
            let mut out = Vec::with_capacity(Self::size_link(&*guard));
            Self::in_order_collect(&*guard, &mut out);
            ArrayStPerS::from_vec(out)
        }

        pub fn pre_order(&self) -> ArrayStPerS<T> {
            let guard = self.root.read().unwrap();
            let mut out = Vec::with_capacity(Self::size_link(&*guard));
            Self::pre_order_collect(&*guard, &mut out);
            ArrayStPerS::from_vec(out)
        }

        fn is_red(link: &Link<T>) -> bool { matches!(link, Some(node) if node.color == Color::Red) }

        fn size_link(link: &Link<T>) -> N { link.as_ref().map_or(0, |n| n.size) }

        fn update(node: &mut Node<T>) { node.size = 1 + Self::size_link(&node.left) + Self::size_link(&node.right); }

        fn rotate_left(link: &mut Link<T>) {
            if let Some(mut h) = link.take() {
                if let Some(mut x) = h.right.take() {
                    h.right = x.left.take();
                    Self::update(&mut h);
                    x.color = h.color;
                    h.color = Color::Red;
                    x.left = Some(h);
                    if let Some(left) = x.left.as_mut() {
                        Self::update(left);
                    }
                    Self::update(&mut x);
                    *link = Some(x);
                } else {
                    *link = Some(h);
                }
            }
        }

        fn rotate_right(link: &mut Link<T>) {
            if let Some(mut h) = link.take() {
                if let Some(mut x) = h.left.take() {
                    h.left = x.right.take();
                    Self::update(&mut h);
                    x.color = h.color;
                    h.color = Color::Red;
                    x.right = Some(h);
                    if let Some(right) = x.right.as_mut() {
                        Self::update(right);
                    }
                    Self::update(&mut x);
                    *link = Some(x);
                } else {
                    *link = Some(h);
                }
            }
        }

        fn flip_colors(link: &mut Link<T>) {
            if let Some(node) = link.as_mut() {
                node.color = match node.color {
                    | Color::Red => Color::Black,
                    | Color::Black => Color::Red,
                };
                if let Some(left) = node.left.as_mut() {
                    left.color = match left.color {
                        | Color::Red => Color::Black,
                        | Color::Black => Color::Red,
                    };
                }
                if let Some(right) = node.right.as_mut() {
                    right.color = match right.color {
                        | Color::Red => Color::Black,
                        | Color::Black => Color::Red,
                    };
                }
            }
        }

        fn fix_up(link: &mut Link<T>) {
            let rotate_left_needed = match link {
                | Some(node) => Self::is_red(&node.right) && !Self::is_red(&node.left),
                | None => false,
            };
            if rotate_left_needed {
                Self::rotate_left(link);
            }

            let rotate_right_needed = match link {
                | Some(node) => {
                    if let Some(left) = node.left.as_ref() {
                        Self::is_red(&node.left) && Self::is_red(&left.left)
                    } else {
                        false
                    }
                }
                | None => false,
            };
            if rotate_right_needed {
                Self::rotate_right(link);
            }

            let flip_needed = match link {
                | Some(node) => Self::is_red(&node.left) && Self::is_red(&node.right),
                | None => false,
            };
            if flip_needed {
                Self::flip_colors(link);
            }

            if let Some(node) = link.as_mut() {
                Self::update(node);
            }
        }

        fn insert_link(link: &mut Link<T>, value: T) {
            if let Some(node) = link.as_mut() {
                if value < node.key {
                    Self::insert_link(&mut node.left, value);
                } else if value > node.key {
                    Self::insert_link(&mut node.right, value);
                } else {
                    return;
                }
            } else {
                *link = Some(Box::new(Node::new(value)));
                return;
            }
            Self::fix_up(link);
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

    impl<T: StTInMtT + Ord> BSTRBMtEphTrait<T> for BSTRBMtEph<T> {
        fn new() -> Self { BSTRBMtEph::new() }

        fn insert(&self, value: T) { BSTRBMtEph::insert(self, value) }

        fn find(&self, target: &T) -> Option<T> { BSTRBMtEph::find(self, target) }

        fn contains(&self, target: &T) -> B { BSTRBMtEph::contains(self, target) }

        fn size(&self) -> N { BSTRBMtEph::size(self) }

        fn is_empty(&self) -> B { BSTRBMtEph::is_empty(self) }

        fn height(&self) -> N { BSTRBMtEph::height(self) }

        fn minimum(&self) -> Option<T> { BSTRBMtEph::minimum(self) }

        fn maximum(&self) -> Option<T> { BSTRBMtEph::maximum(self) }

        fn in_order(&self) -> ArrayStPerS<T> { BSTRBMtEph::in_order(self) }

        fn pre_order(&self) -> ArrayStPerS<T> { BSTRBMtEph::pre_order(self) }
    }

    #[macro_export]
    macro_rules! BSTRBMtEphLit {
        () => {
            < $crate::BSTRBMtEph::BSTRBMtEph::BSTRBMtEph<_> as $crate::BSTRBMtEph::BSTRBMtEph::BSTRBMtEphTrait<_> >::new()
        };
        ( $( $x:expr ),* $(,)? ) => {{
            let __tree = < $crate::BSTRBMtEph::BSTRBMtEph::BSTRBMtEph<_> as $crate::BSTRBMtEph::BSTRBMtEph::BSTRBMtEphTrait<_> >::new();
            $( __tree.insert($x); )*
            __tree
        }};
    }

    #[allow(dead_code)]
    fn _BSTRBMtEphLit_type_checks() {
        let _ = BSTRBMtEphLit![1, 2, 3]; // non-empty infers (e.g., i32)
        let _: BSTRBMtEph<i32> = BSTRBMtEphLit![]; // empty form requires explicit type
    }
}
