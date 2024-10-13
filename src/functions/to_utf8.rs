use std::path::PathBuf;

use assert2::assert;
use clap::{arg, ArgMatches, Command};
use die_exit::Die;

pub fn command() -> Command {
    Command::new("toutf8")
        .about("auto convert all files to utf-8 encoding")
        .arg(arg!(<dir_or_file> "the dir or file to convert"))
}

pub fn execute(args: &ArgMatches) {
    let path = args
        .get_one::<PathBuf>("dir_or_file")
        .die("input not a valid path");
    assert!(path.exists(), "path not exists: {:?}", path);
}

mod test {
    use uchardet::detect_encoding_name;

    use super::*;

    #[test]
    fn test_detect_encoding_name() -> anyhow::Result<()> {
        assert_eq!(
            "UTF-8",
            detect_encoding_name("©français".as_bytes()).unwrap()
        );
        assert_eq!(
            "GB18030",
            detect_encoding_name(&std::fs::read("test_assets/gbk_encoding.txt")?).unwrap()
        );
        Ok(())
    }
}
