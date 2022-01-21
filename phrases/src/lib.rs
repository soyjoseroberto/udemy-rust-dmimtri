pub mod greetings {
    pub mod english {
        pub fn hello() -> String { "hello".to_string() }
        pub fn goodbye() -> String { "goodbye".to_string() }

    }
    
    pub mod french {
        pub fn hello() -> String { "bonjour".to_string() }
        pub fn goodbye() -> String { "au revoir".to_string() }
    }
}

// Testing: Run 'cargo test' to see testing results
// #[should_panic] when a failing test is expected
// #[ignore] simply ignores the test
// You can create a test folder at the same level as src
// #[cfg(test)] added to the mod tests if you move to its own folder
#[test]
fn english_greeting_correct() {
    assert_eq!("hello", greetings::english::hello());
}