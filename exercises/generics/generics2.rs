// generics2.rs
//
// This powerful wrapper provides the ability to store a positive integer value.
// Rewrite it using generics so that it supports wrapping ANY type.
//
// Execute `rustlings hint generics2` or use the `hint` watch subcommand for a
// hint.

struct Wrapper<T> {
    value: T,
}

impl<T> Wrapper<T> {
    // 新建一个 Wrapper 实例
    pub fn new(value: T) -> Self {
        Wrapper { value }
    }
}

// 示例使用
fn main() {
    let int_wrapper = Wrapper::new(42);
    let str_wrapper = Wrapper::new(String::from("Hello, world!"));
    let float_wrapper = Wrapper::new(3.14);

    println!("Integer Wrapper: {:?}", int_wrapper.value);
    println!("String Wrapper: {:?}", str_wrapper.value);
    println!("Float Wrapper: {:?}", float_wrapper.value);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn store_u32_in_wrapper() {
        assert_eq!(Wrapper::new(42).value, 42);
    }

    #[test]
    fn store_str_in_wrapper() {
        assert_eq!(Wrapper::new("Foo").value, "Foo");
    }
}
