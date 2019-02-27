
#[macro_use]
extern crate unhtml_derive;
extern crate unhtml;

use unhtml::{FromHtml, VecFromHtml};

fn main() -> Result<(), Box<std::error::Error>> {
    // let escaped_html_text = reqwest::get("https://longisland.craigslist.org/search/msa")?.text()?;


    // #[derive(Debug, FromHtml)]
    // struct ResultMeta {
    //     #[html(selector = "span:nth-child(1)", attr = "inner")]
    //     pub price: String,

    //     #[html(selector = "span:nth-child(2)", attr = "inner")]
    //     pub neighborhood: String,
    // }

    // #[derive(Debug, FromHtml)]
    // #[html(selector = ".result-meta")]
    // struct ResultInfo {
    //     #[html(selector = "span")]
    //     pub results: Vec<ResultMeta>,
    // }

    // let listings_meta = ResultInfo::from_html(&escaped_html_text)?;

    // println!("{:?}", listings_meta.results);



    #[derive(Debug, FromHtml)]
    struct TestUser {
        #[html(selector = "p:nth-child(1)", attr = "inner")]
        name: String,

        #[html(selector = "p:nth-child(2)", attr = "inner")]
        age: u8,

        #[html(selector = "p:nth-child(3)", attr = "inner")]
        like_lemon: bool,
    }

    #[derive(Debug, FromHtml)]
    // #[html(selector = ".test")]
    struct TestUsers {
        #[html(selector = ".test > span")]
        pub users: Vec<TestUser>,
    }

    #[derive(Debug, FromHtml)]
    // #[html(selector = "#thing")]
    struct Everything {
        #[html(selector = "#thing")]
        pub users: Vec<TestUsers>,
    }


    let everything = Everything::from_html(r#"<!DOCTYPE html>
        <html lang="en">
        <head>
            <meta charset="UTF-8">
            <title>Title</title>
        </head>
        <body>
            <div id=thing>
                <div class="test">
                    <span>
                        <p>Hexilee</p>
                        <p>20</p>
                        <p>true</p>
                    </span>
                    <span>
                        <p>BigBrother</p>
                        <p>21</p>
                        <p>false</p>
                    </span>
                </div>
                <div class="test">
                    <span>
                        <p>Ryan</p>
                        <p>20</p>
                        <p>true</p>
                    </span>
                    <span>
                        <p>Caryn</p>
                        <p>21</p>
                        <p>false</p>
                    </span>
                </div>
            </div>
        </body>
        </html>"#).unwrap();



    println!("{:?}", everything);



    Ok(())
}
