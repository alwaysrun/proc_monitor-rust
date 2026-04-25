use std::env;

pub struct CliArgs {
    pub log_to_file: bool,
    pub is_background: bool,
    pub show_help: bool,
}

impl Default for CliArgs {
    fn default() -> Self {
        Self {
            log_to_file: false,
            is_background: false,
            show_help: false,
        }
    }
}

pub fn parse_args() -> CliArgs {
    let args: Vec<String> = env::args().collect();
    let mut cli_args = CliArgs::default();

    for arg in &args[1..] {
        match arg.as_str() {
            "-b" | "--background" => {
                cli_args.is_background = true;
                cli_args.log_to_file = true;
            }
            "-l" | "--log_file" => cli_args.log_to_file = true,
            "-h" | "--help" => cli_args.show_help = true,
            _ => {
                eprintln!("Unknown argument: {}", arg);
                eprintln!("Use --help for usage information");
                std::process::exit(1);
            }
        }
    }

    cli_args
}

pub fn print_help() {
    println!("Proc Monitor - Process Monitoring and Auto-closing");
    println!("Usage:");
    println!("  proc_monitor                  Run in foreground with console output");
    println!(
        "  proc_monitor -b/--background  Run in background mode (logs to proc_monitor.log)"
    );
    println!("  proc_monitor -l/--log_file    Start a background instance and exit");
    println!("  proc_monitor -h/--help        Show this help message");
}
