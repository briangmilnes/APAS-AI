//! Ephemeral Red-Black balanced binary search tree with `find` support and public helpers.

pub mod BSTRBStEph {
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
    struct Node<T: StT + Ord> {
        key: T,
        color: Color,
        size: N,
        left: Link<T>,
        right: Link<T>,
    }

    impl<T: StT + Ord> Node<T> {
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

    pub struct BSTRBStEph<T: StT + Ord> {
        root: Link<T>,
    }

    pub type BSTreeRB<T> = BSTRBStEph<T>;

    pub trait BSTRBStEphTrait<T: StT + Ord> {
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

    impl<T: StT + Ord> Default for BSTRBStEph<T> {
        fn default() -> Self { Self::new() }
    }

    impl<T: StT + Ord> BSTRBStEph<T> {
        pub fn new() -> Self { BSTRBStEph { root: None } }

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
            Self::insert_link(&mut self.root, value);
            if let Some(node) = self.root.as_mut() {
                node.color = Color::Black;
            }
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
                    Self::update(x.left.as_mut().unwrap());
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
                    Self::update(x.right.as_mut().unwrap());
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
            if Self::is_red(&link.as_ref().unwrap().right) && !Self::is_red(&link.as_ref().unwrap().left) {
                Self::rotate_left(link);
            }
            if Self::is_red(&link.as_ref().unwrap().left)
                && Self::is_red(&link.as_ref().unwrap().left.as_ref().unwrap().left)
            {
                Self::rotate_right(link);
            }
            if Self::is_red(&link.as_ref().unwrap().left) && Self::is_red(&link.as_ref().unwrap().right) {
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

    impl<T: StT + Ord> BSTRBStEphTrait<T> for BSTRBStEph<T> {
        fn new() -> Self { BSTRBStEph::new() }

        fn size(&self) -> N { BSTRBStEph::size(self) }

        fn is_empty(&self) -> B { BSTRBStEph::is_empty(self) }

        fn height(&self) -> N { BSTRBStEph::height(self) }

        fn insert(&mut self, value: T) { BSTRBStEph::insert(self, value) }

        fn find(&self, target: &T) -> Option<&T> { BSTRBStEph::find(self, target) }

        fn contains(&self, target: &T) -> B { BSTRBStEph::contains(self, target) }

        fn minimum(&self) -> Option<&T> { BSTRBStEph::minimum(self) }

        fn maximum(&self) -> Option<&T> { BSTRBStEph::maximum(self) }

        fn in_order(&self) -> ArrayStPerS<T> { BSTRBStEph::in_order(self) }

        fn pre_order(&self) -> ArrayStPerS<T> { BSTRBStEph::pre_order(self) }
    }

    #[macro_export]
    macro_rules! BSTRBStEphLit {
        () => {
            < $crate::BSTRBStEph::BSTRBStEph::BSTRBStEph<_> as $crate::BSTRBStEph::BSTRBStEph::BSTRBStEphTrait<_> >::new()
        };
        ( $( $x:expr ),* $(,)? ) => {{
            let mut __tree = < $crate::BSTRBStEph::BSTRBStEph::BSTRBStEph<_> as $crate::BSTRBStEph::BSTRBStEph::BSTRBStEphTrait<_> >::new();
            $( __tree.insert($x); )*
            __tree
        }};
    }

    #[allow(dead_code)]
    fn _BSTRBStEphLit_type_checks() {
        let _ = BSTRBStEphLit![1, 2, 3]; // non-empty infers (e.g., i32)
        let _: BSTRBStEph<i32> = BSTRBStEphLit![]; // empty form requires explicit type
    }
}
