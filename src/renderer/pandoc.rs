use crate::config;
use std::error::Error;
use std::io::Write;
use std::path::PathBuf;

pub fn check_pandoc() -> Result<(), Box<dyn Error + Send + Sync>> {
    let pandoc_process = std::process::Command::new("pandoc")
        .arg("--version")
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::inherit())
        .spawn();

    match pandoc_process {
        Ok(pandoc_process) => match pandoc_process.wait_with_output() {
            Ok(output) => {
                if output.status.success() {
                    let stdout = String::from_utf8_lossy(&output.stdout);
                    //println!("{}", stdout);
                    let re = regex::Regex::new(r"pandoc (\d+)\.").unwrap();
                    if let Some(captures) = re.captures(&stdout) {
                        if let Some(major_version) = captures.get(1) {
                            if let Ok(major) = major_version.as_str().parse::<u32>() {
                                if major >= 3 {
                                    return Ok(());
                                }
                            }
                        }
                    }
                    return Err("Pandoc version 3 or newer not found!".into());
                } else {
                    return Err("Failed to execute pandoc process".into());
                }
            }
            Err(e) => Err(format!("Failed to execute pandoc process: {}", e).into()),
        },
        Err(e) => Err(format!("Can't find 'pandoc' binary: {}", e).into()),
    }
}

pub fn pandoc_mdwn_2_html(article_markdown: String) -> Result<String, Box<dyn Error>> {
    // println!("-------------------------");
    // println!("{}", article_markdown.clone());
    // println!("-------------------------");
    let cfg = config::Config::get();
    let mut assets: PathBuf = PathBuf::from(cfg.assets.clone());
    assets.push("pandoc-lua/shifted-numbered-headings.lua");
    let luafile = assets.as_path();
    //println!("luafile path: {:?}", luafile.clone());
    let mut pandoc_process = std::process::Command::new("pandoc")
        .arg("--lua-filter")
        .arg(luafile)
        .arg("-f")
        .arg("markdown")
        .arg("-t")
        .arg("html5")
        .arg("--highlight-style")
        .arg("kate")
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::inherit())
        .spawn()?;
    {
        // Correct mutable borrow and method call
        let stdin = pandoc_process
            .stdin
            .as_mut()
            .ok_or("Failed to open stdin")?;
        stdin.write_all(article_markdown.as_bytes())?;
    }

    let output = pandoc_process.wait_with_output()?;

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        Ok(stdout)
    } else {
        Err("Pandoc process failed".into())
    }
}
