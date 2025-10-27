use std::hint::black_box;
use std::path::Path;

use criterion::{Criterion, criterion_group, criterion_main};
use schiller_lib::db::{AtomicDatabase, BookSearch};
use schiller_lib::util;

fn criterion_benchmark(c: &mut Criterion) {
    util::logging();
    let db = AtomicDatabase::load(Path::new("test/demo.json")).unwrap();
    let db = db.read();

    let search = BookSearch {
        query: String::new(),
        category: String::new(),
        state: schiller_lib::db::BookState::None,
        offset: 0,
        limit: 200,
    };
    c.bench_function("book_all", |b| {
        b.iter(|| {
            let results = black_box(&db.books).search(black_box(&search));
            let json = serde_json::to_string(&results).unwrap();
            black_box(json);
        })
    });

    let search = BookSearch {
        query: "ein".into(),
        category: String::new(),
        state: schiller_lib::db::BookState::None,
        offset: 0,
        limit: 200,
    };
    c.bench_function("book_search", |b| {
        b.iter(|| {
            let results = black_box(&db.books).search(black_box(&search));
            let json = serde_json::to_string(&results).unwrap();
            black_box(json);
        })
    });

    let id = "demo STIX 1".to_string();
    c.bench_function("book_fetch", |b| {
        b.iter(|| {
            let book = black_box(&db.books).fetch(black_box(&id));
            let json = serde_json::to_string(&book).unwrap();
            black_box(json);
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
