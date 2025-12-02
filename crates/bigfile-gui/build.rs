#[cfg(not(windows))]
fn main() {}

#[cfg(windows)]
fn main() -> std::io::Result<()> {
    use winres;

    let mut res = winres::WindowsResource::new();
    res.set_icon("assets/icon.ico");
    res.compile()?;
}
