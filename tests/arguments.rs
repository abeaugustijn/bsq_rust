#[cfg(test)]
mod arguments {
    use bsq::*;

    #[test]
    fn basic() {
        let args = [String::from("execname"), String::from("filename")];
        let config = Config::new(&args).unwrap();
        assert_eq!(config.filename, Some(String::from("filename")));
    }

    #[test]
    fn stdin() {
        let args = [String::from("execname")];
        let config = Config::new(&args).unwrap();
        assert_eq!(config.filename, None);
    }

    #[test]
    fn error() {
        let args = [];
        match Config::new(&args) {
            Ok(_) => assert!(false),
            Err(e) => assert_eq!(e, "invalid amount of arguments"),
        };
    }
}
