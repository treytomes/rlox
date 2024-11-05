pub struct AppInfo {
    pub name: &'static str,
    pub version: &'static str,
    pub description: &'static str,
}

impl AppInfo {
    fn new(name: &'static str, version: &'static str, description: &'static str) -> Self {
        Self {
            name,
            version,
            description,
        }
    }

    pub fn from_env() -> Self {
        let name = env!("CARGO_PKG_NAME");
        let version = env!("CARGO_PKG_VERSION");
        let description = env!("CARGO_PKG_DESCRIPTION");
        AppInfo::new(&name, &version, &description)
    }
}
