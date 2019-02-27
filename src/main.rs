
use std::default::Default;
use unhtml::{self, FromHtml};


fn main() -> Result<(), Box<std::error::Error>> {
    let mut escaped_html_text = reqwest::get("https://longisland.craigslist.org/search/msa")?.text()?;


    #[derive(FromHtml)]
    #[html(selector = "#test")]
    struct Listing {
        #[html(selector = "p:nth-child(1)", attr = "inner")]
        name: String,

        #[html(selector = "p:nth-child(2)", attr = "inner")]
        age: u8,

        #[html(selector = "p:nth-child(3)", attr = "inner")]
        like_lemon: bool,
    }


    Ok(())
}
