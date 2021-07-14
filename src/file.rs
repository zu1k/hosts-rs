
#[cfg(target_os = "windows")]
const HOST_FILE_PATH: &str = "C:\\Windows\\System32\\drivers\\etc\\hosts";

#[cfg(not(target_os = "windows"))]
const HOST_FILE_PATH: &str = "/etc/hosts";
