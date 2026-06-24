use dioxus::prelude::*;

#[derive(Debug, Props, PartialEq, Clone)]
pub struct ButtonFooterProps {
    pub name: Option<String>,
    #[props(default = "".to_string())]
    pub tooltip: String,
    pub click: EventHandler,
    pub icon: Option<Element>,
    #[props(default = Signal::new(false))]
    pub active: Signal<bool>,
}

#[component]
pub fn ButtonFooter(props: ButtonFooterProps) -> Element {
    let name = if let Some(n) = &props.name {
        rsx! { span { class: "text", "{n}" } }
    } else {
        rsx! {}
    };

    let icon = if let Some(i) = &props.icon {
        let inner_icon = i.to_owned();
        rsx! { span { class: "icon", {inner_icon}  } }
    } else {
        rsx! {}
    };

    let status = if *props.active.read() {
        "active"
    } else {
        "inactive"
    };

    rsx! {
        button {
            "data-component": "button_footer",
            "data-status": "{status}",
            "title": props.tooltip,
            onclick: move |_| {
                props.click.call(());
            },
            {icon}
            {name}
        }
    }
}
