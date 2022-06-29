mod strings {
    pub struct Alphabet<'a> {
        language: &'a str,
        characters: Vec<&'a str>,
    }

    impl Alphabet<'static> {
        pub fn new() -> Alphabet<'static> {
            Alphabet {
                language: String::from("English").as_str(),
                characters: vec!["ABCDEFGHIJKLMNOPQRSTUVWXYZ"]
            }
        }
        pub fn nth(n: usize) -> &'static str {
            let alpha = Alphabet::new();
            let chars: Vec<&str> = alpha.characters;
            chars[n]
        }
    }
}

pub fn find_nth_character(n: usize) -> &'static str {
    let char = strings::Alphabet::nth(n);
    char
}

pub fn longest_str<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}