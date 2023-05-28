fn format_mac(m: &str) -> Result<String, &'static str> {
    let mut result: Vec<u8> = vec![
        0u8, 0u8, b':', 0u8, 0u8, b':', 0u8, 0u8, b':', 0u8, 0u8, b':', 0u8, 0u8, b':', 0u8, 0u8,
    ];
    let mut i = 0usize;

    if m.len() != 12 {
        return Err("unknown error");
    }

    let n = m.bytes();
    for c in n {
        match c {
            65..=70 => result[i] = c + 32,
            48..=57 => result[i] = c,
            _ => return Err("unknown error"),
        }
        match i {
            1 | 4 | 7 | 10 | 13 => i += 2,
            _ => i += 1,
        }
    }

    Ok(String::from_utf8(result).unwrap())
}

fn main() {
    let new_mac = format_mac("00B0D063C226").unwrap();
    println!("Converted MAC {}", new_mac);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;

    struct TestCases(&'static str, &'static str, bool);

    #[test]
    fn test_format_mac() {
        let test_cases: Vec<TestCases> = vec![
            TestCases("859C9081002F", "85:9c:90:81:00:2f", false),
            TestCases("D2BE423F9A3E", "d2:be:42:3f:9a:3e", false),
            TestCases("1C95859FD478", "1c:95:85:9f:d4:78", false),
            TestCases("F938F9467104", "f9:38:f9:46:71:04", false),
            TestCases("9AB8FCBA1A45", "9a:b8:fc:ba:1a:45", false),
            TestCases("71024BD91E72", "71:02:4b:d9:1e:72", false),
            TestCases("29C9802C5224", "29:c9:80:2c:52:24", false),
            TestCases("1851B0BF10DD", "18:51:b0:bf:10:dd", false),
            TestCases("925C97E05D19", "92:5c:97:e0:5d:19", false),
            TestCases("68A4D596B4E7", "68:a4:d5:96:b4:e7", false),
            TestCases("68A4D596B4E7777", "", true),
            TestCases("*$(*&%$)#", "", true),
            TestCases("68A4D59^6B4E7", "", true),
            TestCases("68:a4:d5:96:b4:e7", "", true),
            TestCases("71:02:4B:D9:1E:72", "", true),
        ];

        let now = Instant::now();
        for tc in test_cases {
            let res = format_mac(tc.0);
            match res {
                Ok(r) => {
                    if r != tc.1 {
                        panic!("Positive failed for {}", tc.0);
                    };
                }
                Err(_) => {
                    if !tc.2 {
                        panic!("Negative test failed for {}", tc.0);
                    }
                }
            }
        }
        println!(
            "Test finished in {} microseconds",
            now.elapsed().as_micros()
        );
    }
}
