use std::error::Error;
//use indoc::indoc;

pub fn hiddentext(input: &str) -> Result<String, Box<dyn Error>> {
    // HTML structure for toggling hidden text
    let html = format!(
        indoc::indoc! {r#"
            <div>
            <span class="hidden_text_toggle_button" onclick="
                    const hiddenText = this.nextElementSibling;
                    if (hiddenText.style.display === 'none' || hiddenText.style.display === '') {{
                        hiddenText.style.display = 'block';
                        this.textContent = 'Hide solution';
                    }} else {{
                        hiddenText.style.display = 'none';
                        this.textContent = 'Show solution';
                    }}
                ">Show Text</span>
                <div class="revealed_text" style="display: none;">{}</div>
            </div>
        "#},
        input
    );
    Ok(html)
}
