#[cfg(test)]
mod tests {

    extern crate phrases;
    #[test]
    fn english_greeting_correct() {
        assert_eq!("Hello", phrases::greetings::english::hello());
    }
    #[test]
    fn french_greeting_correct() {
        assert_eq!("Bonjour", phrases::greetings::french::hello());
    }
    #[test]
    #[should_panic]
    fn french_greeting_incorrect() {
        assert_eq!("Bonjourt", phrases::greetings::french::hello());
    }
}
