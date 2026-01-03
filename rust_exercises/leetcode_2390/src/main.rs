pub fn remove_stars_slow(s: String) -> String {
    let mut stack = String::from("");
    for ch in s.chars(){
        if ch == '*' {
            stack.pop();
        }
        else {
            stack.push(ch);
        }
    }
// this solution is slow because of the fact that strings are utf-8 encoded
       
// this solution directly works with the bytes. if the string had utf-8 chars there would be errors
pub fn remove_stars_fast_unsafe(s: String) -> String {
    let mut stack: Vec<u8> = Vec::with_capacity(s.len()); //preallocates length to avoid resizing
    
    for byte in s.as_bytes() {
        if *byte == b'*' {
            stack.pop();
        } else {
            stack.push(*byte);
        }
    }
    
    unsafe { String::from_utf8_unchecked(stack) } // faster because it doesn't check for utf-8 validity
}

//similar safe version
pub fn remove_stars_fast_safe(s: String) -> String {
    let mut stack: Vec<u8> = Vec::with_capacity(s.len()); //preallocates length to avoid resizing
    
    for byte in s.as_bytes() {
        if *byte == b'*' {
            stack.pop();
        } else {
            stack.push(*byte);
        }
    }
    
    String::from_utf8(stack).unwrap() // faster because it doesn't check for utf-8 validity
}