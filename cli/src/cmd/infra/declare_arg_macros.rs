macro_rules! __internal_arg_bool_declare_fn_enabled {
    ($long:literal, SetFalse) => {
        __internal_arg_bool_declare_fn_enabled! {$long, SetTrue}
    };
    ($long:literal, SetTrue) => {
        #[allow(unused)]
        /// Check is argument is enabled
        pub fn enabled(args: &clap::ArgMatches) -> bool {
            args.get_flag($long)
        }
    };
}

macro_rules! __internal_arg_bool_get_action {
    (SetTrue) => {
        clap::ArgAction::SetTrue
    };
    (SetFalse) => {
        clap::ArgAction::SetFalse
    };
}

macro_rules! __internal_arg_declare_get {
    ($long:literal, $type:ty) => {
        #[allow(unused)]
        pub fn get(args: &clap::ArgMatches) -> Option<&$type> {
            args.get_one::<$type>($long)
        }
    };
}

macro_rules! declare_arg {
    ($arg:ident($long_name:literal) {
        $(short: $short:literal,)?
        $(help: $help:literal,)?
        $(long_help: $long_help:literal,)?
        $(flag: $flag:tt,)?
        $(required: $required:literal,)?
        $(value: $value:ty,)?
    }) => {
        pub struct $arg;

        impl $arg {
            pub fn build() -> clap::Arg {
                clap::Arg::new($long_name)
                    .long($long_name)
                    .required(false)
                    $(.short($short))?
                    $(.help($help))?
                    $(.long_help($long_help))?
                    $(.action(__internal_arg_bool_get_action!{$flag}))?
                    $(.required($required))?
                    $(.value_parser(clap::value_parser!($value)))?
            }

            $(__internal_arg_bool_declare_fn_enabled!{$long_name, $flag})?

            $(__internal_arg_declare_get!{$long_name, $value})?
        }
    };
}

macro_rules! declare_arg_bool {
    ($arg:ident, $long:literal, $help:literal $(,$short:literal)?) => {
        declare_arg!($arg($long) {
            $(short: $short,)?
            help: $help,
            flag: SetTrue,
        });
    };
}

macro_rules! declare_arg_path {
    ($arg:ident, $long:literal, $help:literal $(,$short:literal)? $(, required: $req:literal)?) => {
        declare_arg!($arg($long) {
            $(short: $short,)?
            help: $help,
            $(required: $req,)?
            value: std::path::PathBuf,
        });
    };
}

macro_rules! declare_arg_string {
    ($arg:ident, $long:literal, $help:literal $(,$short:literal)? $(, required: $req:literal)?) => {
        declare_arg!($arg($long) {
            $(short: $short,)?
            help: $help,
            $(required: $req,)?
            value: String,
        });
    };
}

macro_rules! declare_arg_number {
    ($arg:ident($type:ty), $long:literal, $help:literal $(,$short:literal)? $(, required: $req:literal)?) => {
        declare_arg!($arg($long) {
            $(short: $short,)?
            help: $help,
            $(required: $req,)?
            value: $type,
        });
    };
}

#[cfg(test)]
mod tests {
    #[test]
    pub fn test_flag_false_arg() {
        declare_arg!(BoolArg("my_arg") {
            short: 'm',
            help: "help",
            long_help: "long_help",
            flag: SetFalse,
        });

        let test_args = clap::Command::new("cmd")
            .arg(BoolArg::build())
            .try_get_matches_from(["cmd"])
            .unwrap();

        assert!(BoolArg::enabled(&test_args));

        let test_args = clap::Command::new("cmd")
            .arg(BoolArg::build())
            .try_get_matches_from(["cmd", "-m"])
            .unwrap();

        assert!(!BoolArg::enabled(&test_args));
    }

    #[test]
    pub fn test_flag_true_arg() {
        declare_arg!(BoolArg("my_arg") {
            short: 'm',
            help: "help",
            long_help: "long_help",
            flag: SetTrue,
        });

        let test_args = clap::Command::new("cmd")
            .arg(BoolArg::build())
            .try_get_matches_from(["cmd", "-m"])
            .unwrap();

        assert!(BoolArg::enabled(&test_args));

        let test_args = clap::Command::new("cmd")
            .arg(BoolArg::build())
            .try_get_matches_from(["cmd"])
            .unwrap();

        assert!(!BoolArg::enabled(&test_args));
    }

    #[test]
    pub fn test_value_arg() {
        declare_arg!(PathArg("my_arg") {
            short: 'm',
            value: std::path::PathBuf,
        });

        let test_args = clap::Command::new("cmd")
            .arg(PathArg::build())
            .try_get_matches_from(["cmd", "-m", "/dir/file.txt"])
            .unwrap();

        assert_eq!(
            &std::path::PathBuf::from("/dir/file.txt"),
            PathArg::get(&test_args).unwrap()
        );
    }
}
