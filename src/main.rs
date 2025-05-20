use dioxus::prelude::*;
const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/styling/main.css");
mod components;
use components::graph::MyGraph;
fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    use_effect(move || {
        let eval = document::eval(
            r#"
            if (!document.getElementById("spectrum-web-components-bundle")) {
                const script = document.createElement('script');
                script.id = "spectrum-web-components-bundle";
                script.type = 'module';
                script.src = 'https://jspm.dev/@spectrum-web-components/bundle/elements.js';
                document.head.appendChild(script);
            }
            "#,
        );
    });

    rsx! {
        link { rel: "icon", href: FAVICON }
        link { rel: "stylesheet", href: MAIN_CSS }

        sp-theme {
            color: "dark",
            scale: "medium",
            style: "min-height: 100vh; display: flex; flex-direction: column;",

            header {
                style: "padding: 1rem; border-bottom: 1px solid var(--spectrum-gray-200); display: flex; justify-content: space-between; align-items: center;",
                sp-action-button {
                    quiet: true,
                    href: "#adobe",
                    "Adobe"
                }
                nav {
                    sp-button { variant: "primary", "Sign In" }
                }
            }

            main {
                style: "flex-grow: 1; padding: 2rem;",
                h1 { "Welcome to Dioxus with Spectrum!" }
                p { "This is a basic layout using Spectrum Web Components." }
                sp-button {
                    "Click Me!"
                }
                sp-divider { style: "margin-top: 1rem; margin-bottom: 1rem;" }
                sp-checkbox {
                    checked: true,
                    "Enable awesome feature"
                }
                MyGraph {}
            }

            footer {
                style: "padding: 1rem; border-top: 1px solid var(--spectrum-gray-200); text-align: center;",
                sp-label {
                    size: "s",
                    "© 2024 Dioxus-Spectrum App. All rights reserved."
                }
            }
        }
    }
}
