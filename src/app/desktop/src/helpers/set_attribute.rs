#[macro_export]
macro_rules! set_attribute {
    ($attr:expr, $var: expr) => {
        use_effect(move || {
            let js_eval = document::eval(&format!(
                "document.documentElement.setAttribute('{}', '{}');",
                $attr, $var
            ));

            spawn(async move {
                match js_eval.await {
                    Ok(_) => {}
                    Err(e) => ::log::warn!("Failed to set document attribute: {}", e),
                }
            });
        });
    };
}
