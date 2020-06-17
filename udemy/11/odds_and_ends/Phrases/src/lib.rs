pub mod greetings {
    pub mod english; // takes from greetings/english.rs

    pub mod french {
        pub fn hello() -> String { "bonjour".to_string() }
        pub fn goodbye()->String {"au revoir".to_string() }
    }
}

#[test]
#[should_panic]
#[ignore] // to suppress temporarily
fn english_greeting_incorrect() {
    assert_eq!("hello1", greetings::english::hello());
}