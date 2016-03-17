
pub fn hello(name: Option<&str>) -> String {
    match name {
        Some(ref x) => format!("Hello, {}!", *x),
        None        => format!("Hello, World!"),
    }
}


