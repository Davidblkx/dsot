macro_rules! declare_arg_path {
    ($name:ident, $long:literal$(, short: $short:literal)?, $help:literal) => {
        pub struct $name;

        impl $name {
            /// The long name of the argument.
            pub fn get_name() -> &'static str {
                $long
            }

            /// Builds the clap argument.
            pub fn build() -> clap::Arg {
                clap::Arg::new(Self::get_name())
                    .long($long)
                    $(.short($short))?
                    .help($help)
                    .value_parser(clap::value_parser!(std::path::PathBuf))
                    .required(false)
            }

            /// Get the PathBuf value for this argument.
            pub fn get(args: &clap::ArgMatches) -> Option<&std::path::PathBuf> {
                args.get_one::<std::path::PathBuf>($long)
            }
        }
    };
}

macro_rules! declare_arg_bool {
    ($name:ident, $long:literal$(, short: $short:literal)?, $help:literal$(, action: $action:ident)?) => {
        pub struct $name;

        impl $name {
            /// The long name of the argument.
            pub fn get_name() -> &'static str {
                $long
            }

            /// Builds the clap argument.
            pub fn build() -> clap::Arg {
                clap::Arg::new(Self::get_name())
                    .long($long)
                    $(.short($short))?
                    .help($help)
                    .action(clap::ArgAction::SetTrue)
                    $(.action($action))?
                    .required(false)
            }

            /// Check if flag is enabled.
            pub fn enabled(args: &clap::ArgMatches) -> bool {
                args.get_flag($long)
            }
        }
    };
}
