use crate::{IpKind, utils::error::CronDdnsError};
use load_env::schema::EnvCloudflare;
use serde::Serialize;
use std::process::Command;
use tracing::info;

#[derive(Serialize)]
struct ApiBody {
    id: String,
    proxied: bool,
    r#type: String,
    name: String,
    content: String,
}

pub async fn cf_zone_api(conf: EnvCloudflare) -> Result<(), CronDdnsError> {
    let ip = public_ip(IpKind::V4);
    let EnvCloudflare {
        record_id,
        zone_id,
        api_token,
        email,
    } = conf;
    let url =
        format!("https://api.cloudflare.com/client/v4/zones/{zone_id}/dns_records/{record_id}");
    let body = ApiBody {
        id: zone_id,
        proxied: false,
        r#type: "A".to_string(),
        name: "othi.dev".to_string(),
        content: ip,
    };
    let client = reqwest::Client::new();

    // https://developers.cloudflare.com/api/resources/dns/subresources/records/methods/update/
    let res = client
        .put(url)
        .header("X-Auth-Email", email)
        .header("X-Auth-Key", api_token)
        .json(&body)
        .send()
        .await
        .expect("api err'ing ?");
    // TODO: response validation
    let response = res
        .text()
        .await
        .map_err(|e| CronDdnsError::Unknown(e.to_string()))?;
    info!(response);
    Ok(())
}

fn public_ip(kind: IpKind) -> String {
    // dig -6 TXT +short o-o.myaddr.l.google.com @ns1.google.com
    let cmd = Command::new("dig")
        .args([
            kind.dig_args(),
            "TXT",
            "+short",
            "o-o.myaddr.l.google.com",
            "@ns1.google.com",
        ])
        .output();
    let result = cmd.expect("is dig installed ?").stdout;
    String::from_utf8(result)
        .expect("command output should product normal utf8 characters")
        .trim()
        .replace("\"", "")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn same_with_curl_v6() {
        let left = public_ip(IpKind::V6);

        // curl ifconfig.me
        let right_cmd = Command::new("curl")
            .arg("ifconfig.me")
            .output()
            .expect("valid connection to ifconfig.me?")
            .stdout;

        let right = String::from_utf8(right_cmd).expect("correct string conversion");
        let right = right.trim();
        assert_eq!(left, right);
    }

    #[test]
    fn same_with_curl_v4() {
        let left = public_ip(IpKind::V4);

        // curl ifconfig.me
        let right_cmd = Command::new("curl")
            .arg("-4")
            .arg("ifconfig.me")
            .output()
            .expect("valid connection to ifconfig.me?")
            .stdout;

        let right = String::from_utf8(right_cmd).expect("correct string conversion");
        let right = right.trim();
        assert_eq!(left, right);
    }
}
