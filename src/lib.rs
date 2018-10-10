use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*; // 內部定義許多 I/O 相關的 trait

// 讀取檔案並進行操作
// 由於為了避免程式直接 panic 所以盡量不要用 expect
// fn run(config: Config) {
//     // 開啟檔案
//     let mut f: File = File::open(config.filename).expect("File not found!");

//     // 用來儲存文件內容
//     let mut contents: String = String::new();
//     f.read_to_string(&mut contents).expect("Something went wrong reading the file.");

//     println!("With text:\n{}", contents);
// }

/// 搜尋字符串
/// 使用 lifetime parameter 的原因
/// 是因為回傳為 ref, 編譯器必須知道到底是向 query 還是 contents borrow.
///
/// 參數型別必須為 &str, 因為編譯器無法在編譯時期得知 str 長度
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines() // why &query.to_uppercase(), 因為 contains 要吃 sub slice of str.
        .filter(|line| line.to_uppercase().contains(&query.to_uppercase()))
        .collect()
}

pub fn run(config: Config) -> Result<(), Box<Error>> {
    // question mark 的用途為傳播錯誤, 將錯誤直接回傳
    let mut f: File = File::open(config.filename)?;

    let mut contents: String = String::new();
    f.read_to_string(&mut contents)?;

    // 判斷是否 case sensitive
    let results: Vec<&str> = if config.is_case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

/// 應用程式參數資料
pub struct Config {
    pub query: String,
    pub filename: String,
    pub is_case_sensitive: bool,
}

impl Config {
    /// Constructor
    /// 重構 parse_config 為構造函數
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        if args.len() < 3 {
            // panic!("Not enough arguments.");
            // 直接 panic 並不是最好的
            // 所以改為用 Result 的方式
            return Err("Not enough arguments.");
        }

        // skip first, 第一個為主程式名稱.
        args.next();

        // 將原本使用 clone 修改成 iter.next() 來迭代到我們要求的參數
        let query: String = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string."),
        };
        let filename: String = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name."),
        };

        let is_case_sensitive: bool = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {
            query,
            filename,
            is_case_sensitive,
        })
    }
}

// fn parse_config(args: &[String]) -> Config {

//     // 此處使用 clone, 犧牲小部份效能, 換取程式碼簡潔
//     let query = args[1].clone();
//     let filename = args[2].clone();

//     Config {
//         query,
//         filename
//     }
// }
