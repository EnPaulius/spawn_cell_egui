fn main() {
    if cfg!(target_os = "windows") {
        let mut res = winres::WindowsResource::new();
        res.set_icon("app_icon.ico"); // Use your icon file name
        res.compile().expect("Failed to compile icon resource");
    }
}
