use regex::Regex;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn format_mac(m: &str, mac_regx: &Regex) -> Result<String, &'static str> {
    let mut result = String::new();
    let mut i = 1i32;

    for c in m.chars() {
        if !mac_regx.is_match(&c.to_string()) || i > 12 {
            return Err("unknown error");
        }
        result = result + &c.to_lowercase().to_string();
        if i < 12 && i % 2 == 0 {
            result = result + ":".into();
        }
        i += 1;
    }

    if i == 13 {
        Ok(result)
    } else {
        Err("unknown error")
    }

}

fn benchmark(c: &mut Criterion) {
    let hex_digit = Regex::new(r"[[:xdigit:]]").unwrap();
    c.bench_function("format_mac", |b| {
        b.iter(|| format_mac(black_box("859C9081002F"), black_box(&hex_digit)))
    });
}

criterion_group!(benches, benchmark);
criterion_main!(benches);

// fn main() {
//     let hex_digit = Regex::new(r"[[:xdigit:]]").unwrap();
//     let new_mac = format_mac("D2BE423F9A3E",&hex_digit).unwrap();
//     println!("Converted MAC {}", new_mac);
// }
