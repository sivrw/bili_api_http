use serde_json::Value;
use std::env::var;

pub fn make_header() -> reqwest::header::HeaderMap {
    let mut header = reqwest::header::HeaderMap::new();
    header.insert(
        "accpet-language",
        "zh-HK,zh;q=0.9,ja-JP;q=0.8,ja;q=0.7,en-US;q=0.6,en;q=0.5,zh-CN;q=0.4"
            .parse()
            .unwrap(),
    );
    header.insert("origin", "https://www.bilibili.com".parse().unwrap());
    header.insert("Referer", "https://www.bilibili.com/".parse().unwrap());
    header.insert("User-Agent", "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/88.0.4324.150 Safari/537.36".parse().unwrap());
    header.insert("Cookie", format!("SESSDATA={}", set_sessdata()).parse().unwrap());
    header
}

fn set_sessdata() -> String{
    return match var("SESSDATA") {
        Ok(sessdata) => sessdata,
        Err(_) => String::from("2333333333333333333333333"),
    };
}
