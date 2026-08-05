fn main() -> std::io::Result<()> {
    // Icon embedding is Windows-only; winresource is a Windows-only build-dep.
    #[cfg(windows)]
    {
        use winresource::WindowsResource;
        WindowsResource::new()
            .set_icon("../assets/icons/invicon.ico")
            .compile()?;
    }
    Ok(())
}
