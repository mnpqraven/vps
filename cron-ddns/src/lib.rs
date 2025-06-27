use std::process::Command;

pub fn ip_lookup() -> String {
    // dig -6 TXT +short o-o.myaddr.l.google.com @ns1.google.com
    let cmd = Command::new("dig")
        .args([
            "-6",
            "TXT",
            "+short",
            "o-o.myaddr.l.google.com",
            "@ns1.google.com",
        ])
        .output();
    let result = cmd.expect("is dig installed ?").stdout;
    let ip = String::from_utf8(result)
        .expect("conversion TODO:")
        .trim()
        .replace("\"", "");
    ip
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn same_with_curl() {
        let left = ip_lookup();

        // curl ifconfig.me
        let right_cmd = Command::new("curl")
            .arg("ifconfig.me")
            .output()
            .expect("valid connection to ifconfig.me?")
            .stdout;

        let right = String::from_utf8(right_cmd).expect("conversion TODO:");
        let right = right.trim();
        assert_eq!(left, right);
    }
}
