use std::{
    io::{self, Write},
    process,
};

fn main() {
    println!("cargo:rerun-if-changed=tailwind.config.js");
    println!("cargo:rerun-if-changed=web_assets/tailwind-input.css");
    println!("cargo:rerun-if-changed=web_assets/index.html");

    match process::Command::new("sh")
        .arg("-c")
        .arg("tailwindcss -i web_assets/tailwind-input.css -o assets/tailwind-output.css --minify")
        .output()
    {
        Ok(output) => {
            if !output.status.success() {
                let _ = io::stdout().write_all(&output.stdout);
                let _ = io::stdout().write_all(&output.stderr);
                panic!("Tailwind error");
            }
        }
        Err(e) => panic!("Tailwind error: {:?}", e),
    };
}
