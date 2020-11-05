#[cfg(feature = "boi")]
pub fn print_boi() {
    println!("boi")
}

#[cfg(feature = "foo")]
pub fn print_foo() {
    println!("foo")
}

#[cfg(feature = "bar")]
pub fn print_bar() {
    println!("bar")
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
