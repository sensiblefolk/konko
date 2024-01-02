use std::{ fmt, process::exit };
use crate::container::MINIMAL_KERNEL_VERSION;

// Allows to display a variant with the format {:?}
#[derive(Debug)]
// Contains all possible errors in our tool
pub enum Errcode {
    ArgumentInvalid(String),
    NotSupported(u8),
    ContainerError(u8),
    SocketError(u8),
    ChildProcessError(u8)
}

impl Errcode {
    // Translate an Errcode::X into a number to return the correct UNIX way
    pub fn get_retcode(&self) -> i32 {
        1 // Everything != 0 will be treated as an error
    }
}

// Get the result from a function, and exit the process with the correct exit code
pub fn exit_with_retcode(res: Result<(), Errcode>) {
    match res {
        Ok(_) => {
            log::debug!("Exit without any error, returning 0");
            exit(0)
        },
        Err(e) => {
            let retcode = e.get_retcode();
            log::error!("Error on exit:\n\t{}\n\tReturning {}", e, retcode);
            exit(retcode)
        }
    }
}

#[allow(unreachable_patterns)]
impl fmt::Display for Errcode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            // Message to display when an argument is invalid
            Errcode::ArgumentInvalid(element) => write!(f, "ArgumentInvalid: {}", element),
            _ => write!(f, "{:?}", self),
            Errcode::NotSupported(errtype) => {
                match errtype {
                    0 => write!(f, "Minimal kernel version required: {}", MINIMAL_KERNEL_VERSION),
                    1 => write!(f, "Only x86_64 architecture is supported"),
                    _ => write!(f, "{:?} (unknown id)", self),
                }
            },
        }
    }
}