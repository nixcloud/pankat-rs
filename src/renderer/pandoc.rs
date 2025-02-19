use crate::config;
use std::error::Error;
use std::io::Write;
use std::path::PathBuf;



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
