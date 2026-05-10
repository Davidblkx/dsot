use crate::error::{MusicBrainzError, Result};

pub fn parse<T>(json: String) -> Result<T>
where
    T: serde::de::DeserializeOwned,
{
    let maybe: std::result::Result<T, serde_json::Error> = serde_json::from_str(&json);
    match maybe {
        Ok(v) => Ok(v),
        Err(e) => {
            Err(get_error(json, e))
        }
    }
}

fn get_error(src: String, e: serde_json::Error) -> MusicBrainzError {
    if !e.is_data() && !e.is_syntax() {
        return MusicBrainzError::JsonError(e);
    }

    let line = e.line();
    let column = e.column();
    let error = e.to_string();

    let span_start = match column < 15 {
        true => 0,
        false => column - 15,
    };

    let span = src
        .split('\n')
        .collect::<Vec<&str>>()
        .get(line - 1)
        .map(|l| l.chars().skip(span_start).take(30).collect::<String>())
        .unwrap_or("".to_string());

    MusicBrainzError::JsonMappingError {
        span,
        line,
        column,
        error,
    }

}
