use std::fs::OpenOptions;
use std::io::Write;

fn main() {
    let file_result = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true) // ファイルが存在しなければ作る
        .append(true) // 追記を可能に
        .open("teyst.txt");

    if let Ok(mut file) = file_result {
        let str = "test";
        write!(file, "{}", str).unwrap();
    }
}
