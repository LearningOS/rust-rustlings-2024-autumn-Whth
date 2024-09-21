// lifetimes2.rs
//
// So if the compiler is just validating the references passed to the annotated
// parameters and the return type, what do we need to change?
//
// Execute `rustlings hint lifetimes2` or use the `hint` watch subcommand for a
// hint.

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    // the String is a mutable obj, by that compiler will not be able to check which one is longer during pre-compile stage
    // to solve this, use either str or align the lifetime of them both

    let string1 = "long string is long";
    let result;
    {
        let string2 = "xyz";
        result = longest(string1, string2);
    }
    println!("The longest string is '{}'", result);

    let string1 = String::from("long string is long");
    let result;
    let string2 = String::from("xyz");
    {
        result = longest(string1.as_str(), string2.as_str());
    }
    println!("The longest string is '{}'", result);
}
