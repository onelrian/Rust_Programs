fn longest_string<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    let len1 = s1.len();
    let len2 = s2.len();
    if len2 > len1 {
        s2
    }else{
        s1
    }
}