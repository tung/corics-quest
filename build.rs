fn main() {
    if std::env::var("CARGO_CFG_TARGET_OS").unwrap() == "windows" {
        let mut res = winresource::WindowsResource::new();
        res.set("FileDescription", "CoricsQuest");
        res.set_icon_with_id("win-icon.ico", "1");
        println!("cargo::rerun-if-changed=win-icon.ico");
        res.compile().unwrap();
    }
}
