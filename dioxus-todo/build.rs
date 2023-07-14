use std::process::Command;

fn main() {
    Command::new("npx")
        .args([
            "tailwind",
            "-i",
            "input.css",
            "-c",
            "tailwind.config.js",
            "-o",
            "dist/output.css",
            "--minify",
        ])
        .spawn()
        .unwrap();
}
