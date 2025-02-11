use {
    getargs::{Opt, Options},
    std::{
        collections::HashMap,
        fmt::{Display, Formatter},
    },
    unicode_width::UnicodeWidthStr,
};

type FlagOperation = fn(
    config: &mut Config,
    arg: Option<&'static str>,
    flag: Opt<&'static str>,
) -> Result<(), FlagError>;

/// Configuration from command line flags.
#[derive(Clone, Copy, Debug)]
pub struct Config {
    /// The suggested [device][cpal::platform::Device] to use.
    pub device: Option<&'static str>,
    /// Whether or not to quit after parsing.
    pub quit: bool,
}
impl Config {
    pub fn new() -> Result<Self, FlagError> {
        let mut opts =
            Options::<&'static str, _>::new(argv::iter().skip(1).flat_map(|arg| arg.to_str()));

        let opt_lookup: HashMap<Opt<&'static str>, FlagOperation> = HashMap::from_iter(
            FLAGS
                .iter()
                .map(|flag| (flag.to_opts(), flag.operation))
                .flat_map(|(opts, operation)| {
                    [(opts[0], operation), (opts[1], operation)].into_iter()
                }),
        );
        let mut out = Self {
            device: None,
            quit: false,
        };

        while let Ok(Some(opt)) = opts.next_opt() {
            (opt_lookup
                .get(&opt)
                .ok_or(FlagErrorKind::Unknown.new(opt))?)(
                &mut out, opts.value_opt(), opt
            )?;

            if out.quit {
                break;
            }
        }

        Ok(out)
    }
}

#[derive(Clone, Copy)]
struct Flag {
    short: char,
    long: &'static str,
    /// The message that gets printed for `--help`.
    help: &'static str,
    /// A callback that gets executed if a flag matches this flag.
    operation: FlagOperation,
}
impl Flag {
    /// Only prints the flag help section, not the entire help.
    pub fn print_help(flags: &[Self]) {
        fn create_padding(n: usize) -> String {
            String::from_iter((0..n).map(|_| ' '))
        }

        let max_long = flags
            .iter()
            .map(|flag| flag.long.width())
            .max()
            .unwrap_or_default();

        flags.iter().for_each(|flag| {
            let padding = create_padding(max_long - flag.long.width());
            let mut line = format!(" -{} --{}{} ", flag.short, flag.long, padding);
            let line_start_len = line.width();
            let mut line_padding = None;

            let mut current_x = 0;

            for word in flag.help.split(' ') {
                line.push_str(word);
                line.push(' ');
                current_x += word.width();

                if current_x > 40 {
                    line.push('\n');
                    let line_padding = match line_padding {
                        Some(ref padding) => padding,
                        None => {
                            line_padding = Some(create_padding(line_start_len));
                            line_padding.as_ref().unwrap()
                        }
                    };
                    line.push_str(line_padding);
                    current_x = 0;
                }
            }

            println!("{}", line);
        });
    }
    /// Convert to both the short and long [flags][Opt]
    pub const fn to_opts(self) -> [Opt<&'static str>; 2] {
        [Opt::Short(self.short), Opt::Long(self.long)]
    }
}
/// All allowed flags
const FLAGS: &[Flag] = &[
    FlagBuilder::new()
        .short('d')
        .long("device")
        .help("Change the device used to play audio.")
        .operation(|config, device, flag| {
            config.device = Some(device.ok_or(FlagErrorKind::Missing.new(flag))?);
            Ok(())
        })
        .build(),
    FlagBuilder::new()
        .short('h')
        .long("help")
        .help("Print this message and exit.")
        .operation(|config, _, _| {
            println!(
                "Usage: fplayer [OPTIONS]...

Options:"
            );
            Flag::print_help(FLAGS);
            config.quit = true;
            Ok(())
        })
        .build(),
    FlagBuilder::new()
        .short('v')
        .long("version")
        .help("Print version information and exit.")
        .operation(|config, _, _| {
            println!(concat!(
                env!("CARGO_PKG_NAME"),
                " ",
                env!("CARGO_PKG_VERSION")
            ));
            config.quit = true;
            Ok(())
        })
        .build(),
];

/// Builder for [Flag]
struct FlagBuilder {
    short: Option<char>,
    long: Option<&'static str>,
    help: Option<&'static str>,
    operation: Option<FlagOperation>,
}
impl FlagBuilder {
    /// Constructs the [Flag]
    ///
    /// # Panics
    ///
    /// Panics if any of the fields are not initalized.
    pub const fn build(self) -> Flag {
        Flag {
            short: self.short.unwrap(),
            long: self.long.unwrap(),
            help: self.help.unwrap(),
            operation: self.operation.unwrap(),
        }
    }
    /// Creates an empty instance
    pub const fn new() -> Self {
        Self {
            short: None,
            long: None,
            help: None,
            operation: None,
        }
    }
    /// Set [Self::short]
    pub const fn short(mut self, short: char) -> Self {
        self.short = Some(short);
        self
    }
    /// Set [Self::long]
    pub const fn long(mut self, long: &'static str) -> Self {
        self.long = Some(long);
        self
    }
    /// Set [Self::help]
    pub const fn help(mut self, help: &'static str) -> Self {
        self.help = Some(help);
        self
    }
    /// Set [Self::operation]
    pub const fn operation(mut self, operation: FlagOperation) -> Self {
        self.operation = Some(operation);
        self
    }
}

#[derive(Clone, Copy, Debug)]
pub struct FlagError {
    flag: Opt<&'static str>,
    kind: FlagErrorKind,
}
impl Display for FlagError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self.kind {
            FlagErrorKind::Missing => {
                write!(f, "Flag `{}` is missing a required argument", self.flag)
            }
            FlagErrorKind::Unknown => write!(
                f,
                "Unknown flag `{}`. See `rsplayer --help` for flags.",
                self.flag
            ),
        }
    }
}
impl std::error::Error for FlagError {}

#[derive(Clone, Copy, Debug)]
enum FlagErrorKind {
    Missing,
    Unknown,
}
impl FlagErrorKind {
    pub const fn new(self, flag: Opt<&'static str>) -> FlagError {
        FlagError { flag, kind: self }
    }
}
