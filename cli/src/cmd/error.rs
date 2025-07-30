//! This module contains all logic for error reported by this app

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct AppError {
    pub code: Option<usize>,
    pub message: Option<String>,
}

pub type AppResult<T> = Result<T, AppError>;

impl AppError {
    pub fn new() -> Self {
        Self {
            message: None,
            code: None,
        }
    }

    pub fn with_message<T: Into<String>>(mut self, message: T) -> Self {
        self.message = Some(message.into());
        self
    }

    pub fn with_code(mut self, code: usize) -> Self {
        self.code = Some(code);
        self
    }

    pub fn to_err<T>(self) -> AppResult<T> {
        Err(self)
    }
}

macro_rules! error_codes {
    ($($id:ident:$code:literal$(, $message:literal)?),*$(,)?) => {
        $(
            impl AppError {
                #[allow(non_snake_case)]
                pub fn $id() -> Self {
                    AppError::new()
                        $(.with_message($message))?
                        .with_code($code)
                }
            }
        )*
    };
}

error_codes![
    InvalidCommand: 1, "No valid command provided. Use --help for more information.",
    MissingArgument: 2,
    ConfigError: 3,
    RuntimeError: 4,
];

impl From<dsot_runtime::error::RuntimeError> for AppError {
    fn from(value: dsot_runtime::error::RuntimeError) -> Self {
        AppError::RuntimeError().with_message(value.to_string())
    }
}

impl From<bakunin_config::BakuninError> for AppError {
    fn from(value: bakunin_config::BakuninError) -> Self {
        AppError::ConfigError().with_message(value.to_string())
    }
}
