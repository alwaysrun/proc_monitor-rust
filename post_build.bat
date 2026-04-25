@echo off
REM Build the project
REM cargo build

REM Copy executable to output directory
if exist "D:\work\_sync\self\learning\rust_study\proc_monitor\target\debug\proc_monitor.exe" (
    echo Copying executable to output directory...
    xcopy "D:\work\_sync\self\learning\rust_study\proc_monitor\target\debug\proc_monitor.exe" "D:\work\_sync\self\learning\rust_study\proc_monitor\target\output" /I /Y
    echo Executable copied successfully
) else (
    echo Error: Executable not found at D:\work\_sync\self\learning\rust_study\proc_monitor\target\debug\proc_monitor.exe
)

REM Check if configuration directory was copied successfully
if exist "D:\work\_sync\self\learning\rust_study\proc_monitor\target\output\configure" (
    echo Configuration directory exists in output folder
) else (
    echo Warning: Configuration directory not found in output folder
)
 