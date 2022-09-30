pub mod unzip_wikipedia_dump {

    pub fn unzip(original_dump: &str, output_index: &str) -> Result<(), Box<dyn std::error::Error>> {
        extern crate bzip2;
        extern crate parse_mediawiki_dump;

        let file = std::fs::File::open(original_dump).unwrap();
        let file = std::io::BufReader::new(file);
        let file = bzip2::bufread::BzDecoder::new(file);
        let file = std::io::BufReader::new(file);

        for result in parse_mediawiki_dump::parse(file) {
            match result {
                Err(error) => {
                    eprintln!("Error: {}", error);
                    break;
                }
                Ok(page) => if page.namespace == 0 && match &page.format {
                    None => false,
                    Some(format) => format == "text/x-wiki"
                } && match &page.model {
                    None => false,
                    Some(model) => model == "wikitext"
                } {
                    println!(
                        "The page {title:?} is an ordinary article with byte length {length}.",
                        title = page.title,
                        length = page.text.len()
                    );
                } else {
                    println!("The page {:?} has something special to it.", page.title);
                }
            }
        }
        Ok(())
        // https://docs.rs/parse_mediawiki_dump/latest/parse_mediawiki_dump/
    }
}