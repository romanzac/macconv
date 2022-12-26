use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn format_mac(m: &str) -> Result<String, &'static str> {
    let mut result: Vec<u8> = vec![
        0u8, 0u8, b':', 0u8, 0u8, b':', 0u8, 0u8, b':', 0u8, 0u8, b':', 0u8, 0u8, b':',
        0u8, 0u8
    ];
    let mut i = 0usize;

    if m.len() != 12 {
        return Err("unknown error")
    }

    let n = m.bytes();
    for c in n {
        match c {
            65..=70 => result[i] = c + 32,
            48..=57 => result[i] = c,
            _ => return Err("unknown error")
        }
        match i {
            1 | 4 | 7 | 10 | 13 => i += 2,
            _ => i += 1
        }
    }

    Ok(String::from_utf8(result).unwrap())
}

fn benchmark(c: &mut Criterion) {
    c.bench_function("format_mac", |b| {
        b.iter(|| format_mac(black_box("00B0D063C226")))
    });
}

criterion_group!(benches, benchmark);
criterion_main!(benches);

