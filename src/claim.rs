use std::ffi::OsStr;

use headless_chrome::{browser::default_executable, Browser, LaunchOptions, Tab};
use serde_json::Value;

use crate::utils::screenshot;

pub fn spawn_calim(token: String, path: &str) {
    let probot_daily = String::from("https://probot.io/daily");
    let browser = Browser::new(
        LaunchOptions::default_builder()
            .disable_default_args(true)
            .path(Some(default_executable().unwrap()))
            .extensions(vec![OsStr::new(path)])
            .headless(true)
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
