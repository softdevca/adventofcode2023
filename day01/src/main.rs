fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn foo() {
        panic!("Test ran");
    }
}
