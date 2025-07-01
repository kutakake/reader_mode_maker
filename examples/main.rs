use reader_mode_maker;
use reqwest;
use std::{thread, time};

fn main() {
    let url = "https://gigazine.net/news/20250701-wordpress-matt-mullenweg-interview/";
    let response = reqwest::blocking::get(url).unwrap();
    let content = response.text().unwrap();
    let now = time::Instant::now();
    /*
    for i in 0..1 {
        let reader_mode = reader_mode_maker::old_culling(&content);
    }
    println!("{:?}", now.elapsed());
    */
    let now = time::Instant::now();
    //for i in 0..1 {
        let reader_mode = reader_mode_maker::culling(&content);
    //}
    println!("{:?}", &reader_mode);
    println!("{:?}", now.elapsed());

    //println!("{}", reader_mode);
}
