use std::fs;
use std::io::Write;

#[cfg(target_os = "windows")]
const HOST_FILE_PATH: &str = "C:\\Windows\\System32\\drivers\\etc\\hosts";

#[cfg(not(target_os = "windows"))]
const HOST_FILE_PATH: &str = "/etc/hosts";

const GITHUB_MARK_START: &str = "# GITHUB_MARK_START";
const GITHUB_MARK_END: &str = "# GITHUB_MARK_END";

pub fn mod_hosts_file(hosts: String) -> Result<(), std::io::Error> {
    let mut txt = fs::read_to_string(HOST_FILE_PATH)?;
    let hosts = format!("{}\n{}{}", GITHUB_MARK_START, hosts, GITHUB_MARK_END);

    let idx_start = txt.find(GITHUB_MARK_START);
    let idx_end = txt.rfind(GITHUB_MARK_END);
    if idx_start.is_some() && idx_end.is_some() {
        let idx_start = idx_start.unwrap();
        let idx_end = idx_end.unwrap() + GITHUB_MARK_END.len();

        let pre: String = txt.chars().take(idx_start).collect();
        let suf: String = txt.chars().skip(idx_end).collect();
        txt = format!(
            "{}{}{}",
            pre,
            hosts,
            suf
        );
    } else {
        txt = txt + hosts.as_str();
    }

    let mut file = fs::File::create(HOST_FILE_PATH)?;
    file.write_all( txt.as_bytes())
}
