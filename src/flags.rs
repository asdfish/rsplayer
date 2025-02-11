use {
    getargs::{Opt, Options},
    std::{
        collections::HashMap,
        fmt::{Display, Formatter},
    },
    unicode_width::UnicodeWidthStr,
};

mod list;

fn create_padding(n: usize) -> String {
    String::from_iter((0..n).map(|_| ' '))
}

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
                .ok_or(FlagError::new(opt, FlagErrorKind::Unknown))?)(
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

        let max_long = flags
            .iter()
            .map(|flag| flag.long.width())
            .max()
            .unwrap_or_default();

        flags.iter().for_each(|flag| {
            let padding = create_padding(max_long - flag.long.width());
            println!(" -{} --{}{} {}", flag.short, flag.long, padding, flag.help);
        });
    }
    /// Convert to both the short and long [flags][Opt]
    pub const fn to_opts(self) -> [Opt<&'static str>; 2] {
        [Opt::Short(self.short), Opt::Long(self.long)]
    }
}
/// All allowed flags
const FLAGS: &[Flag] = &[
    Flag {
        short: 'd',
        long: "device",
        help: "Change the device used to play audio.",
        operation: |config, device, flag| {
            config.device = Some(device.ok_or(FlagError::new(flag, FlagErrorKind::Missing))?);
            Ok(())
        },
    },
    Flag {
        short: 'h',
        long: "help",
        help: "Print this message and exit.",
        operation: |config, _, _| {
            println!(
                "Usage: fplayer [OPTIONS]... -l[LIST]...

Options:"
            );
            Flag::print_help(FLAGS);
            config.quit = true;
            Ok(())
        },
    },
    Flag {
        short: 'l',
        long: "list",
        help: "List all availiable options for a topic.",
        operation: |config, arg, flag| {
            let mut items = list::hash_map()
                .get(&arg)
                .ok_or(FlagError::new(flag, FlagErrorKind::UnknownArg { arg: arg, list: None }))?
                .1
                ();
            items.sort();
            items.into_iter()
                .for_each(|item| println!("{}", item));

            config.quit = true;
            Ok(())
        },
    },
    Flag {
        short: 'v',
        long: "version",
        help: "Print version information and exit.",
        operation: |config, _, _| {
            println!(concat!(
                env!("CARGO_PKG_NAME"),
                " ",
                env!("CARGO_PKG_VERSION")
            ));
            config.quit = true;
            Ok(())
        },
    },
];

#[derive(Clone, Copy, Debug)]
pub struct FlagError {
    flag: Opt<&'static str>,
    kind: FlagErrorKind,
}
impl FlagError {
    const fn new(flag: Opt<&'static str>, kind: FlagErrorKind) -> Self {
        #[cfg(debug_assertions)]
        {
            if let FlagErrorKind::UnknownArg { list, .. } = kind {
                debug_assert!(list::hash_map().contains_key(&list));
            }
        }

        Self {
            flag,
            kind,
        }
    }
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
            FlagErrorKind::UnknownArg { arg, list } => write!(f, "Unknown argument `{}` for flag `{}`. See `rsplayer -l{}` for valid arguments.", arg.unwrap_or_default(), self.flag, list.unwrap_or_default()),
        }
    }
}
impl std::error::Error for FlagError {}

#[derive(Clone, Copy, Debug)]
enum FlagErrorKind {
    Missing,
    Unknown,
    UnknownArg {
        arg: Option<&'static str>,
        list: Option<&'static str>,
    },
}
