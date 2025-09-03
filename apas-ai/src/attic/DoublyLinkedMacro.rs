// Moved from lib.rs: historical macro for building doubly linked list sequences.
// Left here for reference and excluded from build.

#[macro_export]
macro_rules! dllseq {
    () => {{
        compile_error!("DoublyLinkedListSeq is in attic; dllseq! is unsupported");
    }};
    ($x:expr; $n:expr) => {{
        compile_error!("DoublyLinkedListSeq is in attic; dllseq! is unsupported");
    }};
    ($($x:expr),* $(,)?) => {{
        compile_error!("DoublyLinkedListSeq is in attic; dllseq! is unsupported");
    }};
}


