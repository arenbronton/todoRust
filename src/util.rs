pub fn display_options() {
    println!(
        "{}1. Mark Task As Complete\n2. Create New Task\n3. Edit Task\n4. Delete Task\n5. List Tasks\n6. Quit{}",
        colors::YELLOW, colors::RESET
    );
}

#[allow(unused)]
pub mod colors {
    pub const RED: &str = "\x1b[31m";
    pub const GREEN: &str = "\x1b[32m";
    pub const YELLOW: &str = "\x1b[33m";
    pub const BLUE: &str = "\x1b[34m";
    pub const MAGENTA: &str = "\x1b[35m";
    pub const CYAN: &str = "\x1b[36m";
    pub const WHITE: &str = "\x1b[37m";
    pub const RESET: &str = "\x1b[0m";
}

pub fn input(prompt: &str) -> String {
    use std::io::Write;
    let mut input = String::new();
    print!("{}", prompt.to_string());
    let _ = std::io::stdout().flush();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    return input.trim().to_string();
}
