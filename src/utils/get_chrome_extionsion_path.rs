pub fn get_chrome_extionsion_path() -> String {
    let mut chrome_extension_path = std::env::current_dir().unwrap().into_os_string();
    chrome_extension_path.push("\\NopeCHA\\dist\\chrome");
    chrome_extension_path.to_str().unwrap().to_owned()
}