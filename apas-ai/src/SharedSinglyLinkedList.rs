//! Immutable, structurally shared (functional) singly linked list with shared tails (Lisp-style) using Rc.

use std::rc::Rc;
use crate::Types::{B, N};

pub struct Node<T> {
    pub value: T,
    pub next: Option<Rc<Node<T>>>,
}

pub struct SharedSinglyLinkedList<T> {
    pub(crate) head: Option<Rc<Node<T>>>,
    pub(crate) len: N,
}

impl<T> Clone for SharedSinglyLinkedList<T> {
    fn clone(&self) -> Self { SharedSinglyLinkedList { head: self.head.clone(), len: self.len } }
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
    /// ChatGPT-5-hard: Work Θ(1), Span Θ(1).
    fn length(&self) -> N;

    /// APAS 20.7 (map f a): Work Θ(1 + Σ x∈a W(f(x))), Span Θ(1 + Σ x∈a S(f(x))).
    fn map<U>(a: &SharedSinglyLinkedList<T>, f: impl Fn(&T) -> U) -> SharedSinglyLinkedList<U>;

    /// APAS 20.7 (append a b): Work Θ(1 + |a|), Span Θ(1 + |a|).
    fn append(a: &SharedSinglyLinkedList<T>, b: &SharedSinglyLinkedList<T>) -> SharedSinglyLinkedList<T>
    where
        T: Clone;
}

impl<T> SharedSList<T> for SharedSinglyLinkedList<T> {
    /// ChatGPT-5-hard: Work Θ(1), Span Θ(1).
    fn empty() -> SharedSinglyLinkedList<T> { SharedSinglyLinkedList { head: None, len: 0 } }
    /// ChatGPT-5-hard: Work Θ(1), Span Θ(1).
    fn isEmpty(&self) -> B { if self.head.is_none() { B::True } else { B::False } }
    /// ChatGPT-5-hard: Work Θ(1), Span Θ(1).
    fn cons(x: T, tail: &SharedSinglyLinkedList<T>) -> SharedSinglyLinkedList<T> {
        SharedSinglyLinkedList { head: Some(Rc::new(Node { value: x, next: tail.head.clone() })), len: tail.len + 1 }
    }
    /// ChatGPT-5-hard: Work Θ(1), Span Θ(1).
    fn head(&self) -> Option<&T> { self.head.as_deref().map(|n| &n.value) }
    /// ChatGPT-5-hard: Work Θ(1), Span Θ(1).
    fn tail(&self) -> SharedSinglyLinkedList<T> {
        let next_head = self.head.as_ref().and_then(|n| n.next.clone());
        let new_len = if self.len > 0 { self.len - 1 } else { 0 };
        SharedSinglyLinkedList { head: next_head, len: new_len }
    }
    /// ChatGPT-5-hard: Work Θ(1), Span Θ(1).
    fn length(&self) -> N { self.len }

    /// APAS: Work Θ(1 + Σ x∈a W(f(x))), Span Θ(1 + Σ x∈a S(f(x))).
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
        SharedSinglyLinkedList { head: map_nodes(a.head.clone(), &f), len: a.len }
    }

    /// APAS: Work Θ(1 + |a|), Span Θ(1 + |a|).
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
        SharedSinglyLinkedList { head: append_nodes(a.head.clone(), &b.head), len: a.len + b.len }
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
    /// ChatGPT-5-hard: Work Θ(1), Span Θ(1).
    pub fn new() -> SharedSinglyLinkedList<T> { SharedSinglyLinkedList { head: None, len: 0 } }
    /// ChatGPT-5-hard: Work Θ(1), Span Θ(1).
    pub fn cons(x: T, tail: &SharedSinglyLinkedList<T>) -> SharedSinglyLinkedList<T> {
        <SharedSinglyLinkedList<T> as SharedSList<T>>::cons(x, tail)
    }
    /// ChatGPT-5-hard: Work Θ(i), Span Θ(i).
    pub fn nth_ref(&self, index: N) -> Option<&T> {
        let mut i = index;
        let mut cur = self.head.as_deref();
        while i > 0 {
            match cur { None => return None, Some(node) => { cur = node.next.as_deref(); i -= 1; } }
        }
        cur.map(|node| &node.value)
    }
    /// APAS: Work Θ(1 + Σ x∈a W(f(x))), Span Θ(1 + Σ x∈a S(f(x))).
    pub fn map<U>(a: &SharedSinglyLinkedList<T>, f: impl Fn(&T) -> U) -> SharedSinglyLinkedList<U> {
        <SharedSinglyLinkedList<T> as SharedSList<T>>::map(a, f)
    }
    /// APAS: Work Θ(1 + |a|), Span Θ(1 + |a|).
    pub fn append(a: &SharedSinglyLinkedList<T>, b: &SharedSinglyLinkedList<T>) -> SharedSinglyLinkedList<T>
    where
        T: Clone,
    {
        <SharedSinglyLinkedList<T> as SharedSList<T>>::append(a, b)
    }
    /// ChatGPT-5-hard: Work Θ(1), Span Θ(1).
    pub fn length(&self) -> N { self.len }
    pub fn ptr_eq_head(&self, other: &SharedSinglyLinkedList<T>) -> bool {
        match (&self.head, &other.head) {
            (Some(a), Some(b)) => Rc::ptr_eq(a, b),
            (None, None) => true,
            _ => false,
        }
    }
}


