use std::{collections::HashMap, io::Cursor};

use dioxus::{logger::tracing, prelude::*};
use image::ImageReader;
use rxing::{
    common::HybridBinarizer, BinaryBitmap, BufferedImageLuminanceSource, MultiFormatReader, Reader,
};
use serde_json::{json, Value};

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
                                if let Some(file) = file_engine.read_file(file_name).await {
                                    let image = ImageReader::new(Cursor::new(file))
                                        .with_guessed_format()
                                        .expect("ASD")
                                        .decode()
                                        .expect("PEPE");
                                    let image = BufferedImageLuminanceSource::new(image);
                                    let mut bitmap = BinaryBitmap::new(HybridBinarizer::new(image));
                                    let mut multi_format_reader = MultiFormatReader::default();
                                    let result = multi_format_reader
                                        .decode(&mut bitmap)
                                        .expect("DECODE FAILED");
                                    tracing::info!(
                                        "BarCode: {} - {}", result.getBarcodeFormat(), result
                                        .getText()
                                    );
                                }
                            }
                        }
                    }
                },
            }
        }
    }
}
