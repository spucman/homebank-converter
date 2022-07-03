use clap::Command;

pub fn create_cli() {
    Command::new("homebank-converter")
        .about("Homebank-Converter CLIs")
        .subcommand_required(true)
        .subcommand(Command::new(""));
}
