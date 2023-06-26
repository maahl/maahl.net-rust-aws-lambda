// Send a greeting
fn say_hello(name: Option<&str>) -> String {
    // if a name was not provided, address the greeting to "stranger"
    let name = name.unwrap_or("stranger");

    format!("Hello, {name}!")
}

fn main() {
    let response = say_hello(Some("world"));
    println!("{}", response);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name_provided() {
        let name = "world";
        let result = say_hello(Some(name));
        assert_eq!(format!("Hello, {name}!"), result);
    }

    #[test]
    fn test_no_name_provided() {
        let result = say_hello(None);
        assert_eq!("Hello, stranger!".to_owned(), result);
    }
}
