use crate::{
    config::ContainerOpts,
    error::Errcode
};
use nix::{
    unistd::Pid,
    sched::{ clone, CloneFlags },
    sys::signal::Signal,
};

const STACK_SIZE: usize = 1024 * 1024;

pub fn generate_child_process(config: ContainerOpts) -> Result<Pid, Errcode> {
    let mut tmp_stack: [u8; STACK_SIZE] = [0; STACK_SIZE];
    let mut flags = CloneFlags::empty();

    //Flags definition
    match clone(Box::new(|| child(config.clone())), &mut tmp_stack, flags, Some(Signal::SIGCHLD as i32)) {
        Ok(Pid) => Ok(Pid),
        Err(_) => Err(Errcode::ChildProcessError(0))
    }
}

fn child(config: ContainerOpts) -> isize {
    log::info!("Starting container with command {} and args {:?}", config.path.to_str().unwrap(), config.argv);
    0
}