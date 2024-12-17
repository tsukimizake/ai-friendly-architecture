mod markdown;

use markdown::parser::parse_markdown;

fn main() {
    let markdown_text = "# AIフレンドリーアーキテクチャくん\n- これは何\n[[プロンプト生成]]";
    let doc = parse_markdown(markdown_text);
    
    println!("Parsed markdown document:");
    for element in doc.elements {
        println!("{:?}", element);
    }
}
