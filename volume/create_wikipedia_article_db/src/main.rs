use create_wikipedia_article_db::index_list::index_list::create_index_list;

const ORIGINAL_INDEX: &str = "./test_index.txt"; 
        //"../WikipediaDump/jawiki-20220920-pages-articles-multistream-index.txt";

fn main() {
    // WikipediaDumpのIndexを読み取り
    create_index_list(ORIGINAL_INDEX);
    // 記事以外を除去
    // リスト保存
    // Wikipedia_bumpのIndexリストのみの読み取り
    // DB保存
}