#[cfg(test)]
mod tests {
    // cargo build
    // cargo test
    extern crate Phrases;
    #[test]
    fn english_greeting_correct() {
        assert_eq!("hello", Phrases::greetings::english::hello());
    }


}