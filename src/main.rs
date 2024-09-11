use std::{fs, io::Write};
use libphext::phext;

#[derive(PartialEq, PartialOrd, Debug, Clone)]
struct PhextShellState
{
    pub filename:String,
    pub coordinate:phext::Coordinate,
    pub status:bool,
    pub phext:String,
    pub scroll:String,
    pub history:String
}

// -----------------------------------------------------------------------------------------------------------
// @fn main
// -----------------------------------------------------------------------------------------------------------
fn main() {
    let mut state:PhextShellState = PhextShellState {
        filename: String::new(),
        coordinate: phext::to_coordinate("1.1.1/1.1.1/1.1.1"),
        status: false,
        phext: String::new(),
        scroll: String::new(),
        history: String::new()
    };

    let args: Vec<String> = std::env::args().collect();
    if args.len() >= 2 {
        let request = args[1..].join(" ");
        handle_request(request, &mut state);
        return;
    }

    while state.status == false {
        let mut display_coordinate = state.coordinate.to_string();
        while display_coordinate.starts_with("1.1.1/") {
            display_coordinate = display_coordinate[6..].to_string();
        }
        print!("{} > ", display_coordinate);
        std::io::stdout().flush().expect("output error");

        let mut request = String::new();
        let total = std::io::stdin().read_line(&mut request).expect("Failed to read line");

        if total == 0 { continue; }

        handle_request(request, &mut state);
    }

    let filename = state.filename + ".history";
    let error_message = format!("Unable to save session history to {}", filename);
    fs::write(filename.clone(), state.history.as_bytes()).expect(error_message.as_str());
}

// -----------------------------------------------------------------------------------------------------------
fn file_exists(path: &str) -> bool {
    fs::metadata(path).is_ok()
}

fn dir_exists(path: &str) -> bool {
    let test = std::path::Path::new(path);
    test.is_dir()
}

// -----------------------------------------------------------------------------------------------------------
fn handle_request(request: String, state:&mut PhextShellState) {
    let trimmed = request.trim();
    let mut handled = false;

    // allow easy exits
    if trimmed.starts_with("exit") ||
       trimmed.starts_with("quit") ||
       trimmed.starts_with(":q!") {
        state.status = true;
        handled = true;
    }

    // git
    let have_git = dir_exists(".git");
    if have_git == false {
        /*
        On Windows, this snippet resulted in permissions errors - keeping it out for now
        use std::process::Command;
        let output = Command::new("git")
                .arg("init")
                .output()
                .expect("failed to run git");

        let program_output = String::from_utf8_lossy(&output.stdout).to_string();
        println!("{}", program_output);
        if output.stderr.len() > 0 {
            println!("Error: {}", String::from_utf8_lossy(&output.stderr));
        }
        */

        println!("* WARNING: You have not created a git repository yet.");
    }

    // rust
    if trimmed.starts_with("add-rust") {
        handled = add_rust(trimmed.to_string());
    }

    if handled == false {
        println!("Error: Unsupported command {}", trimmed);
    } else {
        println!("Done.");
    }
}

fn add_rust(_trimmed: String) -> bool {
    let rust_main = "src/main.rs";
    let have_src = dir_exists("src");
    let have_rust_main = file_exists(rust_main);
    let have_gitignore = file_exists(".gitignore");
    let have_toml = file_exists("Cargo.toml");

    if have_src == false {
        std::fs::create_dir("src").expect("Unable to create src");
        println!("* Created src");
    }

    if have_rust_main == false {            
        std::fs::write(rust_main, "use libphext::phext;
fn main() {
println!(\"Hello World\");
}
").expect("Unable to write src/main.rs");
        println!("* Added {}", rust_main);
    }
    if have_gitignore == false {

        std::fs::write(".gitignore", "/target
Cargo.lock").expect("Unable to add .gitignore");
        println!("* Added .gitignore");
    }
    if have_toml == false {
        std::fs::write("Cargo.toml", "[package]
name = \"new-project\"
version = \"0.1.0\"
edition = \"2021\"

[dependencies]
libphext = \"0.1.7\"
").expect("Unable to add Cargo.toml");
        println!("* Added Cargo.toml");
    }
    return true;
}