
#[macro_use]
extern crate unhtml_derive;
extern crate unhtml;

use unhtml::{FromHtml, VecFromHtml};

fn main() -> Result<(), Box<std::error::Error>> {
    let escaped_html_text = reqwest::get("https://longisland.craigslist.org/search/msa")?.text()?;

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

    let result_metas = ResultMetas::from_html(&escaped_html_text)?;

    for result in result_metas.results {
        println!("Price: {}, Neighboorhood: {}", result.price, result.neighborhood);
    }

    Ok(())
}
