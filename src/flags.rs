use {
    getargs::{Opt, Options},
    std::collections::HashMap,
    thiserror::Error,
    unicode_width::UnicodeWidthStr,
};

type FlagOperation = fn(config: &mut Config, arg: Option<&'static str>, flag: Opt<&'static str>) -> Result<(), FlagOperationError>;

#[derive(Clone, Copy, Debug)]
pub struct Config {
    /// The suggested [device][cpal::platform::Device] to use.
    device: Option<&'static str>,
    /// Whether or not to quit after parsing.
    quit: bool,
}
impl Config {
    pub fn new() -> Result<Self, FlagOperationError> {
        let mut opts = Options::<&'static str, _>::new(argv::iter().skip(1).flat_map(|arg| arg.to_str()));

        let opt_lookup: HashMap<
            Opt<&'static str>,
            FlagOperation,
        > = HashMap::from_iter(
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
            if let Some(operation) = opt_lookup.get(&opt) {
                operation(&mut out, opts.value_opt(), opt)?;
                if out.quit {
                    break;
                }
            }
        }

        Ok(out)
    }
}

#[derive(Clone, Copy)]
struct Flag {
    short: char,
    long: &'static str,
    help: &'static str,
    operation: FlagOperation,
}
impl Flag {
    /// Only prints the flag help section, not the entire help.
    pub fn print_help(flags: &[Self]) {
        fn create_padding(n: usize) -> String {
            String::from_iter((0..n).map(|_| ' '))
        }

        let max_long = flags.iter()
            .map(|flag| flag.long.width())
            .max()
            .unwrap_or_default();

        flags.iter()
            .for_each(|flag| {
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
    pub const fn to_opts(self) -> [Opt<&'static str>; 2] {
        [Opt::Short(self.short), Opt::Long(self.long)]
    }
}
const FLAGS: &[Flag] = &[
    FlagBuilder::new()
        .short('d')
        .long("device")
        .help("Change the device used to play audio.")
        .operation(|config, device, flag| {
            config.device = Some(device.ok_or(FlagOperationError::MissingOption(flag))?);
            Ok(())
        })
        .build(),
    FlagBuilder::new()
        .short('h')
        .long("help")
        .help("Print this message and exit.")
        .operation(|config, _, _| {
            println!("Usage: fplayer [OPTIONS]...

Options:");
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
            println!(concat!(env!("CARGO_PKG_NAME"), " ", env!("CARGO_PKG_VERSION")));
            config.quit = true;
            Ok(())
        })
        .build(),
];

struct FlagBuilder {
    short: Option<char>,
    long: Option<&'static str>,
    help: Option<&'static str>,
    operation: Option<FlagOperation>,
}
impl FlagBuilder {
    pub const fn build(self) -> Flag {
        Flag {
            short: self.short.unwrap(),
            long: self.long.unwrap(),
            help: self.help.unwrap(),
            operation: self.operation.unwrap(),
        }
    }
    pub const fn new() -> Self {
        Self {
            short: None,
            long: None,
            help: None,
            operation: None,
        }
    }
    pub const fn short(mut self, short: char) -> Self {
        self.short = Some(short);
        self
    }
    pub const fn long(mut self, long: &'static str) -> Self {
        self.long = Some(long);
        self
    }
    pub const fn help(mut self, help: &'static str) -> Self {
        self.help = Some(help);
        self
    }
    pub const fn operation(
        mut self,
        operation: FlagOperation,
    ) -> Self {
        self.operation = Some(operation);
        self
    }
}

#[derive(Debug, Error)]
pub enum FlagOperationError {
    #[error("Flag `{0}` is missing a required argument.")]
    MissingOption(Opt<&'static str>),
}
