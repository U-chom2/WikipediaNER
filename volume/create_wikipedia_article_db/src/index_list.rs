
pub mod index_list{
    pub fn create_index_list(original_index: &str) -> Result<(), Box<dyn std::error::Error>> {
        use std::fs::File;
        use std::io::{BufRead, BufReader};   
        
        println!("In file {}", original_index);
        
        //  一気に読み込み
        //  let index = std::fs::read_to_string(original_index)
        //    // ファイルの読み込み中に問題がありました
        //    .expect("something went wrong reading the file");
    
        // 一行ごとに読み込み
        for index in BufReader::new(File::open(original_index)?).lines() {
            let l = index?;
            println!("{}", l);
        }

        Ok(())
    }
}