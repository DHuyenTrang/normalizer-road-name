use criterion::{black_box, criterion_group, criterion_main, Criterion};
use road_name_normalizer::{normalize, Mode};

fn benchmark_normalize(criterion: &mut Criterion) {
    let cases = [
        ("unmatched", "Cầu vượt Đường Sắt"),
        ("early_rule", "Đường cao tốc Biên Hoà - Vũng Tàu"),
        ("late_rule", "Phố Nguyễn Du"),
    ];

    for (name, input) in cases {
        criterion.bench_function(name, |bencher| {
            bencher.iter(|| normalize(black_box(input), black_box(Mode::Abbreviate)));
        });
    }
}

criterion_group!(benches, benchmark_normalize);
criterion_main!(benches);
