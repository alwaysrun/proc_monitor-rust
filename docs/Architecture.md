# Architecture

## Overview

**Proc Monitor** is a Windows process monitoring utility that automatically closes specified programs when certain processes are detected running.

### Use Case

When launching a game (e.g., Steam), automatically close certain background applications (e.g., VPN proxies) to avoid conflicts or improve performance.

## System Architecture

```mermaid
graph TB
    subgraph "Entry Layer"
        A[main.rs] --> B[CLI Parsing]
        A --> C[Background Mode]
    end

    subgraph "Core Modules"
        D[config.rs] --> E[Configuration Loading]
        F[logger.rs] --> G[Logging System]
        H[process.rs] --> I[Process Management]
        J[cli.rs] --> K[Argument Parsing]
    end

    subgraph "External Dependencies"
        L[sysinfo] --> I
        M[toml/serde] --> E
        N[chrono] --> G
    end

    B --> K
    C --> A
    A --> D
    A --> F
    A --> H
```

## Module Structure

```mermaid
flowchart LR
    subgraph "Source Files"
        direction TB
        M[main.rs<br/>Entry Point]
        L[lib.rs<br/>Module Exports]
        C[config.rs<br/>Configuration]
        G[logger.rs<br/>Logging]
        P[process.rs<br/>Process Mgmt]
        CL[cli.rs<br/>CLI Args]
    end

    M --> L
    L --> C
    L --> G
    L --> P
    L --> CL
```

## Process Flow

```mermaid
flowchart TD
    Start([Program Start]) --> ParseArgs{Parse CLI Args}
    
    ParseArgs -->|--help| ShowHelp[Show Help Message]
    ShowHelp --> Exit([Exit])
    
    ParseArgs -->|-b/--background| SpawnBackground[Spawn Background Process]
    SpawnBackground --> Exit
    
    ParseArgs -->|No args/-l| InitLogger[Initialize Logger]
    
    InitLogger --> LoadConfig[Load config.toml]
    LoadConfig --> |Success| InitSystem[Initialize System Info]
    LoadConfig --> |Failure| LogError[Log Error] --> Exit
    
    InitSystem --> InitState[Initialize Detection States]
    InitState --> MainLoop[/Main Loop/]
    
    MainLoop --> RefreshSystem[Refresh Process List]
    RefreshSystem --> CheckProcess{For Each Monitored Process}
    
    CheckProcess --> IsRunning{Is Process Running?}
    IsRunning -->|Yes| WasDetected{Was Previously Detected?}
    IsRunning -->|No| WasDetected2{Was Previously Detected?}
    
    WasDetected -->|No| LogStart[Log: Process Started]
    LogStart --> CloseProgs[Close Configured Programs]
    CloseProgs --> SetDetected[Set Detected = true]
    
    WasDetected -->|Yes| Skip[Skip - Already Handled]
    
    WasDetected2 -->|Yes| LogStop[Log: Process Stopped]
    LogStop --> ResetDetected[Set Detected = false]
    WasDetected2 -->|No| Skip2[Skip - Not Running]
    
    SetDetected --> Sleep
    ResetDetected --> Sleep
    Skip --> Sleep
    Skip2 --> Sleep
    
    Sleep[Sleep for check_interval] --> MainLoop
```

## State Machine

```mermaid
flowchart LR
    subgraph NotDetected["NotDetected State"]
        direction LR
        ND_Desc["State: false<br/>Action: None"]
    end
    
    subgraph Detected["Detected State"]
        direction LR
        D_Desc["State: true<br/>Action: Close programs"]
    end
    
    Start([Start]) --> NotDetected
    
    NotDetected -->|"Process Starts"| Detected
    Detected -->|"Process Stops"| NotDetected
```

## Background Mode Implementation

```mermaid
sequenceDiagram
    participant User
    participant Main
    participant Child
    
    User->>Main: proc_monitor -b
    Main->>Main: Parse args, detect -b
    Main->>Child: Spawn with -l flag
    Note over Child: CREATE_NO_WINDOW flag
    Main->>User: "Background process started"
    Main->>Main: Exit
    
    loop Monitor Loop
        Child->>Child: Check processes
        Child->>Child: Log to file
        Child->>Child: Sleep
    end
```

## Configuration Loading Strategy

```mermaid
flowchart TD
    A[Get Config Path] --> B{Exists in CWD?}
    B -->|Yes| C[Use CWD path]
    B -->|No| D{Exists in Exe Dir?}
    D -->|Yes| E[Use Exe Dir path]
    D -->|No| F[Return CWD path<br/>Will error on load]
    
    C --> G[Load & Parse TOML]
    E --> G
    F --> G
```

## Key Design Decisions

| Decision | Rationale |
|----------|-----------|
| Module separation | Single responsibility, testability |
| State machine for detection | Edge-triggered logic prevents repeated actions |
| taskkill over native API | Simplicity, reliability, force close support |
| Dual logging mode | Flexibility for foreground/background operation |
| TOML configuration | Human-readable, easy to edit |
