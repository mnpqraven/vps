use std::process::Command;

fn public_ip() -> String {
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
        .expect("command output should product normal utf8 characters")
        .trim()
        .replace("\"", "");
    ip
}

pub async fn update_cf_conf() {
    let _ip = public_ip();
    todo!("CALLING CF UPDATE API");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn same_with_curl() {
        let left = public_ip();

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
