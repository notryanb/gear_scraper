#[macro_use]
extern crate unhtml_derive;
extern crate unhtml;

use std::fmt;
use unhtml::{FromHtml, VecFromHtml};

fn main() -> Result<(), Box<std::error::Error>> {
    let escaped_html_text = reqwest::get("https://longisland.craigslist.org/search/msa")?.text()?;

    #[derive(Debug, FromHtml)]
    struct SearchResults {
        #[html(selector = "li.result-row")]
        pub results: Vec<ResultRow>,
    }

    #[derive(Debug, FromHtml)]
    struct ResultRow {
        #[html(attr = "data-pid", default = 0)]
        pub pid: i64,

        #[html(selector = "a.result-image", attr = "href", default = "-")]
        pub posting_url: String,

        #[html(selector = "p.result-info", default = "-")]
        pub result_info: ResultInfo,
    }

    #[derive(Debug, FromHtml)]
    struct ResultInfo {
        #[html(selector = "time.result-date", attr = "datetime", default = "--")]
        pub date_time: String,

        #[html(selector = "a.result-title", attr = "inner", default = "--")]
        pub title: String,

        #[html(selector = "span.result-meta", default = "--")]
        pub meta: ResultMeta,
    }

    #[derive(Debug, FromHtml)]
    struct ResultMeta {
        #[html(selector = "span.result-price", attr = "inner", default = "---")]
        pub price: String,

        #[html(selector = "span.result-hood", attr = "inner", default = "---")]
        pub neighborhood: String,
    }

    impl fmt::Display for ResultRow {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Posting =====================\npid: {}\ntitle: {}\ndatetime: {}\nurl: {}\nprice: {}\nlocation: {}\n\n",
                self.pid,
                self.result_info.title,
                self.result_info.date_time,
                self.posting_url,
                self.result_info.meta.price,
                self.result_info.meta.neighborhood
            )
        }
    }

    let result_metas = SearchResults::from_html(&escaped_html_text)?;

    for result in result_metas.results {
        println!("{}", result);
    }

    Ok(())
}
