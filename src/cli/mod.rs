use clap::{crate_version, value_parser, Arg, ArgAction, Command};

#[derive(Clone, Debug)]
pub enum OutputErrorMode {
    Warn,
    WarnNoPipe,
    Exit,
    ExitNoPipe,
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Options {
    pub append: bool,
    pub files: Vec<String>,
    pub output_error: Option<OutputErrorMode>,
    pub rotate: bool,
    pub max_size_bytes: u64,
}

pub mod options {
    pub const APPEND: &str = "append";
    pub const FILE: &str = "file";
    pub const OUTPUT_ERROR: &str = "output-error";
    pub const ROTATE: &str = "rotate";
    pub const MAX_SIZE_BYTES: &str = "max-size-bytes";
}

fn command_factory() -> Command {
    Command::new("tee_rotate")
        .version(crate_version!())
        .about("Tee Rotate")
        .override_usage("cargo run -- -r -s 1000 -a test.log")
        .infer_long_args(true)
        // Since we use value-specific help texts for "--output-error", clap's "short help" and "long help" differ.
        // However, this is something that the GNU tests explicitly test for, so we *always* show the long help instead.
        .disable_help_flag(true)
        .arg(
            Arg::new("--help")
                .short('h')
                .long("help")
                .help("Print help")
                .action(ArgAction::HelpLong),
        )
        .arg(
            Arg::new(options::APPEND)
                .long(options::APPEND)
                .short('a')
                .help("Append to the given FILEs, do not overwrite")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new(options::ROTATE)
                .long(options::ROTATE)
                .short('r')
                .help("Rotate files with a maximum size of MAX_SIZE_BYTES")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new(options::MAX_SIZE_BYTES)
                .long(options::MAX_SIZE_BYTES)
                .short('s')
                .help("Maximum size of each file in bytes")
                .value_parser(value_parser!(u64)),
        )
        .arg(
            Arg::new(options::FILE)
                .action(ArgAction::Append)
                .value_hint(clap::ValueHint::FilePath),
        )
        .arg(
            Arg::new(options::OUTPUT_ERROR)
                .long(options::OUTPUT_ERROR)
                .require_equals(true)
                .num_args(0..=1)
                .value_parser(value_parser!(String))
                .help("Set write error behavior"),
        )
}

pub fn init() -> Options {
    let matches = command_factory().get_matches();

    Options {
        append: matches.get_flag(options::APPEND),
        files: matches
            .get_many::<String>(options::FILE)
            .map(|v| v.map(ToString::to_string).collect())
            .unwrap_or_default(),
        rotate: matches.get_flag(options::ROTATE),
        max_size_bytes: match matches.get_one::<u64>(options::MAX_SIZE_BYTES) {
            Some(v) => *v,
            None => 1000,
        },
        output_error: {
            if let Some(v) = matches.get_one::<String>(options::OUTPUT_ERROR) {
                match v.as_str() {
                    "warn" => Some(OutputErrorMode::Warn),
                    "warn-nopipe" => Some(OutputErrorMode::WarnNoPipe),
                    "exit" => Some(OutputErrorMode::Exit),
                    "exit-nopipe" => Some(OutputErrorMode::ExitNoPipe),
                    _ => unreachable!(),
                }
            } else {
                Some(OutputErrorMode::WarnNoPipe)
            }
        },
    }
}
