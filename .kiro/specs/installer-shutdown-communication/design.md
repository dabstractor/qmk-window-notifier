# Design Document

## Overview

This design addresses the installer shutdown communication issue by implementing a coordinated shutdown system that ensures proper communication between the Windows installer and the QMK Window Notifier tray application. The solution involves refactoring the existing shutdown handlers into a unified system that can respond to installer requests within required timeouts while maintaining normal application functionality.

## Architecture

### Current Architecture Issues

The current implementation has several uncoordinated shutdown mechanisms:
- Console control handler (for Ctrl+C events)
- Hidden window message handler (for WM_QUERYENDSESSION/WM_ENDSESSION)
- Named mutex signaling system
- Tray event loop with WindowsShutdown user event

These handlers run in separate threads and don't coordinate their responses, leading to race conditions and unreliable shutdown behavior.

### Proposed Architecture

The new architecture implements a **Centralized Shutdown Coordinator** that:

1. **Single Point of Control**: All shutdown signals route through one coordinator
2. **Immediate Response**: Responds to installer queries immediately while initiating shutdown
3. **Timeout Management**: Enforces shutdown timeouts with fallback to process termination
4. **Thread Coordination**: Properly coordinates between event loop and shutdown threads

```
┌─────────────────┐    ┌──────────────────────┐    ┌─────────────────┐
│   Installer     │───▶│  Shutdown Coordinator │───▶│  Tray Event     │
│   (WiX)         │    │                      │    │  Loop           │
└─────────────────┘    └──────────────────────┘    └─────────────────┘
                                │
                                ▼
                       ┌──────────────────────┐
                       │  Process Termination │
                       │  (with timeout)      │
                       └──────────────────────┘
```

## Components and Interfaces

### 1. ShutdownCoordinator

**Purpose**: Central manager for all shutdown operations

**Interface**:
```rust
pub struct ShutdownCoordinator {
    event_proxy: EventLoopProxy<UserEvent>,
    shutdown_initiated: AtomicBool,
    shutdown_timeout: Duration,
}

impl ShutdownCoordinator {
    pub fn new(proxy: EventLoopProxy<UserEvent>) -> Self;
    pub fn initiate_shutdown(&self, source: ShutdownSource) -> bool;
    pub fn register_handlers(&self) -> Result<(), Box<dyn Error>>;
    pub fn is_shutdown_initiated(&self) -> bool;
}
```

**Responsibilities**:
- Receive shutdown signals from all sources
- Coordinate immediate response to installer queries
- Manage shutdown timeouts
- Ensure process termination

### 2. ShutdownSource Enum

**Purpose**: Track the source of shutdown requests for appropriate handling

```rust
pub enum ShutdownSource {
    Installer,           // From WiX CloseApplication
    SystemShutdown,      // From Windows shutdown/logoff
    UserRequest,         // From tray menu quit
    ConsoleSignal,       // From Ctrl+C or console close
}
```

### 3. Enhanced Message Handlers

**WindowMessageHandler**: Handles WM_QUERYENDSESSION and WM_ENDSESSION
- Immediately returns TRUE to WM_QUERYENDSESSION
- Signals shutdown coordinator
- Implements timeout-based force termination

**ConsoleControlHandler**: Handles console control events
- Routes signals through shutdown coordinator
- Maintains existing Ctrl+C behavior

### 4. Singleton Manager Improvements

**Purpose**: Fix mutex handling during installer scenarios

**Interface**:
```rust
pub struct SingletonManager {
    instance: OnceLock<SingleInstance>,
    mutex_name: String,
}

impl SingletonManager {
    pub fn new(name: &str) -> Self;
    pub fn is_already_running(&self) -> Result<bool, Box<dyn Error>>;
    pub fn release_on_shutdown(&self);
    pub fn force_reset(&self) -> Result<(), Box<dyn Error>>;
}
```

## Data Models

### ShutdownState

```rust
#[derive(Debug, Clone)]
pub struct ShutdownState {
    pub initiated: bool,
    pub source: Option<ShutdownSource>,
    pub timestamp: Instant,
    pub timeout_duration: Duration,
}
```

### ShutdownConfig

```rust
#[derive(Debug, Clone)]
pub struct ShutdownConfig {
    pub installer_response_timeout: Duration,    // 5 seconds
    pub graceful_shutdown_timeout: Duration,     // 3 seconds
    pub force_termination_timeout: Duration,     // 2 seconds
    pub enable_force_termination: bool,          // true
}
```

## Error Handling

### Timeout Scenarios

1. **Installer Response Timeout**: If shutdown coordinator can't respond within 5 seconds
   - Log error to Windows Event Log
   - Force immediate process termination
   - Return failure to installer (handled by force termination)

2. **Graceful Shutdown Timeout**: If event loop doesn't exit within 3 seconds
   - Log warning
   - Initiate force termination thread
   - Exit process with code 0

3. **Force Termination Timeout**: If process doesn't exit within 2 seconds
   - Call `std::process::exit(0)` directly
   - As last resort, call `std::process::abort()`

### Error Recovery

1. **Mutex Conflicts**: If singleton mutex is in invalid state
   - Attempt to reset mutex handle
   - Log warning and continue with startup
   - Don't block application startup

2. **Handler Registration Failures**: If Windows API calls fail
   - Log errors but continue startup
   - Fallback to basic shutdown handling
   - Ensure core functionality remains available

## Testing Strategy

### Unit Tests

1. **ShutdownCoordinator Tests**
   - Test immediate response to installer queries
   - Test timeout enforcement
   - Test coordination between handlers
   - Test multiple shutdown signal handling

2. **SingletonManager Tests**
   - Test normal singleton behavior
   - Test mutex release on shutdown
   - Test recovery from invalid mutex state
   - Test installer scenario handling

### Integration Tests

1. **Installer Simulation Tests**
   - Mock WiX CloseApplication behavior
   - Test WM_QUERYENDSESSION/WM_ENDSESSION handling
   - Verify response timing requirements
   - Test force termination scenarios

2. **System Shutdown Tests**
   - Test Windows shutdown/logoff scenarios
   - Test console control signal handling
   - Verify graceful shutdown behavior
   - Test timeout and fallback mechanisms

### Manual Testing

1. **Real Installer Testing**
   - Test with actual MSI installer
   - Verify no timeout errors during installation
   - Test upgrade scenarios
   - Test installation cancellation

2. **System Integration Testing**
   - Test Windows shutdown behavior
   - Test user logoff scenarios
   - Test task manager termination
   - Test multiple instance prevention

## Implementation Notes

### Thread Safety

- Use `Arc<AtomicBool>` for shutdown state sharing
- Use `OnceLock` for singleton instance management
- Minimize shared mutable state
- Use message passing where possible

### Windows API Considerations

- Handle API failures gracefully
- Use proper error codes for installer communication
- Ensure proper cleanup of Windows handles
- Follow Windows shutdown protocol requirements

### Performance Impact

- Minimal overhead during normal operation
- Shutdown handlers only active when needed
- No impact on tray functionality
- Fast response to shutdown signals (< 100ms)

### Backward Compatibility

- Maintain existing tray menu functionality
- Preserve existing command-line arguments
- Keep existing configuration behavior
- No changes to user-visible features