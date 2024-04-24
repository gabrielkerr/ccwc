use clap::Parser;

/// Coding Challenges Word Count CLI tool.
#[derive(Parser)]
#[command(name = "ccwc")]
#[command(version = "1.0")]
#[command(about = "Command line utility for counting words.", long_about = None)]
struct Args {
    /// Count the number of bytes in the input.
    #[arg(short, default_value = "false")]
    c_option: bool,

    /// Count the number of lines in the input.
    #[arg(short, default_value = "false")]
    l_option: bool,

    /// Count the number of words in the input.
    #[arg(short, default_value = "false")]
    w_option: bool,

    /// Count the number of characters in the input.
    #[arg(short, default_value = "false")]
    m_option: bool,

    /// Path to the file to operate on.
    #[arg(short, long)]
    filename: Option<String>
}

pub mod wc {
    pub struct Content {
        contents: String,
    }

    impl Content {
        pub fn from_file(filename: &str) -> Content {
            let contents = std::fs::read_to_string(filename)
                .expect("Could not open file.");
            Content { contents }
        }

        pub fn from_stdin() -> Content {
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            Content { contents: input }
        }

        pub fn byte_count(&self) -> u64 {
            self.contents.len() as u64
        }

        pub fn line_count(&self) -> u64 {
            self.contents.lines().count() as u64
        }

        pub fn word_count(&self) -> u64 {
            self.contents.split_whitespace().count() as u64
        }

        pub fn character_count(&self) -> u64 {
            self.contents.chars().count() as u64
        }
    }
}

fn process_content(content: wc::Content, mut args: Args) {
    // If no options are provided, default to -clw.
    if !args.c_option && !args.l_option && !args.w_option && !args.m_option {
        args.c_option = true;
        args.l_option = true;
        args.w_option = true;
    }

    if args.c_option {
        print!("{}\t", content.byte_count());
    }
    if args.l_option {
        print!("{}\t", content.line_count());
    }
    if args.w_option {
        print!("{}\t", content.word_count());
    }
    if args.m_option {
        print!("{}\t", content.character_count());
    }
}

fn main() {
    let args = Args::parse();

    if let Some(ref filename) = args.filename {
        let cloned_filename = filename.clone();
        let content = wc::Content::from_file(&filename);
        process_content(content, args);
        print!("{}", cloned_filename);
    } else {
        // Process stdin if no file is provided.
        let content = wc::Content::from_stdin();
        process_content(content, args);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_byte_count() {
        let args = Args {
            c_option: true,
            l_option: false,
            m_option: false,
            w_option: false,
            filename: Option::from("test.txt".to_string()),
        };
        let content = wc::Content::from_file(args.filename.unwrap().as_str());
        assert_eq!(content.byte_count(), 342190);
    }

    #[test]
    fn test_line_count() {
        let args = Args {
            c_option: false,
            l_option: true,
            m_option: false,
            w_option: false,
            filename: Some("test.txt".to_string()),
        };
        let content = wc::Content::from_file(args.filename.unwrap().as_str());
        assert_eq!(content.line_count(), 7145);
    }

    #[test]
    fn test_word_count() {
        let args = Args {
            c_option: false,
            l_option: false,
            m_option: false,
            w_option: true,
            filename: Some("test.txt".to_string()),
        };
        let content = wc::Content::from_file(args.filename.unwrap().as_str());
        assert_eq!(content.word_count(), 58164);
    }

    #[test]
    fn test_character_count() {
        let args = Args {
            c_option: false,
            l_option: false,
            m_option: true,
            w_option: false,
            filename: Some("test.txt".to_string()),
        };
        let content = wc::Content::from_file(args.filename.unwrap().as_str());
        assert_eq!(content.character_count(), 339292);
    }
}