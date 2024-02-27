use content_mapp_rs::{get_all_file_paths, ncd};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn bench(c: &mut Criterion) {
    let paths = get_all_file_paths("./test_data").unwrap();
    c.bench_function("get n most similar", |b| {
        b.iter(|| {
            ncd::get_n_most_similar_files(black_box(5), black_box(&paths[0]), black_box(&paths))
        })
    });
}

criterion_group! {
    name = benches;
    config = Criterion::default().sample_size(50);
    targets = bench
}
criterion_main!(benches);
