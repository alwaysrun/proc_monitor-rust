use proc_monitor::{load_config, parse_args, print_help, CliArgs, Logger};
use std::env;
use std::time::Duration;
use sysinfo::{ProcessesToUpdate, System};

fn ensure_config_directory(logger: &mut Logger) -> Result<(), Box<dyn std::error::Error>> {
    if let Ok(exe_path) = env::current_exe() {
        if let Some(exe_dir) = exe_path.parent() {
            let target_config_dir = exe_dir.join("configure");
            if !target_config_dir.exists() {
                logger.warning(&format!(
                    "Configuration directory not found: {}",
                    target_config_dir.display()
                ));
                logger.warning(
                    "Note: The configuration directory should be in the same directory as the executable."
                );
            }
        }
    }
    Ok(())
}

fn run_in_background() -> Result<(), Box<dyn std::error::Error>> {
    let exe_path = env::current_exe()?;
    let args: Vec<String> = vec![exe_path.to_string_lossy().to_string(), "-l".to_string()];

    #[cfg(target_os = "windows")]
    {
        use std::os::windows::process::CommandExt;

        const CREATE_NO_WINDOW: u32 = 0x08000000;

        std::process::Command::new(&args[0])
            .args(&args[1..])
            .creation_flags(CREATE_NO_WINDOW)
            .spawn()?;
    }

    #[cfg(not(target_os = "windows"))]
    {
        std::process::Command::new(&args[0])
            .args(&args[1..])
            .spawn()?;
    }

    Ok(())
}

fn run_monitor(cli_args: CliArgs) -> Result<(), Box<dyn std::error::Error>> {
    let mut logger = Logger::new(cli_args.log_to_file)?;

    logger.info("#### Proc Monitor Started - Process Monitoring and Auto-closing");

    ensure_config_directory(&mut logger)?;

    let config = load_config()?;
    logger.info("Configuration loaded successfully:");
    for (index, process_config) in config.monitor.process.iter().enumerate() {
        logger.info(&format!("  Process #{}:", index + 1));
        logger.info(&format!("    Monitored: {}", process_config.monitored));
        logger.info(&format!("    Actions: {:?}", process_config.action));
        logger.info(&format!(
            "    Check interval: {} seconds",
            process_config.check_interval
        ));
    }

    let mut sys = System::new_all();
    let mut process_detections = vec![false; config.monitor.process.len()];

    let check_interval = config
        .monitor
        .process
        .iter()
        .map(|p| p.check_interval)
        .min()
        .unwrap_or(10);

    logger.info(&format!(
        "Starting to monitor {} processes with minimum interval of {} seconds",
        config.monitor.process.len(),
        check_interval
    ));

    loop {
        sys.refresh_processes(ProcessesToUpdate::All, true);

        for (index, process_config) in config.monitor.process.iter().enumerate() {
            let is_running = proc_monitor::is_process_running(&sys, &process_config.monitored);
            let detected = &mut process_detections[index];

            if is_running && !*detected {
                logger.info(&format!(
                    "Detected process {} has started",
                    process_config.monitored
                ));
                logger.info(&format!(
                    "Executing configured actions for {}...",
                    process_config.monitored
                ));
                proc_monitor::execute_actions(&sys, &process_config.action, &mut logger);
                *detected = true;
            } else if !is_running && *detected {
                logger.info(&format!(
                    "Process {} has stopped running",
                    process_config.monitored
                ));
                *detected = false;
            }
        }

        std::thread::sleep(Duration::from_secs(check_interval));
    }
}

fn main() {
    let cli_args = parse_args();

    if cli_args.show_help {
        print_help();
        return;
    }

    if cli_args.is_background {
        println!("Starting Proc Monitor in background mode...");
        if let Err(e) = run_in_background() {
            eprintln!("Failed to start background process: {}", e);
            std::process::exit(1);
        }
        println!("Background process started successfully.");
        return;
    }

    if let Err(e) = run_monitor(cli_args) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
