use dioxus::prelude::*; 
#[component]
pub fn Graph() -> Element {

    rsx! {
        canvas {
            width: "1280",
            height: "720",
        }
    }
}



