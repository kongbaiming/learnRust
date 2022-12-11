pub fn first_world(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i,&item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}


