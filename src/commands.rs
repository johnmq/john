use river::River;
pub use river::PeekResult;

/// Push command - stateless
///
/// Used to push messages to rivers like this:
///
/// ```
/// john::PushCommand::new().execute("river_name", "message");
/// ```
pub struct PushCommand;

impl PushCommand {
    /// Constructor ::new()
    ///
    /// Creates new instance of PushCommand
    pub fn new() -> PushCommand {
        PushCommand
    }

    /// Used to execute push command, specifying a river name and message
    /// This can be called multiple times with different arguments
    /// since PushCommand is stateless
    pub fn execute(&self, river: &str, message: &str) {
        River::new(river).push(message);
    }
}

/// Peek command - stateless
///
/// Used to peek messages from rivers like this:
///
/// ```
/// // read latest message from river
/// john::PushCommand::new().execute("river name", "a message");
/// john::PushCommand::new().execute("river name", "a message 1");
/// john::PushCommand::new().execute("river name", "a message 2");
/// john::PushCommand::new().execute("river name", "a message 3");
/// john::PeekCommand::new().execute("river name", None);
///
/// // read message from river at specific offset
/// john::PeekCommand::new().execute("river name", Some(2));
/// ```
///
/// It returns Option < PeekResult >. When it was able to peek a message, the result will contain
/// peeked message and new offset to specify to peek command (if you want to get next message)
pub struct PeekCommand;

impl PeekCommand {
    /// Constructor ::new()
    ///
    /// Creates new instance of PeekCommand
    pub fn new() -> PeekCommand {
        PeekCommand
    }

    /// Used to execute peek command, specifying a river name and optionally offset to peek at
    pub fn execute(&self, river: &str, offset: Option < uint >) -> Option < PeekResult > {
        River::new(river).peek_at(offset)
    }
}

/// Clear command - stateless
///
/// Used to clear messages from rivers like this:
///
/// ```
/// john::ClearCommand::new().execute("river_name");
/// ```
pub struct ClearCommand;

impl ClearCommand {
    /// Constructor ::new()
    ///
    /// Creates new instance of ClearCommand
    pub fn new() -> ClearCommand {
        ClearCommand
    }

    /// Used to execute push command, specifying a river name and message
    /// This can be called multiple times with different arguments
    /// since PushCommand is stateless
    pub fn execute(&self, river: &str) {
        River::new(river).destroy();
    }
}

