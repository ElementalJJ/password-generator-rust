pub mod password_generator {
    use rand::distributions::Alphanumeric;
    use rand::rngs::StdRng;
    use rand::{Rng, SeedableRng};

    #[derive(Debug)]
    pub struct Password {
        prefix: String,
        length: u8,
        password: String,
    }

    fn characters(password: &mut Password) -> String {
        let mut rng: StdRng = StdRng::from_entropy();

        let chars: String = (0..(password.length - (1 + password.prefix.len() as u8)))
            .map(|_| rng.sample(Alphanumeric) as char)
            .collect();

        chars
    }

    impl Password {
        pub fn new(prefix: &str, length: &u8) -> Password {
            Password {
                prefix: prefix.trim().to_string(),
                length: length.to_owned(),
                password: String::new(),
            }
        }

        pub fn generate(&mut self) -> String {
            self.password = self.prefix.to_owned() + "." + &characters(self);

            self.password.to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::password_generator::Password;

    #[test]
    fn password_exists() {
        let mut password: Password = Password::new("test", &64_u8);
        assert_ne!(password.generate(), "");
    }

    #[test]
    fn password_prefix() {
        let mut password: Password = Password::new("test", &64_u8);
        assert_eq!(password.generate().get(0..4).unwrap(), "test");
    }

    #[test]
    fn password_separator() {
        let mut password: Password = Password::new("test", &64_u8);
        assert_eq!(password.generate().get(4..5).unwrap(), ".");
    }

    #[test]
    fn password_length() {
        let mut password: Password = Password::new("test", &64_u8);
        assert_eq!(password.generate().len(), 64);
    }
}
