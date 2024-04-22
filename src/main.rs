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
    #[arg(short, long, required = true)]
    filename: String,
}

pub mod wc {
    pub fn byte_count(filename: &str) -> u64 {
        let contents = std::fs::File::open(filename)
            .expect("Could not open file.");
        contents.metadata().unwrap().len()
    }

    pub fn line_count(filename: &str) -> u64 {
        let contents = std::fs::read_to_string(filename)
            .expect("Could not open file.");
        contents.lines().count() as u64
    }

    pub fn word_count(filename: &str) -> u64 {
        let contents = std::fs::read_to_string(filename)
            .expect("Could not open file.");
        contents.split_whitespace().count() as u64
    }

    pub fn character_count(filename: &str) -> u64 {
        let contents = std::fs::read_to_string(filename)
            .expect("Could not open file.");
        contents.chars().count() as u64
    }
}

fn main() {
    let mut args = Args::parse();
    // If no options are provided, default to -clw.
    if !args.c_option && !args.l_option && !args.w_option && !args.m_option {
        args.c_option = true;
        args.l_option = true;
        args.w_option = true;
    }

    if args.c_option {
        print!("{}\t", wc::byte_count(args.filename.as_str()));
    }
    if args.l_option {
        print!("{}\t", wc::line_count(args.filename.as_str()));
    }
    if args.w_option {
        print!("{}\t", wc::word_count(args.filename.as_str()));
    }
    if args.m_option {
        print!("{}\t", wc::character_count(args.filename.as_str()));
    }
    print!("{}", args.filename);
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
            filename: "test.txt".to_string(),
        };
        assert_eq!(wc::byte_count(args.filename.as_str()), 342190);
    }

    #[test]
    fn test_line_count() {
        let args = Args {
            c_option: false,
            l_option: true,
            m_option: false,
            w_option: false,
            filename: "test.txt".to_string(),
        };
        assert_eq!(wc::line_count(args.filename.as_str()), 7145);
    }

    #[test]
    fn test_word_count() {
        let args = Args {
            c_option: false,
            l_option: false,
            m_option: false,
            w_option: true,
            filename: "test.txt".to_string(),
        };
        assert_eq!(wc::word_count(args.filename.as_str()), 58164);
    }

    #[test]
    fn test_character_count() {
        let args = Args {
            c_option: false,
            l_option: false,
            m_option: true,
            w_option: false,
            filename: "test.txt".to_string(),
        };
        assert_eq!(wc::character_count(args.filename.as_str()), 339292);
    }
}