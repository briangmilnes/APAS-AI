//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 44: Document Index Benchmarks

use apas_ai::Chap19::ArraySeqStPer::ArraySeqStPer::*;
use apas_ai::Chap44::DocumentIndex::DocumentIndex::*;
use apas_ai::Chap44::Example44_1::Example44_1::*;
use apas_ai::DocumentCollectionLit;
use apas_ai::Types::Types::*;
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};

/// Create a document collection of specified size for benchmarking
fn create_benchmark_documents(size: usize) -> DocumentCollection {
    let mut docs = ArraySeqStPerS::empty();

    for i in 0..size {
        let doc_id = format!("doc{}", i);
        let content = format!(
            "document {} contains words like test data item {} benchmark performance analysis",
            i,
            i % 10
        );
        let pair = Pair(doc_id, content);
        let single_seq = ArraySeqStPerS::singleton(pair);
        docs = ArraySeqStPerS::append(&docs, &single_seq);
    }

    docs
}

/// Brute force search implementation for comparison
fn brute_force_search(docs: &DocumentCollection, word: &str) -> Vec<String> {
    let mut results = Vec::new();

    for i in 0..docs.length() {
        let doc = docs.nth(i);
        let content = &doc.1;

        // Simple word matching (case-insensitive)
        if content.to_lowercase().contains(&word.to_lowercase()) {
            results.push(doc.0.clone());
        }
    }

    results
}

/// Benchmark makeIndex operation (Algorithm 44.2)
fn bench_make_index(c: &mut Criterion) {
    let mut group = c.benchmark_group("makeIndex");

    for size in [10, 50, 100, 500].iter() {
        let docs = create_benchmark_documents(*size);

        group.bench_with_input(BenchmarkId::new("DocumentIndex::make_index", size), size, |b, _| {
            b.iter(|| {
                let index = DocumentIndex::make_index(black_box(&docs));
                black_box(index)
            })
        });
    }

    group.finish();
}

/// Benchmark find operation
fn bench_find_operation(c: &mut Criterion) {
    let mut group = c.benchmark_group("find");

    for size in [10, 50, 100, 500].iter() {
        let docs = create_benchmark_documents(*size);
        let index = DocumentIndex::make_index(&docs);

        group.bench_with_input(BenchmarkId::new("DocumentIndex::find", size), size, |b, _| {
            b.iter(|| {
                let result = index.find(black_box(&"test".to_string()));
                black_box(result)
            })
        });
    }

    group.finish();
}

/// Benchmark query operations (AND, OR, AND NOT)
fn bench_query_operations(c: &mut Criterion) {
    let docs = create_benchmark_documents(100);
    let index = DocumentIndex::make_index(&docs);

    let test_docs = index.find(&"test".to_string());
    let data_docs = index.find(&"data".to_string());

    c.bench_function("query_and", |b| {
        b.iter(|| {
            let result = DocumentIndex::query_and(black_box(&test_docs), black_box(&data_docs));
            black_box(result)
        })
    });

    c.bench_function("query_or", |b| {
        b.iter(|| {
            let result = DocumentIndex::query_or(black_box(&test_docs), black_box(&data_docs));
            black_box(result)
        })
    });

    c.bench_function("query_and_not", |b| {
        b.iter(|| {
            let result = DocumentIndex::query_and_not(black_box(&test_docs), black_box(&data_docs));
            black_box(result)
        })
    });
}

/// Benchmark indexed search vs brute force search
fn bench_indexed_vs_brute_force(c: &mut Criterion) {
    let mut group = c.benchmark_group("search_comparison");

    for size in [10, 50, 100, 500].iter() {
        let docs = create_benchmark_documents(*size);
        let index = DocumentIndex::make_index(&docs);

        // Benchmark indexed search
        group.bench_with_input(BenchmarkId::new("indexed_search", size), size, |b, _| {
            b.iter(|| {
                let result = index.find(black_box(&"test".to_string()));
                black_box(result)
            })
        });

        // Benchmark brute force search
        group.bench_with_input(BenchmarkId::new("brute_force_search", size), size, |b, _| {
            b.iter(|| {
                let result = brute_force_search(black_box(&docs), black_box("test"));
                black_box(result)
            })
        });
    }

    group.finish();
}

/// Benchmark tweet example scenarios
fn bench_tweet_examples(c: &mut Criterion) {
    let examples = TweetQueryExamples::new();

    c.bench_function("tweet_search_fun", |b| {
        b.iter(|| {
            let result = examples.search_fun();
            black_box(result)
        })
    });

    c.bench_function("tweet_complex_query", |b| {
        b.iter(|| {
            let result = examples.complex_query_fun_and_food_or_chess();
            black_box(result)
        })
    });

    c.bench_function("tweet_count_query", |b| {
        b.iter(|| {
            let result = examples.count_fun_but_not_chess();
            black_box(result)
        })
    });

    c.bench_function("tweet_query_builder", |b| {
        b.iter(|| {
            let result = examples.query_builder_example();
            black_box(result)
        })
    });
}

/// Benchmark tokenization performance
fn bench_tokenization(c: &mut Criterion) {
    let mut group = c.benchmark_group("tokenization");

    let short_text = "hello world programming";
    let medium_text = "I had fun in dance club today and it was amazing";
    let long_text = "This is a much longer text that contains many more words and should take more time to tokenize because it has significantly more content to process and analyze";

    group.bench_function("short_text", |b| {
        b.iter(|| {
            let result = tokens(black_box(&short_text.to_string()));
            black_box(result)
        })
    });

    group.bench_function("medium_text", |b| {
        b.iter(|| {
            let result = tokens(black_box(&medium_text.to_string()));
            black_box(result)
        })
    });

    group.bench_function("long_text", |b| {
        b.iter(|| {
            let result = tokens(black_box(&long_text.to_string()));
            black_box(result)
        })
    });

    group.finish();
}

/// Benchmark size and toSeq operations
fn bench_utility_operations(c: &mut Criterion) {
    let docs = create_benchmark_documents(100);
    let index = DocumentIndex::make_index(&docs);
    let test_docs = index.find(&"test".to_string());

    c.bench_function("size_operation", |b| {
        b.iter(|| {
            let result = DocumentIndex::size(black_box(&test_docs));
            black_box(result)
        })
    });

    c.bench_function("to_seq_operation", |b| {
        b.iter(|| {
            let result = DocumentIndex::to_seq(black_box(&test_docs));
            black_box(result)
        })
    });

    c.bench_function("word_count", |b| {
        b.iter(|| {
            let result = index.word_count();
            black_box(result)
        })
    });
}

/// Benchmark complex query combinations
fn bench_complex_queries(c: &mut Criterion) {
    let docs = create_benchmark_documents(200);
    let index = DocumentIndex::make_index(&docs);

    let test_docs = index.find(&"test".to_string());
    let data_docs = index.find(&"data".to_string());
    let document_docs = index.find(&"document".to_string());
    let benchmark_docs = index.find(&"benchmark".to_string());

    c.bench_function("complex_and_or_query", |b| {
        b.iter(|| {
            // (test AND data) OR (document AND benchmark)
            let left = DocumentIndex::query_and(black_box(&test_docs), black_box(&data_docs));
            let right = DocumentIndex::query_and(black_box(&document_docs), black_box(&benchmark_docs));
            let result = DocumentIndex::query_or(&left, &right);
            black_box(result)
        })
    });

    c.bench_function("complex_nested_query", |b| {
        b.iter(|| {
            // ((test OR data) AND document) AND NOT benchmark
            let test_or_data = DocumentIndex::query_or(black_box(&test_docs), black_box(&data_docs));
            let with_document = DocumentIndex::query_and(&test_or_data, black_box(&document_docs));
            let result = DocumentIndex::query_and_not(&with_document, black_box(&benchmark_docs));
            black_box(result)
        })
    });
}

/// Benchmark scalability with varying document sizes
fn bench_scalability(c: &mut Criterion) {
    let mut group = c.benchmark_group("scalability");

    // Test with different document collection sizes
    for size in [100, 500, 1000, 2000].iter() {
        let docs = create_benchmark_documents(*size);

        group.bench_with_input(BenchmarkId::new("index_creation", size), size, |b, _| {
            b.iter(|| {
                let index = DocumentIndex::make_index(black_box(&docs));
                black_box(index)
            })
        });
    }

    // Test search performance with different index sizes
    for size in [100, 500, 1000, 2000].iter() {
        let docs = create_benchmark_documents(*size);
        let index = DocumentIndex::make_index(&docs);

        group.bench_with_input(BenchmarkId::new("search_performance", size), size, |b, _| {
            b.iter(|| {
                let result = index.find(black_box(&"test".to_string()));
                black_box(result)
            })
        });
    }

    group.finish();
}

/// Benchmark memory efficiency by testing with large datasets
fn bench_memory_efficiency(c: &mut Criterion) {
    let large_docs = create_benchmark_documents(1000);
    let index = DocumentIndex::make_index(&large_docs);

    c.bench_function("large_index_search", |b| {
        b.iter(|| {
            let result = index.find(black_box(&"test".to_string()));
            black_box(result)
        })
    });

    c.bench_function("large_index_complex_query", |b| {
        b.iter(|| {
            let test_docs = index.find(&"test".to_string());
            let data_docs = index.find(&"data".to_string());
            let result = DocumentIndex::query_and(black_box(&test_docs), black_box(&data_docs));
            black_box(result)
        })
    });
}

criterion_group!(
    benches,
    bench_make_index,
    bench_find_operation,
    bench_query_operations,
    bench_indexed_vs_brute_force,
    bench_tweet_examples,
    bench_tokenization,
    bench_utility_operations,
    bench_complex_queries,
    bench_scalability,
    bench_memory_efficiency
);

criterion_main!(benches);
