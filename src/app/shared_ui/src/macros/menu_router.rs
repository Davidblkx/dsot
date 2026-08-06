#[macro_export]
macro_rules! declare_router_menu {
    ($router:ident {
        layout: $layout:expr,
        $(
            $path:literal => $name:ident $( menu: $icon:ident $title:literal )?
        ),* $(,)?
    }) => {
        use ::dioxus::prelude::*;

        #[derive(Debug, Clone, Routable, PartialEq)]
        #[rustfmt::skip]
        pub enum $router {
            #[layout($layout)]
            $(
                #[route($path)]
                $name,
            )*
        }

        ::paste::paste! {
            #[component]
            pub fn [<$router Menu>]() -> Element {
                use dioxus_free_icons::Icon;
                use $crate::components::{Menu, MenuItem};

                let nav = use_navigator();
                let current_route = use_route::<$router>();

                rsx! {
                    Menu {
                        $(
                            $(
                                MenuItem {
                                    title: $title,
                                    active: current_route == $router::$name,
                                    click: move |_| { nav.push($router::$name); },
                                    icon: rsx! {
                                        Icon {
                                            icon: $icon
                                        }
                                    }
                                }
                            )?
                        )*
                    }
                }
            }
        }
    };
}
