use data_cli::{command::Command, factory};

fn main() {
    let args = std::env::args();
    let command = Command::from(args);
    if !command.validate() {
        command.print_usage();
        return;
    }
    
    factory::run(&command);
}
