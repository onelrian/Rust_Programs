use std::env;
use std::process;

fn main() {

    let current_dir = env::current_dir().unwrap_or_else(|err| {
        eprintln!("Failed to get current directory: {}", err);
        process::exit(1);
    });


    let os_name = if cfg!(target_os = "windows") {
        "Windows"
    } else if cfg!(target_os = "macos") {
        "macOS"
    } else if cfg!(target_os = "linux") {
        "Linux"
    } else {
        "Unknown OS"
    };


    let cpu_cores = num_cpus::get();


    println!("Current Working Directory: {}", current_dir.display());
    println!("OS Name: {}", os_name);
    println!("Number of CPU Cores: {}", cpu_cores);
}