use structopt::StructOpt;
#[derive(StructOpt)]
pub struct Options {
    /// Specify directory: -d [directory]
    #[structopt(short = "d", long = "directory", default_value = ".\\")]
    pub directory: String,

    /// Specify key: -k [key]
    #[structopt(short = "k", long = "key", default_value = "foobar")]
    pub key: String,

    /// Specify output file: -o [filename]
    #[structopt(short = "o", long = "output", default_value = "output.txt")]
    pub output: String,
}

impl Options {
    /// Return the str slice inside of the object's key member.
    /// RegexBuilder::new() doesn't take String structs - can't use the self.key directly.
    pub fn get_key(&self) -> &str {
        &self.key[..]
    }
}