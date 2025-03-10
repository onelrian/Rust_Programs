///
/// This function finds and return the longest
///  substring in a given string
///
/// Logic:
///  - initializes array of string slice,  
///  - check if the given string length < 2a string slice containing the entire Stri
///         return string
///  - else add the first character of given string
///     to the first elt of the array
///  - loop over the given string character
///     * check if char is in array of string
///         continue to the next slice
///     * else you add char to the string slice
///  - find the longest slice in the array.
///       
pub fn longest_common_substring(string: String) -> String {
    if string.len() < 2 {
        return string;
    }

    // Option 1
    // let char = string.get(0..1).unwrap();

    // Option 2
    // let char = string.get(0..1).expect("Failed to colled string slice");
    
    // Option 3
    let mut char_iter = string.chars();
    let char = match char_iter.next() {
        Some(str) => str,
        None => panic!(),
    };

    let mut arr_slices = vec![char];

    // for i in 0..string.len() {
    //     let c_char = string.get(i..i + 1).unwrap();
    // }

    let length = arr_slices.len();
    let arr_slices = &arr_slices;
    while let Some(char) = char_iter.next() {
        for i in 0..length {
        let str = arr_slices
        }
        println!("char: {char}");
    }

    String::new()
}
