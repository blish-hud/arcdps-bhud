fn main() {
    let mut info = windows_exe_info::versioninfo::VersionInfo::from_cargo_env();
    info.file_info[0].original_filename =
        format!("{}.dll", std::env::var("CARGO_PKG_NAME").unwrap()).into();
    info.link().unwrap();
}
