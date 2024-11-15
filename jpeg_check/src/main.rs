use std::io;

fn main() {
    let mut input = String::new();
    println!("画像ファイルのパスを入力してください: ");

    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            let path = input.trim();
            println!("入力されたパス: {}", path);
        }
        Err(e) => {
            println!("エラーが発生しました: {}", e);
        }
    }
}
