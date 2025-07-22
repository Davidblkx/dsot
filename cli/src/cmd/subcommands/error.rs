use bakunin_config::BakuninError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SubCommandError {
    pub message: Option<String>,
    pub code: Option<i32>,
}

macro_rules! error_codes {
    ($($id:ident:$code:literal$(, $message:literal)?),*$(,)?) => {
        $(
            impl SubCommandError {
                #[allow(non_snake_case)]
                pub fn $id() -> Self {
                    SubCommandError::new()
                        $(.with_message($message))?
                        .with_code($code)
                }
            }
        )*
    };
}

impl SubCommandError {
    pub fn new() -> Self {
        SubCommandError {
            message: None,
            code: None,
        }
    }

    pub fn with_message<T: Into<String>>(mut self, message: T) -> Self {
        self.message = Some(message.into());
        self
    }

    pub fn with_code(mut self, code: i32) -> Self {
        self.code = Some(code);
        self
    }

    pub fn to_err(self) -> Result<(), Self> {
        Err(self)
    }

    pub fn from_bakunin_error(e: BakuninError) -> Self {
        SubCommandError::ConfigError().with_message(e.to_string())
    }
}

impl Default for SubCommandError {
    fn default() -> Self {
        SubCommandError {
            message: None,
            code: None,
        }
    }
}

error_codes![
    InvalidCommand: 1, "No valid command provided. Use --help for more information.",
    MissingArgument: 2,
    ConfigError: 3,
];
