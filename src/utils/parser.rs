pub fn parse_input(input: &str) -> Result<String, String> {
    match input {
        "" => Err("Input is empty".to_string()),
        _ => Ok(input.to_string()),
    }
}