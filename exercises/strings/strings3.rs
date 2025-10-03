// strings3.rs
//
// Execute `rustlings hint strings3` or use the `hint` watch subcommand for a
// hint.

// 写的太复杂了，str本身已有trim方法
// fn trim_me(input: &str) -> String {
//     // Remove whitespace from both ends of a string!
//     let mut cnt_tail_zeros = 0;
//     for c in input.chars().rev() {
//         match c {
//             ' ' => cnt_tail_zeros = cnt_tail_zeros + 1,
//             _ => break,
//         }
//     }
//     let mut cnt_head_zeros = 0;
//     for c in input.chars() {
//         match c {
//             ' ' => cnt_head_zeros = cnt_head_zeros + 1,
//             _ => break,
//         }
//     }
//     (input[cnt_head_zeros .. input.len() - cnt_tail_zeros]).to_string()
// }

fn trim_me(input: &str) -> String {
    // Remove whitespace from both ends of a string!
    input.trim().to_string()
}

// 直接format!能一次性拼接，而不是用push_str
// fn compose_me(input: &str) -> String {
//     // Add " world!" to the string! There's multiple ways to do this!
//     let mut s = input.to_string();
//     s.push_str(" world!");
//     s
// }

fn compose_me(input: &str) -> String {
    // Add " world!" to the string! There's multiple ways to do this!
    format!("{} world!", input)
}

// 原本这么实现，虽然结果一样，但复杂了，replace是str/String都有的
// fn replace_me(input: &str) -> String {
//     // Replace "cars" in the string with "balloons"!
//     let s = input.to_string();
//     let s = s.replace("cars", "balloons");
//     s
// }

fn replace_me(input: &str) -> String {
    // Replace "cars" in the string with "balloons"!
    input.replace("cars", "balloons")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trim_a_string() {
        assert_eq!(trim_me("Hello!     "), "Hello!");
        assert_eq!(trim_me("  What's up!"), "What's up!");
        assert_eq!(trim_me("   Hola!  "), "Hola!");
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
