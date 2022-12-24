use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn format_mac(m: &str) -> Result<String, &'static str> {
    let mut result = String::new();
    let mut i = 1i32;

    if m.len() != 12 {
        return Err("unknown error")
    }

    for c in m.chars() {
        if !c.is_ascii_hexdigit() {
            return Err("unknown error")
        }
        result.push(c.to_ascii_lowercase());
        if i != 12 && i % 2 == 0 {
            result.push(':');
        }
        i += 1;
    }

    Ok(result)
}

fn benchmark(c: &mut Criterion) {
    c.bench_function("format_mac", |b| {
        b.iter(|| format_mac(black_box("00B0D063C226")))
    });
}

criterion_group!(benches, benchmark);
criterion_main!(benches);

