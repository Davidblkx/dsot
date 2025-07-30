use super::AppCommandContext;

impl AppCommandContext {
    pub fn print_message(&self, text: impl Into<String>) {
        let is_quiet = crate::cmd::QuietModeArg::enabled(&self.global);
        let use_ansi: bool = !crate::cmd::DisableAnsiArg::enabled(&self.global);

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
}
