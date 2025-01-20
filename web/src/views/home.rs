use dioxus::prelude::*;
use ui::Scanner;

#[component]
pub fn Home() -> Element {
    rsx! {
        Scanner {}
    }
}
