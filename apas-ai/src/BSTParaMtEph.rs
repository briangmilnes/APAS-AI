//! Parametric multi-threaded BST built around a joinMid interface.

pub mod BSTParaMtEph {
    use std::sync::{Arc, RwLock};

    use crate::ArraySeqStPer::ArraySeqStPer::*;
    use crate::Types::Types::*;

    #[derive(Clone)]
    pub enum Exposed<T: StTinMtT + Ord> {
        Leaf,
        Node(ParamBST<T>, T, ParamBST<T>),
    }

    #[derive(Clone)]
    struct NodeInner<T: StTinMtT + Ord> {
        key: T,
        size: N,
        left: ParamBST<T>,
        right: ParamBST<T>,
    }

    #[derive(Clone)]
    pub struct ParamBST<T: StTinMtT + Ord> {
        root: Arc<RwLock<Option<Box<NodeInner<T>>>>>,
    }

    pub trait ParamBSTTrait<T: StTinMtT + Ord>: Sized {
        fn new() -> Self;
        fn expose(&self) -> Exposed<T>;
        fn join_mid(exposed: Exposed<T>) -> Self;
        fn size(&self) -> N;
        fn is_empty(&self) -> B;
        fn insert(&self, key: T);
        fn delete(&self, key: &T);
        fn find(&self, key: &T) -> Option<T>;
        fn split(&self, key: &T) -> (Self, B, Self);
        fn join_pair(&self, other: Self) -> Self;
        fn union(&self, other: &Self) -> Self;
        fn in_order(&self) -> ArrayStPerS<T>;
    }

    impl<T: StTinMtT + Ord> ParamBST<T> {
        fn expose_internal(&self) -> Exposed<T> {
            let guard = self.root.read().unwrap();
            match &*guard {
                | None => Exposed::Leaf,
                | Some(node) => Exposed::Node(node.left.clone(), node.key.clone(), node.right.clone()),
            }
        }

        fn join_mid(exposed: Exposed<T>) -> Self {
            match exposed {
                | Exposed::Leaf => ParamBST::new(),
                | Exposed::Node(left, key, right) => {
                    let size = 1 + left.size() + right.size();
                    ParamBST {
                        root: Arc::new(RwLock::new(Some(Box::new(NodeInner { key, size, left, right })))),
                    }
                }
            }
        }

        fn split_inner(tree: &Self, key: &T) -> (Self, B, Self) {
            match tree.expose_internal() {
                | Exposed::Leaf => (ParamBST::new(), B::False, ParamBST::new()),
                | Exposed::Node(left, root_key, right) => match key.cmp(&root_key) {
                    | std::cmp::Ordering::Less => {
                        let (ll, found, lr) = ParamBST::split_inner(&left, key);
                        let rebuilt = ParamBST::join_mid(Exposed::Node(lr, root_key, right));
                        (ll, found, rebuilt)
                    }
                    | std::cmp::Ordering::Greater => {
                        let (rl, found, rr) = ParamBST::split_inner(&right, key);
                        let rebuilt = ParamBST::join_mid(Exposed::Node(left, root_key, rl));
                        (rebuilt, found, rr)
                    }
                    | std::cmp::Ordering::Equal => (left, B::True, right),
                },
            }
        }

        fn join_m(left: Self, key: T, right: Self) -> Self { ParamBST::join_mid(Exposed::Node(left, key, right)) }

        fn min_key(tree: &Self) -> Option<T> {
            match tree.expose_internal() {
                | Exposed::Leaf => None,
                | Exposed::Node(left, key, _) => match ParamBST::min_key(&left) {
                    | Some(rec) => Some(rec),
                    | None => Some(key),
                },
            }
        }

        fn join_pair_inner(left: Self, right: Self) -> Self {
            match right.expose_internal() {
                | Exposed::Leaf => left,
                | Exposed::Node(_, key, _) => {
                    let min_key = ParamBST::min_key(&right).unwrap_or(key);
                    let (_, _, reduced_right) = ParamBST::split_inner(&right, &min_key);
                    ParamBST::join_m(left, min_key, reduced_right)
                }
            }
        }

        fn union_inner(a: &Self, b: &Self) -> Self {
            match a.expose_internal() {
                | Exposed::Leaf => b.clone(),
                | Exposed::Node(al, ak, ar) => {
                    let (bl, _, br) = ParamBST::split_inner(b, &ak);
                    let left_union = ParamBST::union_inner(&al, &bl);
                    let right_union = ParamBST::union_inner(&ar, &br);
                    ParamBST::join_m(left_union, ak, right_union)
                }
            }
        }

        fn collect_in_order(tree: &Self, out: &mut Vec<T>) {
            match tree.expose_internal() {
                | Exposed::Leaf => {}
                | Exposed::Node(left, key, right) => {
                    ParamBST::collect_in_order(&left, out);
                    out.push(key);
                    ParamBST::collect_in_order(&right, out);
                }
            }
        }
    }

    impl<T: StTinMtT + Ord> ParamBSTTrait<T> for ParamBST<T> {
        fn new() -> Self {
            ParamBST {
                root: Arc::new(RwLock::new(None)),
            }
        }

        fn expose(&self) -> Exposed<T> { self.expose_internal() }

        fn join_mid(exposed: Exposed<T>) -> Self { ParamBST::join_mid(exposed) }

        fn size(&self) -> N {
            let guard = self.root.read().unwrap();
            guard.as_ref().map_or(0, |node| node.size)
        }

        fn is_empty(&self) -> B {
            if self.size() == 0 {
                B::True
            } else {
                B::False
            }
        }

        fn insert(&self, key: T) {
            let (left, _, right) = ParamBST::split_inner(self, &key);
            let rebuilt = ParamBST::join_m(left, key, right);
            let new_state = rebuilt.root.read().unwrap().clone();
            let mut guard = self.root.write().unwrap();
            *guard = new_state;
        }

        fn delete(&self, key: &T) {
            let (left, _, right) = ParamBST::split_inner(self, key);
            let merged = ParamBST::join_pair_inner(left, right);
            let new_state = merged.root.read().unwrap().clone();
            let mut guard = self.root.write().unwrap();
            *guard = new_state;
        }

        fn find(&self, key: &T) -> Option<T> {
            match self.expose_internal() {
                | Exposed::Leaf => None,
                | Exposed::Node(left, root_key, right) => match key.cmp(&root_key) {
                    | std::cmp::Ordering::Less => ParamBSTTrait::find(&left, key),
                    | std::cmp::Ordering::Greater => ParamBSTTrait::find(&right, key),
                    | std::cmp::Ordering::Equal => Some(root_key),
                },
            }
        }

        fn split(&self, key: &T) -> (Self, B, Self) { ParamBST::split_inner(self, key) }

        fn join_pair(&self, other: Self) -> Self { ParamBST::join_pair_inner(self.clone(), other) }

        fn union(&self, other: &Self) -> Self { ParamBST::union_inner(self, other) }

        fn in_order(&self) -> ArrayStPerS<T> {
            let mut out = Vec::with_capacity(self.size());
            ParamBST::collect_in_order(self, &mut out);
            ArrayStPerS::from_vec(out)
        }
    }
}
