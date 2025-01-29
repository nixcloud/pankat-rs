use std::error::Error;
use std::io::Write;

pub fn render_file(path: String) -> Result<String, Box<dyn Error>> {
    let article_markdown = std::fs::read_to_string(&path)?;

    let luafile = std::path::Path::new("documents/pandoc-lua/shifted-numbered-headings.lua");

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
        crate::renderer::html::create_html_from_article_template();
        Ok(stdout)
    } else {
        Err("Pandoc process failed".into())
    }
}
