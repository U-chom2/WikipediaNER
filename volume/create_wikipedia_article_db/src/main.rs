use create_wikipedia_article_db::index_list::index_list::create_index_list;

const ORIGINAL_INDEX: &str = "../WikipediaDump/jawiki-20220920-pages-articles-multistream-index.txt";
const OUTPUT_INDEX: &str = "./output_index.txt";

fn main() {
    // WikipediaDumpのIndexを読み取り
    create_index_list(ORIGINAL_INDEX, OUTPUT_INDEX);
    // 記事以外を除去
    // リスト保存
    // Wikipedia_bumpのIndexリストのみの読み取り
    // DB保存
}