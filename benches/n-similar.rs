use content_mapp_rs::indexer::Indexer;
use criterion::{criterion_group, criterion_main, Criterion};

pub fn bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("n_similar");
    // group.sample_size(50);
    let mut indexer = Indexer::new("./test_data", 5).unwrap();
    group.sample_size(10);
    group.bench_function("n_similar_all_path_parallel", |b| {
        b.iter(|| {
            indexer.index_all_files().unwrap();
        })
    });
}

criterion_group!(benches, bench);
criterion_main!(benches);
