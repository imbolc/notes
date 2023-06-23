/// State machines in Rust
/// it uses a `next` method to move between states, rather then `From` trait
/// as the `next` method can be async
///
/// Blog posts:
/// - https://hoverbear.org/blog/rust-state-machine-pattern/
/// - https://blog.yoshuawuyts.com/state-machines/
/// - https://deislabs.io/posts/a-fistful-of-states/

fn main() {
    // InitState
    let state = Machine::new();
    // MiddleState
    let state = state.next();
    // Error state
    let state = state.error();
    // This won't compile as no transition allowed from the error state
    let state = state.next();
}

pub struct InitState;
pub struct MiddleState;
pub struct DoneState;
pub struct ErrorState;

#[derive(Debug)]
struct Machine<S> {
    state: S,
}

// Allow transition from any state to ErrorState
impl<T> Machine<T> {
    fn error(self) -> Machine<ErrorState> {
        Machine {
            state: ErrorState {},
        }
    }
}

impl Machine<InitState> {
    /// Create the machine in InitState
    pub fn new() -> Machine<InitState> {
        Machine {
            state: InitState {},
        }
    }

    /// Allow transition from InitState to MiddleState
    pub fn next(self) -> Machine<MiddleState> {
        Machine {
            state: MiddleState {},
        }
    }
}

/// Allow transition from MiddleState to DoneState
impl Machine<MiddleState> {
    pub fn next(self) -> Machine<DoneState> {
        Machine {
            state: DoneState {},
        }
    }
}
