// strings3.rs
// Execute `rustlings hint strings3` or use the `hint` watch subcommand for a hint.

fn trim_me(input: &str) -> String {
    let bytes = input.as_bytes();
    let mut loopFlag = true;
    let mut start = 0;
    let mut end = bytes.len();

    for (_i, &item) in bytes.iter().enumerate() {
        if loopFlag && item == b' '{
            start += 1;
        }else{
            loopFlag &= item == b' ';
        }
    }
    loopFlag = true;
    for (_i, &item) in bytes.iter().rev().enumerate() {
        if loopFlag && item == b' '{
            end -= 1;
        }else{
            loopFlag &= item == b' ';
        }
    }
    if start > end {
        end = start;
    }
    input[start..end].to_string()
    // TODO: Remove whitespace from both ends of a string!
}

fn compose_me(input: &str) -> String {
    let mut s1 = input.to_string();
    s1.push_str(" world!");
    s1
    // TODO: Add " world!" to the string! There's multiple ways to do this!
}

fn replace_me(input: &str) -> String {
    input.to_string().replace("cars","balloons")
    // TODO: Replace "cars" in the string with "balloons"!
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trim_a_string() {
        assert_eq!(trim_me("Hello!     "), "Hello!");
        assert_eq!(trim_me("  What's up!"), "What's up!");
        assert_eq!(trim_me("   Hola!  "), "Hola!");
        assert_eq!(trim_me("  "), "");
    }

    #[test]
    fn compose_a_string() {
        assert_eq!(compose_me("Hello"), "Hello world!");
        assert_eq!(compose_me("Goodbye"), "Goodbye world!");
    }

    #[test]
    fn replace_a_string() {
        assert_eq!(replace_me("I think cars are cool"), "I think balloons are cool");
        assert_eq!(replace_me("I love to look at cars"), "I love to look at balloons");
    }
}
