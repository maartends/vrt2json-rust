extern crate xmlJSON;
extern crate rustc_serialize;

use xmlJSON::XmlDocument;
use rustc_serialize::json;
use std::str::FromStr;

fn main() {
    let s = "<test lang=\"rust\">An XML Document <testElement>A teSt element</testElement></test>";
    let document : XmlDocument = XmlDocument::from_str(s).unwrap();
    let jsn : json::Json = document.to_json();
}
