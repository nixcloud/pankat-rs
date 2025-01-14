use std::io::Write;
use std::error::Error;


pub fn render_file(
    path: String
) -> Result<String, Box<dyn Error>> {

    let article_markdown = std::fs::read_to_string(&path)?;
    
    let luafile = std::path::Path::new("documents/pandoc-lua/shifted-numbered-headings.lua"); // Use the Path::new method correctly.
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
        let stdin = pandoc_process.stdin.as_mut().ok_or("Failed to open stdin")?;
        stdin.write_all(article_markdown.as_bytes())?; // Correct byte conversion
    }

    let output = pandoc_process.wait_with_output()?;
    let output_str = String::from_utf8(output.stdout.clone())?;

    if output.status.success() {
          let stdout = String::from_utf8_lossy(&output.stdout).to_string();
          println!("\n{}", stdout);
          println!("-- Rendering file done --");
        
        Ok(output_str)
    } else {
        Err("Pandoc process failed".into())
    }
}