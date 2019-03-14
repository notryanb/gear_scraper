#[macro_use]
extern crate unhtml_derive;
extern crate unhtml;

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
    }

    #[derive(Debug, FromHtml)]
    struct ResultMetas {
        #[html(selector = "span.result-meta")]
        pub results: Vec<ResultMeta>,
    }

    #[derive(Debug, FromHtml)]
    struct ResultMeta {
        #[html(selector = "span.result-price", attr = "inner", default = "---")]
        pub price: String,

        #[html(selector = "span.result-hood", attr = "inner", default = "---")]
        pub neighborhood: String,
    }

    let result_metas = SearchResults::from_html(&escaped_html_text)?;

    for result in result_metas.results {
        println!(
            "PID: {}, DateTime: {:?}, Title: {:?},  URL: {}",
            result.pid, result.result_info.date_time, result.result_info.title, result.posting_url
        );
    }

    Ok(())
}
