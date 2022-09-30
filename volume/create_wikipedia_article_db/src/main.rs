use create_wikipedia_article_db::index_list::index_list::create_index_list;
use create_wikipedia_article_db::unzip_wikipedia_dump::unzip_wikipedia_dump::unzip;

const ORIGINAL_INDEX: &str = "../WikipediaDump/jawiki-20220920-pages-articles-multistream-index.txt";
const OUTPUT_INDEX: &str = "./output_index.txt";
const ORIGINAL_DUMP: &str = "../WikipediaDump/jawiki-20220920-pages-articles-multistream.xml.bz2";

fn main() {
    // WikipediaDumpのIndexを読み取り・記事以外を除去・リスト保存
    // create_index_list(ORIGINAL_INDEX, OUTPUT_INDEX);

    // Wikipedia_bumpのIndexリストのみの読み取り
    unzip(ORIGINAL_DUMP, OUTPUT_INDEX);
    // DB保存

}