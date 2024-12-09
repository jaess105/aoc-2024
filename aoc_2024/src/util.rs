pub fn unwrap_to_i32(s: &str) -> i32 {
    i32::from_str_radix(s, 10).expect(format!("invalid i32 {} ", s).as_str())
}

pub fn unwrap_to_i64(s: &str) -> i64 {
    i64::from_str_radix(s, 10).unwrap()
}
