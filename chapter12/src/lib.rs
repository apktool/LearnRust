pub mod cfg {
    pub struct Config {
        pub query: String,
        pub file_path: String,
    }

    impl Config {
        pub fn build(args: &[String]) -> Result<Config, &'static str> {
            if args.len() < 3 {
                return Err("not enough arguments");
            }

            let query = args[1].clone();
            let file_path = args[2].clone();

            Ok(Config { query, file_path })
        }
    }
}

pub mod minigrep {
    use std::error::Error;
    use std::fs;
    use crate::cfg::Config;

    pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
        let mut result = Vec::new();
        let query = query.to_lowercase();

        for line in contents.lines() {
            if line.to_lowercase().contains(&query) {
                result.push(line);
            }
        }
        result
    }


    pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
        let mut result = Vec::new();

        for line in contents.lines() {
            if line.contains(&query) {
                result.push(line);
            }
        }
        result
    }

    pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
        let contents = fs::read_to_string(config.file_path)?;
        for line in search(&config.query, &contents) {
            println!("{line}");
        }
        Ok(())
    }
}