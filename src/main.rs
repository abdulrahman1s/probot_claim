use headless_chrome::browser::default_executable;
use headless_chrome::protocol::cdp::Page::CaptureScreenshotFormatOption;
use headless_chrome::{Browser, LaunchOptions, Tab};
use serde_json::Value;
use std::ffi::OsStr;
use std::sync::Arc;
use std::{fs, vec};
#[tokio::main]
async fn main() {
    let mut test = std::env::current_dir().unwrap().into_os_string();
    test.push("\\NopeCHA\\dist\\chrome");
    let data = fs::read_to_string("./probot_tokens.txt").expect("Unable to read file");
    let tokens = data.split("\n");
    for token in tokens {
        spawn(token.to_string(), test.to_str().unwrap());
    }
}

fn spawn(token: String, path: &str) {
    let probot_daily = String::from("https://probot.io/daily");
    let browser = Browser::new(
        LaunchOptions::default_builder()
            .disable_default_args(true)
            .path(Some(default_executable().unwrap()))
            .extensions(vec![OsStr::new(path)])
            .headless(false)
            .build()
            .unwrap(),
    )
    .unwrap();
    let tab = browser.wait_for_initial_tab().unwrap();
    tab.navigate_to(&probot_daily)
        .unwrap_or_else(|_| panic!("couldn't navigate to \"{}\"!", &probot_daily));
    tab.wait_for_element("body")
        .unwrap()
        .call_js_fn(
            r#"function locals (token) {
    localStorage.setItem("ac", token)
}"#,
            vec![Value::String(token.clone().to_string())],
            false,
        )
        .unwrap();
    tab.navigate_to(&probot_daily)
        .unwrap_or_else(|_| panic!("couldn't navigate to \"{}\"!", &probot_daily));
    tab.wait_for_element(".sidebar_ltr__kXJvp ").unwrap();
    tab.wait_for_element(".daily-logo-text")
        .unwrap()
        .click()
        .unwrap();
    check(&tab);
    screenshot(&tab, token);
}

fn check(tab: &Tab) -> &Tab {
    match tab.wait_for_element("#daily-time-left") {
        Ok(_) => tab,
        Err(_) => check(tab),
    }
}

fn screenshot(tab: &Arc<Tab>, token: String) {
    let png_data = tab
        .wait_for_element("body")
        .expect("couldn't find body element")
        .capture_screenshot(CaptureScreenshotFormatOption::Png)
        .expect("couldn't take a screenshot");
    fs::write(format!("./screenshots/{}", token), &png_data)
        .unwrap_or_else(|_| panic!("couldn't take a screenshot!"));
}
