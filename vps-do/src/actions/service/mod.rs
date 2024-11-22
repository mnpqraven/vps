use crate::utils::args::ServiceCommands;
use list::list_running_service;

pub mod build;
pub mod deploy;
pub mod list;

pub fn handle_service_arg(arg: &ServiceCommands) {
    match arg {
        ServiceCommands::List => list_running_service(),
        ServiceCommands::Deploy => todo!(),
    }
}
