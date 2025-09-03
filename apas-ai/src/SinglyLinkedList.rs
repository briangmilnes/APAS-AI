//! A minimal singly linked list sequence `SinglyLinkedList<T>` implemented from scratch.
//! Provides a subset of sequence operations with explicit complexity documentation.

use crate::Types::{N, B};

pub struct Node<T> {
    pub value: T,
    pub next: Option<Box<Node<T>>>,
}

pub struct SinglyLinkedList<T> {
    head: Option<Box<Node<T>>>,
    len: N,
}

pub trait SinglyList<T> {
    /// ChatGPT-5-hard: Work Θ(1), Span Θ(1).
    fn empty() -> SinglyLinkedList<T>;
    /// ChatGPT-5-hard: Work Θ(length), Span Θ(length).
    fn new(length: N, init_value: T) -> SinglyLinkedList<T> where T: Clone;
    /// APAS: Work Θ(1), Span Θ(1).
    fn length(&self) -> N;
    /// APAS: Work Θ(i), Span Θ(i).
    fn nth(&self, index: N) -> &T;
    /// APAS: Work Θ(1), Span Θ(1).
    fn isEmpty(&self) -> B;
    /// APAS: Work Θ(1), Span Θ(1).
    fn isSingleton(&self) -> B;
    /// APAS: Work Θ(1), Span Θ(1).
    fn singleton(item: T) -> SinglyLinkedList<T>;
    /// APAS: Work Θ(1 + |a|), Span Θ(1 + |a|). 
    /// ChatGPT-5-hard: Work Θ(i), Span Θ(i).
    /// BUG: APAS and ChatGPT-5-hard algorithmic analyses differ.
    fn set(&mut self, index: N, item: T) -> Result<&mut SinglyLinkedList<T>, &'static str>;
    /// APAS: Work Θ(1 + i), Span Θ(1 + i).
    /// ChatGPT-5-hard: Work Θ(1 + i + length), Span Θ(1 + i).
    /// BUG: APAS and ChatGPT-5-hard algorithmic analyses differ.
    fn subseq_copy(&self, start: N, length: N) -> SinglyLinkedList<T> where T: Clone;
}

impl<T> SinglyLinkedList<T> {
    fn push_front_node(&mut self, node: Box<Node<T>>) {
        let mut n = node;
        n.next = self.head.take();
        self.head = Some(n);
        self.len += 1;
    }
}

impl<T> SinglyList<T> for SinglyLinkedList<T> {
    fn empty() -> SinglyLinkedList<T> { SinglyLinkedList { head: None, len: 0 } }

    fn new(length: N, init_value: T) -> SinglyLinkedList<T> where T: Clone {
        let mut list = SinglyLinkedList::empty();
        for _ in 0..length { list.push_front_node(Box::new(Node { value: init_value.clone(), next: None })); }
        // reverse to maintain intuitive order
        let mut rev: Option<Box<Node<T>>> = None;
        while let Some(mut n) = list.head.take() { let next = n.next.take(); n.next = rev.take(); rev = Some(n); list.head = next; }
        list.head = rev; list
    }

    fn length(&self) -> N { self.len }

    fn nth(&self, index: N) -> &T {
        let mut i = 0; let mut cur = self.head.as_ref();
        while let Some(n) = cur { if i == index { return &n.value; } i += 1; cur = n.next.as_ref(); }
        panic!("index out of bounds");
    }

    fn isEmpty(&self) -> B { if self.len == 0 { B::True } else { B::False } }

    fn isSingleton(&self) -> B { if self.len == 1 { B::True } else { B::False } }

    fn singleton(item: T) -> SinglyLinkedList<T> {
        SinglyLinkedList { head: Some(Box::new(Node { value: item, next: None })), len: 1 }
    }

    fn set(&mut self, index: N, item: T) -> Result<&mut SinglyLinkedList<T>, &'static str> {
        let mut i = 0; let mut cur = self.head.as_mut();
        while let Some(n) = cur { if i == index { n.value = item; return Ok(self); } i += 1; cur = n.next.as_mut(); }
        Err("Index out of bounds")
    }

    fn subseq_copy(&self, start: N, length: N) -> SinglyLinkedList<T> where T: Clone {
        let n = self.len; let s = start.min(n); let e = start.saturating_add(length).min(n);
        if e <= s { return SinglyLinkedList::empty(); }
        let mut out = SinglyLinkedList::empty();
        let mut i = 0; let mut cur = self.head.as_ref();
        while let Some(node) = cur { if i >= s && i < e { out.push_front_node(Box::new(Node { value: node.value.clone(), next: None })); }
            if i >= e { break; } i += 1; cur = node.next.as_ref(); }
        // reverse to maintain order
        let mut rev: Option<Box<Node<T>>> = None;
        while let Some(mut n) = out.head.take() { let next = n.next.take(); n.next = rev.take(); rev = Some(n); out.head = next; }
        out.head = rev; out
    }
}

impl<T: std::fmt::Debug> std::fmt::Debug for SinglyLinkedList<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut v = Vec::with_capacity(self.len);
        let mut cur = self.head.as_ref();
        while let Some(n) = cur { v.push(&n.value); cur = n.next.as_ref(); }
        f.debug_list().entries(v).finish()
    }
}


