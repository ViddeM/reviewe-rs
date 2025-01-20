use dioxus::prelude::*;

const SCANNER_CSS: Asset = asset!("/assets/styling/scanner.css");

#[component]
pub fn Scanner() -> Element {
    let mut s = use_signal(|| "image".to_string());
    println!("LELE");

    rsx! {
        document::Link { rel: "stylesheet", href: SCANNER_CSS }

        div { id: "scanner",
            input {
                id: "image_upload",
                r#type: "file",
                accept: "image/*",
                onchange: move |evt| {
                    async move {
                        if let Some(file_engine) = evt.files() {
                            let files = file_engine.files();
                            for file_name in &files {
                                if let Some(file) = file_engine.read_file_to_string(file_name).await
                                {
                                    println!("FILE: {file:?}");
                                }
                            }
                        }
                    }
                },
            }
        }
    }
}
