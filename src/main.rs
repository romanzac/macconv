use regex::Regex;

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


fn main() {
    let hex_digit = Regex::new(r"[[:xdigit:]]").unwrap();
    let new_mac = format_mac("D2BE423F9A3E",&hex_digit).unwrap();
    println!("Converted MAC {}", new_mac);
}

#[cfg(test)]
mod tests {
    use super::*;

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

        let hex_digit = Regex::new(r"[[:xdigit:]]").unwrap();

        for tc in test_cases {
            format_mac(tc.0, &hex_digit);
            // match res {
            //     Ok(r) => {
            //         if r != tc.1 {
            //             panic!("Positive failed for {}", tc.0);
            //         };
            //     }
            //     Err(_) => {
            //         if !tc.2 {
            //             panic!("Negative test failed for {}", tc.0);
            //         }
            //     }
            // }
        }
    }
}