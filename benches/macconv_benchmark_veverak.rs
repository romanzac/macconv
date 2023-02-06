pub struct FormattedMac {
    inner: [u8; 17],
}

impl std::ops::Deref for FormattedMac {
    type Target = str;

    fn deref(&self) -> &str {
        unsafe { std::str::from_utf8_unchecked(&self.inner) }
    }
}

impl std::fmt::Debug for FormattedMac {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self, formatter)
    }
}

impl std::fmt::Display for FormattedMac {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(&**self, formatter)
    }
}

pub fn format_mac(source: &str) -> Result<FormattedMac, ()> {
    if source.len() != 12 {
        return Err(());
    }
    let mut buffer = [std::mem::MaybeUninit::uninit(); 17];
    let mut bytes = source.bytes();
    let mut position = 0;
    loop {
        for _ in 0..2 {
            let byte = bytes.next().unwrap();
            buffer[position].write(match byte {
                b'0'..=b'9' => byte,
                b'A'..=b'F' => byte - b'A' + b'a',
                _ => {
                    return Err(());
                }
            });
            position += 1;
        }
        if bytes.len() == 0 {
            break;
        }
        buffer[position].write(b':');
        position += 1;
    }
    Ok(FormattedMac {
        inner: buffer.map(|byte| unsafe { byte.assume_init() }),
    })
}


fn benchmark(c: &mut Criterion) {
    c.bench_function("format_mac", |b| {
        b.iter(|| format_mac(black_box("00B0D063C226")))
    });
}

criterion_group!(benches, benchmark);
criterion_main!(benches);

