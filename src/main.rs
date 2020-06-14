use infer;
use reqwest;
use scraper::{Html, Selector};
use std::fs::File;
use std::{format, io};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::blocking::get(
        "https://towardsdatascience.com/image-scraping-with-python-a96feda8af2d",
    )?
    .text()?;
    let document = Html::parse_document(&resp);
    let img_selector = Selector::parse("img").unwrap();
    for img_node in document.select(&img_selector) {
        if let Some(url) = img_node.value().attr("src") {
            println!("downloading {}", url);
            let img_bytes = reqwest::blocking::get(url)?.bytes()?;
            let img_extension = infer::Infer::new()
                .get(&img_bytes)
                .unwrap_or(infer::Type {
                    mime: String::from("image/png"),
                    ext: String::from("png"),
                })
                .ext;
            let pic_name = url
                .split("/")
                .last()
                .and_then(|x| x.split(".").next())
                .unwrap_or("nameless");
            let mut file =
                File::create(format!("downloads/pic_{}.{}", pic_name, img_extension)).unwrap();
            io::copy(&mut img_bytes.as_ref(), &mut file).unwrap();
            println!("dowloaded {}", url);
        }
    }
    Ok(())
}
