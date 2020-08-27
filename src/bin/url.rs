use url::{Url};
/**
*
* @date 2020/12/2
*/
fn main() {
    let full = "http://llever.com/rust-cookbook-zh/web/url.zh.html?a=123";
    let parsed = Url::parse(full).unwrap();
    println!("{:?}", parsed.scheme());
    println!("{:?}", parsed.host());
    println!("{:?}", parsed.path());
    println!("{:?}", parsed.path_segments());
    println!("{:?}", parsed.query());
}