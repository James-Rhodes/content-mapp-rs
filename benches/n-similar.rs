use content_mapp_rs::{file_walk::get_all_file_paths, indexer::index_all_files};
use criterion::{criterion_group, criterion_main, Criterion};

pub fn bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("n_similar");
    // group.sample_size(50);
    let paths = get_all_file_paths("./test_data").unwrap();
    // group.bench_function("n_similar_one_path", |b| {
    //     b.iter(|| {
    //         ncd::get_n_most_similar_files(black_box(5), black_box(&paths[0]), black_box(&paths))
    //     })
    // });

    group.sample_size(10);
    group.bench_function("n_similar_all_path_parallel", |b| {
        b.iter(|| {
            let _results = index_all_files(&paths, 5).unwrap();
        })
    });
}

criterion_group!(benches, bench);
criterion_main!(benches);
