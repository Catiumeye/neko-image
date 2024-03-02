use clap::Arg;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref ARG_OUTPUT: Arg = Arg::new("output")
    .short('o')
    .long("output")
    .required(false)
    .help("Path to output");
}
