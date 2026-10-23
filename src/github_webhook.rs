//! POST {subdir}/github-webhook: github notifies about a `git push` to the documents repository (content type
//! application/json, signed with github_webhook_secret), pankat runs `git pull --ff-only` in the input
//! directory and the file monitor rebuilds what changed
use crate::config::{self, NOT_SET};
use crate::error::AppError;
use axum::body::Bytes;
use axum::http::{HeaderMap, StatusCode};
use axum::response::{IntoResponse, Response};
use sha2::{Digest, Sha256};
use std::path::Path;
use tokio::sync::Mutex;

/// one git pull at a time, a push during a pull pulls again afterwards
static PULL: Mutex<()> = Mutex::const_new(());

/// the webhook is disabled (404) until github_webhook_secret is set in pankat.toml (or the environment)
pub fn enabled() -> bool {
    let secret = &config::Config::get().github_webhook_secret;
    !secret.is_empty() && secret != NOT_SET
}

pub async fn github_webhook(headers: HeaderMap, body: Bytes) -> Response {
    if !enabled() {
        return AppError::NotFound.into_response();
    }
    let header = |name: &str| headers.get(name).and_then(|v| v.to_str().ok());
    let secret = &config::Config::get().github_webhook_secret;
    if !signature_valid(secret, &body, header("x-hub-signature-256")) {
        println!("github webhook: missing or invalid X-Hub-Signature-256");
        return (StatusCode::UNAUTHORIZED, "invalid signature\n").into_response();
    }
    match header("x-github-event") {
        // sent when the webhook is created
        Some("ping") => (StatusCode::OK, "pong\n").into_response(),
        Some("push") => {
            // github waits 10 seconds for the response, the pull may take longer
            let input = config::Config::get().input.clone();
            tokio::spawn(async move {
                match git_pull(&input).await {
                    Ok(output) => println!(
                        "github webhook: git pull in {}: {}",
                        input.display(),
                        output.trim()
                    ),
                    Err(e) => eprintln!(
                        "github webhook: git pull in {} failed: {}",
                        input.display(),
                        e.trim()
                    ),
                }
            });
            (StatusCode::ACCEPTED, "git pull started\n").into_response()
        }
        event => {
            println!("github webhook: ignoring event {:?}", event);
            StatusCode::NO_CONTENT.into_response()
        }
    }
}

/// X-Hub-Signature-256 is "sha256=" and the hex of the HMAC-SHA256 of the body, compared in constant time
fn signature_valid(secret: &str, body: &[u8], signature: Option<&str>) -> bool {
    let Some(given) = signature else {
        return false;
    };
    let expected = sign(secret, body);
    let given = given.to_ascii_lowercase();
    let difference = given
        .bytes()
        .zip(expected.bytes())
        .fold(0u8, |acc, (a, b)| acc | (a ^ b));
    given.len() == expected.len() && difference == 0
}

/// the X-Hub-Signature-256 header github sends along with the body
pub fn sign(secret: &str, body: &[u8]) -> String {
    let hex: String = hmac_sha256(secret.as_bytes(), body)
        .iter()
        .map(|b| format!("{:02x}", b))
        .collect();
    format!("sha256={}", hex)
}

/// RFC 2104 with SHA-256 (block size 64 bytes)
fn hmac_sha256(key: &[u8], message: &[u8]) -> [u8; 32] {
    let mut block = [0u8; 64];
    if key.len() > block.len() {
        block[..32].copy_from_slice(&Sha256::digest(key));
    } else {
        block[..key.len()].copy_from_slice(key);
    }
    let inner_pad: Vec<u8> = block.iter().map(|b| b ^ 0x36).collect();
    let outer_pad: Vec<u8> = block.iter().map(|b| b ^ 0x5c).collect();
    let inner = Sha256::new()
        .chain_update(&inner_pad)
        .chain_update(message)
        .finalize();
    let outer = Sha256::new()
        .chain_update(&outer_pad)
        .chain_update(inner)
        .finalize();
    let mut mac = [0u8; 32];
    mac.copy_from_slice(&outer);
    mac
}

/// stdout and stderr of `git pull --ff-only`, Err when it failed; never asks for credentials
async fn git_pull(dir: &Path) -> Result<String, String> {
    let _pull = PULL.lock().await;
    let output = tokio::process::Command::new("git")
        .arg("-C")
        .arg(dir)
        .args(["pull", "--ff-only"])
        .env("GIT_TERMINAL_PROMPT", "0")
        .output()
        .await
        .map_err(|e| format!("can't run git: {}", e))?;
    let text = format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    if output.status.success() {
        Ok(text)
    } else {
        Err(text)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::process::Command;

    fn hex(bytes: &[u8]) -> String {
        bytes.iter().map(|b| format!("{:02x}", b)).collect()
    }

    #[test]
    fn test_hmac_sha256() {
        // RFC 4231 test cases 1 and 6 (key longer than the block)
        assert_eq!(
            hex(&hmac_sha256(&[0x0b; 20], b"Hi There")),
            "b0344c61d8db38535ca8afceaf0bf12b881dc200c9833da726e9376c2e32cff7"
        );
        assert_eq!(
            hex(&hmac_sha256(
                &[0xaa; 131],
                b"Test Using Larger Than Block-Size Key - Hash Key First"
            )),
            "60e431591ee0b67f0d8a26aacbf5b77f8e0bc6213728c5140546040f0ee37f54"
        );
    }

    #[test]
    fn test_signature_valid() {
        // the example of the github documentation (validating webhook deliveries)
        let secret = "It's a Secret to Everybody";
        let signature = "sha256=757107ea0eb2509fc211221cce984b8a37570b6d7586c22c46f4379c8b043e17";
        assert!(signature_valid(secret, b"Hello, World!", Some(signature)));
        assert!(signature_valid(secret, b"Hello, World!", Some(&signature.to_uppercase().replace("SHA256=", "sha256="))));
        assert!(!signature_valid(secret, b"Hello, World?", Some(signature)));
        assert!(!signature_valid("another secret", b"Hello, World!", Some(signature)));
        assert!(!signature_valid(secret, b"Hello, World!", Some(&signature[..40])));
        assert!(!signature_valid(secret, b"Hello, World!", Some(&signature["sha256=".len()..])));
        assert!(!signature_valid(secret, b"Hello, World!", None));
    }

    fn git(dir: &Path, args: &[&str]) {
        let status = Command::new("git")
            .arg("-C")
            .arg(dir)
            .args(["-c", "user.name=test", "-c", "user.email=test@example.org"])
            .args(args)
            .status()
            .unwrap();
        assert!(status.success(), "git {:?} in {}", args, dir.display());
    }

    #[tokio::test]
    async fn test_git_pull() {
        let dir = std::env::temp_dir().join("pankat-test-github-webhook");
        let _ = std::fs::remove_dir_all(&dir);
        let (origin, author, input) = (dir.join("origin"), dir.join("author"), dir.join("input"));
        std::fs::create_dir_all(&author).unwrap();
        git(&dir, &["init", "--bare", "--initial-branch=main", "origin"]);
        git(&author, &["init", "--initial-branch=main"]);
        std::fs::write(author.join("first.mdwn"), "first").unwrap();
        git(&author, &["add", "."]);
        git(&author, &["commit", "-m", "first"]);
        git(&author, &["push", origin.to_str().unwrap(), "main"]);
        git(&dir, &["clone", origin.to_str().unwrap(), "input"]);

        // a push to the documents repository
        std::fs::write(author.join("second.mdwn"), "second").unwrap();
        git(&author, &["add", "."]);
        git(&author, &["commit", "-m", "second"]);
        git(&author, &["push", origin.to_str().unwrap(), "main"]);

        git_pull(&input).await.unwrap();
        assert_eq!(std::fs::read_to_string(input.join("second.mdwn")).unwrap(), "second");
        assert!(git_pull(&dir.join("missing")).await.is_err());
        std::fs::remove_dir_all(&dir).unwrap();
    }
}
