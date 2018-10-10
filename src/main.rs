extern crate minigrep;

use std::env;
use std::process;

use minigrep::Config;

/// 主程式
fn main() {
    // let args: Vec<String> = env::args().collect(); // collect() 用來將 iter 轉為集合

    // // Config 的構造函數改為回傳 Result, 因此此處對 Result 進行處理
    // // unwarp_or_else() 將自動拆封 Ok(value), 如果出錯, 則呼叫傳入的閉包(callback)
    // let config: Config = Config::new(&args).unwrap_or_else(|err| {
    //     // 使用 eprintln!() 時, $ cargo run > output.txt, 不會將錯誤輸出至 output
    //     // 因為只有 println! 會輸出至 output.txt, 相反的就不會顯示在 terminal
    //     eprintln!("Problem parsing arguments: {}", err);
    //     process::exit(1);
    // });

    let config: Config = Config::new(env::args()).unwrap_or_else(|err| {
        // 使用 eprintln!() 時, $ cargo run > output.txt, 不會將錯誤輸出至 output
        // 因為只有 println! 會輸出至 output.txt, 相反的就不會顯示在 terminal
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    // 上面用 closure 這邊用 pattern matching
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);

        process::exit(1);
    }
}
