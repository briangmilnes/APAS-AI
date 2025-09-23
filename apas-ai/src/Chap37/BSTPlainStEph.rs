//! Ephemeral binary search tree built on `BBTEph` primitives.

pub mod BSTPlainStEph {
    use crate::Chap19::ArraySeqStPer::ArraySeqStPer::*;
    use crate::BBTEph::BBTEph::BBTree;
    use crate::Types::Types::*;

    pub struct BSTPlainStEph<T: StT + Ord> {
        root: BBTree<T>,
    }

    pub type BSTree<T> = BSTPlainStEph<T>;

    pub trait BSTPlainStEphTrait<T: StT + Ord> {
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

    impl<T: StT + Ord> BSTPlainStEph<T> {
        pub fn new() -> Self { BSTPlainStEph { root: BBTree::leaf() } }

        pub fn size(&self) -> N { self.root.size() }

        pub fn is_empty(&self) -> B { self.root.is_leaf() }

        pub fn height(&self) -> N { self.root.height() }

        pub fn insert(&mut self, value: T) { insert_node(&mut self.root, value); }

        pub fn find(&self, target: &T) -> Option<&T> { find_node(&self.root, target) }

        pub fn contains(&self, target: &T) -> B { contains_node(&self.root, target) }

        pub fn minimum(&self) -> Option<&T> { min_node(&self.root) }

        pub fn maximum(&self) -> Option<&T> { max_node(&self.root) }

        pub fn in_order(&self) -> ArrayStPerS<T> { self.root.in_order() }

        pub fn pre_order(&self) -> ArrayStPerS<T> { self.root.pre_order() }
    }

    impl<T: StT + Ord> BSTPlainStEphTrait<T> for BSTPlainStEph<T> {
        fn new() -> Self { BSTPlainStEph::new() }

        fn size(&self) -> N { BSTPlainStEph::size(self) }

        fn is_empty(&self) -> B { BSTPlainStEph::is_empty(self) }

        fn height(&self) -> N { BSTPlainStEph::height(self) }

        fn insert(&mut self, value: T) { BSTPlainStEph::insert(self, value) }

        fn find(&self, target: &T) -> Option<&T> { BSTPlainStEph::find(self, target) }

        fn contains(&self, target: &T) -> B { BSTPlainStEph::contains(self, target) }

        fn minimum(&self) -> Option<&T> { BSTPlainStEph::minimum(self) }

        fn maximum(&self) -> Option<&T> { BSTPlainStEph::maximum(self) }

        fn in_order(&self) -> ArrayStPerS<T> { BSTPlainStEph::in_order(self) }

        fn pre_order(&self) -> ArrayStPerS<T> { BSTPlainStEph::pre_order(self) }
    }

    fn insert_node<T: StT + Ord>(node: &mut BBTree<T>, value: T) {
        match node {
            | BBTree::Leaf => {
                *node = BBTree::node(BBTree::leaf(), value, BBTree::leaf());
            }
            | BBTree::Node(inner) => {
                if value < inner.value {
                    insert_node(&mut inner.left, value);
                } else if value > inner.value {
                    insert_node(&mut inner.right, value);
                }
            }
        }
    }

    fn contains_node<'a, T: StT + Ord>(node: &'a BBTree<T>, target: &T) -> B {
        match node {
            | BBTree::Leaf => B::False,
            | BBTree::Node(inner) => {
                if target == &inner.value {
                    B::True
                } else if target < &inner.value {
                    contains_node(&inner.left, target)
                } else {
                    contains_node(&inner.right, target)
                }
            }
        }
    }

    fn find_node<'a, T: StT + Ord>(node: &'a BBTree<T>, target: &T) -> Option<&'a T> {
        match node {
            | BBTree::Leaf => None,
            | BBTree::Node(inner) => {
                if target == &inner.value {
                    Some(&inner.value)
                } else if target < &inner.value {
                    find_node(&inner.left, target)
                } else {
                    find_node(&inner.right, target)
                }
            }
        }
    }

    fn min_node<'a, T: StT + Ord>(node: &'a BBTree<T>) -> Option<&'a T> {
        match node {
            | BBTree::Leaf => None,
            | BBTree::Node(inner) => match &inner.left {
                | BBTree::Leaf => Some(&inner.value),
                | _ => min_node(&inner.left),
            },
        }
    }

    fn max_node<'a, T: StT + Ord>(node: &'a BBTree<T>) -> Option<&'a T> {
        match node {
            | BBTree::Leaf => None,
            | BBTree::Node(inner) => match &inner.right {
                | BBTree::Leaf => Some(&inner.value),
                | _ => max_node(&inner.right),
            },
        }
    }

    #[macro_export]
    macro_rules! BSTPlainStEphLit {
        () => { $crate::Chap37::BSTPlainStEph::BSTPlainStEph::BSTPlainStEph::new() };
        ($x:expr; $n:expr) => {{
            let mut __tree = $crate::Chap37::BSTPlainStEph::BSTPlainStEph::BSTPlainStEph::new();
            for _ in 0..$n { __tree.insert($x.clone()); }
            __tree
        }};
        ($($x:expr),+ $(,)?) => {{
            let mut __tree = $crate::Chap37::BSTPlainStEph::BSTPlainStEph::BSTPlainStEph::new();
            $( __tree.insert($x); )*
            __tree
        }};
    }

    #[allow(dead_code)]
    fn _BSTPlainStEphLit_type_checks() {
        use crate::Chap37::BSTPlainStEph::BSTPlainStEph::BSTPlainStEph;
        let _: BSTPlainStEph<i32> = BSTPlainStEphLit![];
        let _ = BSTPlainStEphLit![1, 2, 3];
        let _ = BSTPlainStEphLit![0; 2];
    }
}
