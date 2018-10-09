extern crate minigrep;

#[test]
fn cast_sensitive() {
    let query = "duct";
    let contents = "\
Rust:
safe, fast, productive.
Duct three.";

    assert_eq!(vec!["safe, fast, productive."], minigrep::search(query, contents));
}

#[test]
fn cast_insensitive() {
    let query = "RusT";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

    assert_eq!(
        vec!["Rust:", "Trust me."],
        minigrep::search_case_insensitive(query, contents)
    );
}
