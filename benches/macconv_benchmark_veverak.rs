use criterion::{black_box, criterion_group, criterion_main, Criterion};

// Comments by Veverak
// Vec and String are complex types that allocate memory and are optimized for pushing an unbounded
// amount of items to the end. You don't need them. I would use an array for the buffer.
// That is [u8; 17]. Unfortunately there is no standard type for owned strings backed by a primitive array,
// so you have to make your own. I would make a type that wraps the array and implements the trait Deref<str>.
//
// The loop contains a match expression with quite a lot of conditions. I would unroll the loop to write
// two digits at a time and then write the delimiter.
//
// Since it's clear that there will never be any invalid UTF-8, I would use the unchecked variant of
// String::from_utf8 (or better str::from_utf8 in the Deref implementation suggested above).
//
// As we're starting to get comfortable with unsafe code after applying the last suggestion,
// we can as well make the buffer [std::mem::MaybeUninit::uninit(); 17] and only write to each byte once.


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

