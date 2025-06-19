use reader_mode_maker;
use reqwest;

fn main() {

    let url = "https://example.com";
    let response = reqwest::blocking::get(url).unwrap();
    let content = response.text().unwrap();
    let reader_mode = reader_mode_maker::culling(&content);
    println!("{}", reader_mode);
}
