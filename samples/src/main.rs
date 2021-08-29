
fn main() {
    println!("{}", print_message("Gabriel".to_string(), "Gonçalves".to_string()));
}

pub fn print_message(first_name: String, last_name: String) -> String {
    format!("Hello {firstName} {lastName}", firstName=first_name, lastName=last_name)
}

#[cfg(test)]
mod tests {
    use crate::print_message;
    
    #[test]
    fn should_return_full_name() {
        let first_name = "Gabriel";
        let last_name = "Gonçalves";
        let result = print_message(first_name.to_string(), last_name.to_string());
        assert_eq!("Hello Gabriel Gonçalves", result);
    }
}