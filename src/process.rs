use crate::config::ActionType;
use crate::logger::Logger;
use std::collections::HashMap;
use std::ffi::OsStr;
use std::process::Command;
use sysinfo::System;

pub fn is_process_running(sys: &System, process_name: &str) -> bool {
    sys.processes_by_name(OsStr::new(process_name))
        .next()
        .is_some()
}

pub fn close_process(
    sys: &System,
    process_name: &str,
    logger: &mut Logger,
) -> Result<(), Box<dyn std::error::Error>> {
    logger.info(&format!("Attempting to close program: {}", process_name));

    if !sys
        .processes_by_name(OsStr::new(process_name))
        .next()
        .is_some()
    {
        logger.info(&format!(
            "Program {} is not running, skipping.",
            process_name
        ));
        return Ok(());
    }

    let output = Command::new("taskkill")
        .args(["/F", "/IM", process_name])
        .output()?;

    if output.status.success() {
        logger.info(&format!("Successfully closed program: {}", process_name));
        Ok(())
    } else {
        let error_message = String::from_utf8_lossy(&output.stderr);
        Err(format!("Failed to close program: {}", error_message).into())
    }
}

pub fn start_process(
    process_name: &str,
    logger: &mut Logger,
) -> Result<(), Box<dyn std::error::Error>> {
    logger.info(&format!("Attempting to start program: {}", process_name));

    let output = Command::new("cmd")
        .args(["/C", "start", "", process_name])
        .spawn();

    match output {
        Ok(_) => {
            logger.info(&format!("Successfully started program: {}", process_name));
            Ok(())
        }
        Err(e) => Err(format!("Failed to start program {}: {}", process_name, e).into()),
    }
}

pub fn execute_actions(sys: &System, actions: &HashMap<String, ActionType>, logger: &mut Logger) {
    for (process_name, action_type) in actions {
        match action_type {
            ActionType::Close => match close_process(sys, process_name, logger) {
                Ok(_) => (),
                Err(e) => logger.error(&format!("Error closing program {}: {}", process_name, e)),
            },
            ActionType::Start => match start_process(process_name, logger) {
                Ok(_) => (),
                Err(e) => logger.error(&format!("Error starting program {}: {}", process_name, e)),
            },
        }
    }
}

pub fn close_processes(sys: &System, process_names: &[String], logger: &mut Logger) {
    for process_name in process_names {
        match close_process(sys, process_name, logger) {
            Ok(_) => (),
            Err(e) => logger.error(&format!("Error closing program {}: {}", process_name, e)),
        }
    }
}
