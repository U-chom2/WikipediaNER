
pub mod index_list{
    pub fn create_index_list<'a>(original_index: &'a str, output_index: &'a str) -> Result<&'a str, std::io::Error> {
        use std::fs::File;
        use std::io::{BufRead, Write, BufReader};
        use prog_rs::prelude::*;
        
        println!("In file {}", original_index); 
        //  一気に読み込み
        //  let index = std::fs::read_to_string(original_index)
        //    // ファイルの読み込み中に問題がありました
        //    .expect("something went wrong reading the file");
    
        // リスト管理する場合
        // let mut index_filter = vec![];
        let mut file = File::create(output_index)?;
        // 一行ごとに読み込み
        for index in BufReader::new(File::open(original_index)?).lines().progress() {
            let l = index?;
            if !(l.contains("Wikipedia:") || l.contains("(曖昧さ回避)") || l.contains("Category:") || l.contains("Template:") || l.contains("Help:") || l.contains("ファイル:")){
                // index_filter.push(l);
                writeln!(file, "{}", &l)?;
            };
        }

        Ok(original_index)
    }
}