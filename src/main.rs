#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
extern crate bili_api_http;
extern crate reqwest;
extern crate serde_json;

use bili_api_http::make_header;
use serde_json::json;
use serde_json::Value;

#[get("/status")]
fn status() -> String {
    let client = reqwest::blocking::Client::new();
    let data: Value = serde_json::from_reader(
        client
            .get("https://api.live.bilibili.com/ip_service/v1/ip_service/get_ip_addr")
            .headers(make_header())
            .send()
            .unwrap(),
    )
    .unwrap();
    if data["code"] != 0 {
        json!({
        "data": {
            "code": -1,
            "status": "看起来好像暂时被小陈ban了？ ╮(╯_╰)╭"
        }
        }).to_string()
    } else {
        json!({
        "data": {
            "code": 0,
            "status": "可用",
            "country": data["data"]["country"],
            "region": format!("{}{}", data["data"]["province"], data["data"]["city"]).replace("\"", "")
            }
        }).to_string()
    }
}

#[get("/video?<id>")]
fn video(id: String) -> String {
    let client = reqwest::blocking::Client::new();

    let avid_api = "https://api.bilibili.com/x/web-interface/view?aid=";
    let bvid_api = "https://api.bilibili.com/x/web-interface/view?bvid=";
    let ep_api = "https://api.bilibili.com/pgc/view/web/season?ep_id=";
    let ss_api = "https://api.bilibili.com/pgc/view/web/season?season_id=";

    if id.starts_with("av") {
        client
            .get(&(format!("{}{}", avid_api, &id[2..])))
            .headers(make_header())
            .send()
            .unwrap()
            .text()
            .unwrap()
    } else if id.starts_with("BV") {
        client
            .get(&(format!("{}{}", bvid_api, id)))
            .headers(make_header())
            .send()
            .unwrap()
            .text()
            .unwrap()
    } else if id.starts_with("ss") {
        client
            .get(&(format!("{}{}", ss_api, &id[2..])))
            .headers(make_header())
            .send()
            .unwrap()
            .text()
            .unwrap()
    } else if id.starts_with("ep") {
        client
            .get(&(format!("{}{}", ep_api, &id[2..])))
            .headers(make_header())
            .send()
            .unwrap()
            .text()
            .unwrap()
    } else {
        id
    }
}

#[get("/playurl?<avid>&<cid>&<qn>&<fourk>")]
pub fn get_download_url_qn(
    avid: String,
    cid: String,
    qn: Option<String>,
    fourk: Option<String>,
) -> String {
    let client = reqwest::blocking::Client::new();
    let fourk = match fourk {
        Some(fourk_value) => format!("fourk={}", fourk_value),
        None => String::from("fourk=0"),
    };
    match qn {
        Some(qn) => {
            let api = "https://api.bilibili.com/x/player/playurl?";
            let url = format!("{}avid={}&cid={}&qn={}&{}", api, avid, cid, qn, fourk);
            client
                .get(&url)
                .headers(make_header())
                .send()
                .unwrap()
                .text()
                .unwrap()
        }
        None => {
            let api = "https://api.bilibili.com/x/player/playurl?";
            let url = format!("{}avid={}&cid={}&{}", api, avid, cid, fourk);
            client
                .get(&url)
                .headers(make_header())
                .send()
                .unwrap()
                .text()
                .unwrap()
        }
    }
}

#[get("/")]
fn index() -> String {
    format!("你传了个锤子进来(")
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index, video, get_download_url_qn, status])
        .launch();
}
