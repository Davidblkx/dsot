pub fn print_message(args: &clap::ArgMatches, text: impl Into<String>) {
    let is_quiet = crate::cmd::QuietModeArg::enabled(args);
    let use_ansi: bool = !crate::cmd::DisableAnsiArg::enabled(args);

    if is_quiet {
        log::trace!("quiet: {}", text.into());
        return;
    }

    let mut message = text.into();
    if use_ansi {
        // TODO: implement ansi code system
        message = message;
    }
    println!("{message}");
}
