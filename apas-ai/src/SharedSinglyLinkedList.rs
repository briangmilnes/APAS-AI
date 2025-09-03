//! Persistent singly linked list with shared tails (Lisp-style) using Rc.

use std::rc::Rc;
use crate::Types::{B};

pub struct Node<T> {
    pub value: T,
    pub next: Option<Rc<Node<T>>>,
}

pub struct SharedSinglyLinkedList<T> {
    pub(crate) head: Option<Rc<Node<T>>>,
}

pub trait SharedSList<T> {
    /// ChatGPT-5-hard: Work Θ(1), Span Θ(1).
    fn empty() -> SharedSinglyLinkedList<T>;
    /// ChatGPT-5-hard: Work Θ(1), Span Θ(1).
    fn isEmpty(&self) -> B;
    /// ChatGPT-5-hard: Work Θ(1), Span Θ(1).
    fn cons(x: T, tail: &SharedSinglyLinkedList<T>) -> SharedSinglyLinkedList<T>;
    /// ChatGPT-5-hard: Work Θ(1), Span Θ(1).
    fn head(&self) -> Option<&T>;
    /// ChatGPT-5-hard: Work Θ(1), Span Θ(1).
    fn tail(&self) -> SharedSinglyLinkedList<T>;
    /// ChatGPT-5-hard: Work Θ(1), Span Θ(1).
    fn cdr(&self) -> SharedSinglyLinkedList<T> { self.tail() }

    /// APAS 20.7 (map f a): Work Θ(1 + Σ x∈a W(f(x))), Span Θ(1 + Σ x∈a S(f(x))).
    fn map<U>(a: &SharedSinglyLinkedList<T>, f: impl Fn(&T) -> U) -> SharedSinglyLinkedList<U>;

    /// APAS 20.7 (append a b): Work Θ(1 + |a|), Span Θ(1 + |a|).
    fn append(a: &SharedSinglyLinkedList<T>, b: &SharedSinglyLinkedList<T>) -> SharedSinglyLinkedList<T>
    where
        T: Clone;
}

impl<T> SharedSList<T> for SharedSinglyLinkedList<T> {
    fn empty() -> SharedSinglyLinkedList<T> { SharedSinglyLinkedList { head: None } }
    fn isEmpty(&self) -> B { if self.head.is_none() { B::True } else { B::False } }
    fn cons(x: T, tail: &SharedSinglyLinkedList<T>) -> SharedSinglyLinkedList<T> {
        SharedSinglyLinkedList { head: Some(Rc::new(Node { value: x, next: tail.head.clone() })) }
    }
    fn head(&self) -> Option<&T> { self.head.as_deref().map(|n| &n.value) }
    fn tail(&self) -> SharedSinglyLinkedList<T> {
        SharedSinglyLinkedList { head: self.head.as_ref().and_then(|n| n.next.clone()) }
    }

    fn map<U>(a: &SharedSinglyLinkedList<T>, f: impl Fn(&T) -> U) -> SharedSinglyLinkedList<U> {
        fn map_nodes<T, U, F: Fn(&T) -> U>(n: Option<Rc<Node<T>>>, f: &F) -> Option<Rc<Node<U>>> {
            match n {
                None => None,
                Some(rc) => {
                    let mapped = f(&rc.value);
                    let mapped_tail = map_nodes(rc.next.clone(), f);
                    Some(Rc::new(Node { value: mapped, next: mapped_tail }))
                }
            }
        }
        SharedSinglyLinkedList { head: map_nodes(a.head.clone(), &f) }
    }

    fn append(a: &SharedSinglyLinkedList<T>, b: &SharedSinglyLinkedList<T>) -> SharedSinglyLinkedList<T>
    where
        T: Clone,
    {
        fn append_nodes<T: Clone>(n: Option<Rc<Node<T>>>, tail: &Option<Rc<Node<T>>>) -> Option<Rc<Node<T>>> {
            match n {
                None => tail.clone(),
                Some(rc) => Some(Rc::new(Node { value: rc.value.clone(), next: append_nodes(rc.next.clone(), tail) })),
            }
        }
        SharedSinglyLinkedList { head: append_nodes(a.head.clone(), &b.head) }
    }
}

impl<T: std::fmt::Debug> std::fmt::Debug for SharedSinglyLinkedList<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        let mut first = true;
        let mut cur = self.head.as_ref().map(|rc| rc.clone());
        while let Some(rc) = cur {
            if !first { write!(f, ", ")?; }
            first = false;
            write!(f, "{:?}", rc.value)?;
            cur = rc.next.as_ref().map(|x| x.clone());
        }
        write!(f, "]")
    }
}

impl<T> SharedSinglyLinkedList<T> {
    pub fn ptr_eq_head(&self, other: &SharedSinglyLinkedList<T>) -> bool {
        match (&self.head, &other.head) {
            (Some(a), Some(b)) => Rc::ptr_eq(a, b),
            (None, None) => true,
            _ => false,
        }
    }
}


