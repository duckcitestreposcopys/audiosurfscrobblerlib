use serde_derive::{Deserialize, Serialize};
use std::ffi::{c_char, CStr};
use mljcl::{self, MalojaCredentials};
use url::Url;

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

fn maloja(title: String, artist: String, url: String, key: String) {
    let url = Url::parse(&url).expect("Invalid host");
    let https = url.scheme() == "https";
    let default = match https {
        true => 443,
        false => 80
    };
    let port = url.port().unwrap_or(default);
    let creds: MalojaCredentials = MalojaCredentials {
        https,
        skip_cert_verification: true,
        ip: url.host_str().unwrap().to_string(),
        port,
        api_key: Some(key),
    };
    mljcl::scrobble(title, artist, creds).expect("Error while scrobbling");
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
