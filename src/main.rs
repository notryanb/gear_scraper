fn main() -> Result<(), Box<std::error::Error>> {
    let response = reqwest::get("https://longisland.craigslist.org/search/msa")?;

    println!("{:?}", response);
    Ok(())
}
