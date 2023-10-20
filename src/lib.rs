use reqwest::blocking::{Client, Response};
use serde_derive::{Deserialize, Serialize};
use std::ffi::{c_char, CStr};

pub struct ScrobbleData {
    pub artist: String,
    pub title: String,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct ScrobbleReq {
    s: String,
    a: Vec<String>,
    t: Vec<String>,
    i: Vec<String>,
    o: Vec<String>,
    r: Vec<String>,
    l: Vec<String>,
    b: Vec<String>,
    n: Vec<String>,
    m: Vec<String>,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
struct MalojaReq {
    artist: String,
    title: String,
    key: String,
}

fn maloja(title: String, artist: String, url: String, key: String) -> Response {
    let client = Client::new();
    let scrobblebody = MalojaReq { artist, title, key };
    client
        .post(url + "/apis/mlj_1/newscrobble")
        .json(&scrobblebody)
        .send()
        .unwrap()
}

fn deserialize_poststr(poststr: String) -> ScrobbleData {
    let config = serde_qs::Config::new(1, false);
    let req: ScrobbleReq = config.deserialize_str(&poststr).unwrap();
    return ScrobbleData {
        artist: req.a.get(0).unwrap().to_string(),
        title: req.t.get(0).unwrap().to_string(),
    };
}

#[no_mangle]
pub unsafe extern "system" fn scrobble(
    postdata: *const c_char,
    url: *const c_char,
    key: *const c_char,
) {
    let postdata = CStr::from_ptr(postdata).to_str().unwrap();
    let url = CStr::from_ptr(url).to_str().unwrap().to_string();
    let key = CStr::from_ptr(key).to_str().unwrap().to_string();
    let scrobbledata = deserialize_poststr(postdata.to_string());
    maloja(scrobbledata.title, scrobbledata.artist, url, key);
}
