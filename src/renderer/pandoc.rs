use crate::renderer::assets::Asset;
use std::error::Error;
use std::io::Write;

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

/// toc_title: Some(article title) adds the table of contents (toc.lua) with the title as its root
pub fn pandoc_mdwn_2_html(
    article_markdown: String,
    anchorjs: bool,
    toc_title: Option<&str>,
) -> Result<String, Box<dyn Error>> {
    // println!("-------------------------");
    // println!("{}", article_markdown.clone());
    // println!("-------------------------");
    let shifted_numbered_headings = Asset::ShiftedNumberedHeadingsLua
        .path()
        .display()
        .to_string();
    let create_anchors = Asset::CreateAnchorsLua.path().display().to_string();
    let toc = Asset::TocLua.path().display().to_string();
    let toc_title_metadata = format!("pankat-title={}", toc_title.unwrap_or(""));

    // Print the command line for debugging
    //println!("Executing pandoc with command: pandoc --lua-filter {:?} --lua-filter {:?} -f markdown -t html5 --highlight-style kate", luafile, luafile2);

    let mut args = vec![];

    if anchorjs {
        args.push("--lua-filter");
        args.push(&shifted_numbered_headings);
    }

    args.push("--lua-filter");
    args.push(&create_anchors);

    // after create-anchors.lua, the toc then lists the final heading ids
    if toc_title.is_some() {
        args.push("--lua-filter");
        args.push(&toc);
        args.push("--metadata");
        args.push(&toc_title_metadata);
    }

    args.push("-f");
    args.push("markdown");
    args.push("-t");
    args.push("html5");
    args.push("--highlight-style");
    args.push("kate");

    let mut pandoc_process = std::process::Command::new("pandoc")
        .args(args)
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
