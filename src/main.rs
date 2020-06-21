use futures::future::join_all;
use infer;
use reqwest;
use scraper::{Html, Selector};
use std::format;
use tokio;

fn get_file_extension(byte_stream: &[u8]) -> String {
    infer::Infer::new()
        .get(byte_stream)
        .unwrap_or(infer::Type {
            mime: String::from("image/png"),
            ext: String::from("png"),
        })
        .ext
}

fn get_image_name(url: &str) -> &str {
    url.split("/")
        .last()
        .and_then(|x| x.split(".").next())
        .unwrap_or("nameless")
}

async fn download_image<'a>(
    img_node: scraper::element_ref::ElementRef<'a>,
    save_location: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(url) = img_node.value().attr("src") {
        println!("downloading {}", url);
        let img_bytes = reqwest::get(url).await?.bytes().await?;
        let img_extension = get_file_extension(img_bytes.as_ref());
        let pic_name = get_image_name(url);
        let mut file = tokio::fs::File::create(format!(
            "{}/pic_{}.{}",
            save_location, pic_name, img_extension
        ))
        .await?;
        tokio::io::copy(&mut img_bytes.as_ref(), &mut file).await?;
        println!("dowloaded {}", url);
    }
    Ok(())
}

async fn download_images(url: &str, save_location: &str) -> Result<(), Box<dyn std::error::Error>> {
    let html_resp = reqwest::get(url).await?.text().await?;
    let document = Html::parse_document(&html_resp);
    let img_selector = Selector::parse("img").expect("Error while creating 'img' selector");
    let mut image_futures = Vec::new();

    for img_node in document.select(&img_selector) {
        image_futures.push(download_image(img_node, save_location));
    }
    join_all(image_futures).await;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    download_images(
        "https://towardsdatascience.com/image-scraping-with-python-a96feda8af2d",
        "downloads",
    )
    .await?;
    Ok(())
}
