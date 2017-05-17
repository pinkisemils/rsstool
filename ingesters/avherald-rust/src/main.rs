extern crate hyper;
extern crate hyper_native_tls;
extern crate select;

use hyper::Client;
use hyper::net::HttpsConnector;
use hyper::header::UserAgent;
use hyper_native_tls::NativeTlsClient;
use std::io::Read;

use select::document::Document;
use select::predicate::{Attr, Name, And};

fn main() {
    let ssl = NativeTlsClient::new().unwrap();
    let connector = HttpsConnector::new(ssl);
    let client = Client::with_connector(connector);

    //let mut resp = client.get("https://avherald.com/")
    let mut resp = client
        .get("https://gist.githubusercontent.com/AbstractBeliefs/b4b04f607e9336fe9d822bb214e497ef/raw/597092880b03124a724d7d5c22658ba42805707a/index.html")
        .header(UserAgent("Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/57.0.2987.98 Safari/537.36".to_string()))
        .send()
        .unwrap();
    let mut body = String::new();
    resp.read_to_string(&mut body).unwrap();

    let dom = Document::from(body.as_str());
    let article_root = dom.find(Attr("id", "ad1cell")).nth(0).expect("Couldn't find the article list");

    for node in article_root.find(And(Name("td"), Attr("align", "center"))) {
        // Find the severity
        let severity = node.find(Name("img")).nth(0)
                .and_then(|img| img.attr("alt"))
                .unwrap_or("Unknown severity");
        // Find the title
        let title = node.next()
                .and_then(|contentcell| Some(contentcell.text()))
                .unwrap_or("No title found".to_string());

        // Find the link
        let link = node.next()
                    .and_then(|c_cell| c_cell.find(Name("a")).nth(0)
                        .and_then(|link| link.attr("href"))
                        .and_then(|addr| Some("https://avherald.com".to_string() + addr)))
                    .unwrap_or("no address found".to_string());


        println!("[{}] {}: {}", severity, title, link);
    }
}
