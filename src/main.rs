use dioxus::prelude::*;
const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/styling/main.css");
mod components;
use components::graph::MyGraph;
fn main() {
    // The `launch` function is the main entry point for a dioxus app. It takes a component and renders it with the platform feature
    // you have enabled
    dioxus::launch(App);
}

/// App is the main component of our app. Components are the building blocks of dioxus apps. Each component is a function
/// that takes some props and returns an Element. In this case, App takes no props because it is the root of our app.
///
/// Components should be annotated with `#[component]` to support props, better error messages, and autocomplete
#[component]
fn App() -> Element {
    // Use `use_effect` to load the Spectrum Web Components bundle when the component mounts.
    // `document()` here comes from `dioxus::prelude::*` which re-exports `dioxus_web::document()`.
    // `eval()` is a method on the document object from `dioxus_web`.
    use_effect(move || {
        // Use dioxus_web::eval to run JavaScript for loading Spectrum components.
        // Ensure `dioxus-web` is a dependency in Cargo.toml.
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
        ); // Assuming eval itself doesn't fail to initialize, unwrap for simplicity.
                    // A production app should handle this more gracefully.
        
        // We don't need to call send or recv for this one-shot script injection.
        // The eval runs when created. We can drop it if we don't need to interact further.
        drop(eval);
    });

    rsx! {
        // Standard HTML link tags for favicon and CSS
        link { rel: "icon", href: FAVICON }
        link { rel: "stylesheet", href: MAIN_CSS }

        // Use unquoted hyphenated tag names for web components.
        sp-theme {
            color: "dark",
            scale: "medium",
            style: "min-height: 100vh; display: flex; flex-direction: column;",

            // Header (using standard HTML tags for structure)
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

            // Main content area
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

            // Footer
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
