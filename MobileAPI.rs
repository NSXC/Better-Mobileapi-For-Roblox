use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_LENGTH, CONTENT_TYPE};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

static mut CSRF: Option<String> = None;

#[derive(Debug, Deserialize, Serialize)]
struct UserInfo {
    UserID: u32,
    UserName: String,
    RobuxBalance: u32,
    ThumbnailUrl: String,
    IsAnyBuildersClubMember: bool,
    IsPremium: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let roblosecurity = "ROBLOXAPI";
    let client = Client::new();
    
    let mut headers = HeaderMap::new();
    headers.insert("Host", HeaderValue::from_static("auth.roblox.com"));
    headers.insert("Cookie", HeaderValue::from_str(&format!(".ROBLOSECURITY={}", roblosecurity))?);
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    headers.insert(CONTENT_LENGTH, HeaderValue::from_static("0"));
    
    let response = client
        .post("https://auth.roblox.com/v2/logout")
        .headers(headers.clone())
        .send()?;
    
    if let Some(csrf_token) = response.headers().get("x-csrf-token") {
        if let Ok(csrf_token_value) = csrf_token.to_str() {
            unsafe {
                CSRF = Some(csrf_token_value.to_string());
            }
        }
    }
    

    
    let mobileapi = client
        .get("https://www.roblox.com/mobileapi/userinfo")
        .header("Cookie", format!(".ROBLOSECURITY={}", roblosecurity))
        .send()?;
    
    if mobileapi.status().is_success() {
        let bodymobileapi = mobileapi.text()?;
        let user_info: Result<UserInfo, serde_json::Error> = serde_json::from_str(&bodymobileapi);
        
        match user_info {
            Ok(user_info) => {
                let mut user_data = Map::new();
                user_data.insert("UserID".to_string(), Value::Number(serde_json::Number::from(user_info.UserID)));
                user_data.insert("UserName".to_string(), Value::String(user_info.UserName));
                user_data.insert("RobuxBalance".to_string(), Value::Number(serde_json::Number::from(user_info.RobuxBalance)));
                user_data.insert("ThumbnailUrl".to_string(), Value::String(user_info.ThumbnailUrl));
                user_data.insert("IsAnyBuildersClubMember".to_string(), Value::Bool(user_info.IsAnyBuildersClubMember));
                user_data.insert("IsPremium".to_string(), Value::Bool(user_info.IsPremium));
                user_data.insert("Status".to_string(), Value::Number(serde_json::Number::from(200u32)));
                unsafe {if let Some(ref csrf) = CSRF {user_data.insert("Token".to_string(), Value::String(csrf.clone()));}}
                let json_string = serde_json::to_string(&user_data).unwrap();
                println!("{}", json_string);
            }
            Err(err) => {
                let mut user_data = Map::new();
                user_data.insert("Status".to_string(), Value::Number(serde_json::Number::from(400u32)));
                let json_string = serde_json::to_string(&user_data).unwrap();
                println!("{}", json_string);
            }
        }
    } else {
        let mut user_data = Map::new();
        user_data.insert("Status".to_string(), Value::Number(serde_json::Number::from(400u32)));
        let json_string = serde_json::to_string(&user_data).unwrap();
        println!("{}", json_string);
    }

    Ok(())
}
