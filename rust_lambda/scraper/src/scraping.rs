use reqwest;
extern crate select;
use serde::Serialize;
use select::document::Document;
use select::node::Node;
use select::predicate::{Name, Predicate};

#[derive(Debug)]
#[derive(Serialize)]
pub struct Meta {
    pub name: String,
    pub description: String,
    pub image: String,
}
#[allow(dead_code)]
pub async fn fetch(url: String)-> Result<Meta, bool> {
    let res = match reqwest::get(url).await {
        Ok(response) => response,
        Err(_) => {
            return Err(false);
        }
    };

    let html = match res.text().await {
        Ok(text) => text,
        Err(_) => {
            return Err(false);
        }
    };

    Ok(html_parse(html))
}

fn html_parse(html: String) -> Meta {

    let document = Document::from(html.as_str());

    //get website title
    
    let title = document.find(Name("title")).next()
    .map(|n| n.text())
    .unwrap_or_else(|| "".to_string());

    // get description or og:description
    let description = document.find(Name("meta").and(|n: &Node| {
        n.attr("property").unwrap_or("").eq_ignore_ascii_case("og:description")
    })).next()
        .and_then(|node| node.attr("content"))
        .or_else(|| {
            document.find(Name("meta").and(|n: &Node| {
                n.attr("name").unwrap_or("").eq_ignore_ascii_case("description")
            })).next()
            .and_then(|node| node.attr("content"))
        }).unwrap_or("").to_string();
    
    // get og:image path
    let image = document.find(Name("meta").and(|n: &Node| {
        n.attr("property").unwrap_or("").eq_ignore_ascii_case("og:image")
    })).next()
        .and_then(|node| node.attr("content"))
        .unwrap_or("").to_string();

    let meta = Meta {
        name: title,
        description: description,
        image: image,
    };

    return meta;

}