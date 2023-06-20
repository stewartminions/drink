use clap::Parser;

#[derive(Parser)]
pub enum CliCommand {
    #[clap(alias = "b")]
    Build,
    #[clap(alias = "d")]
    Deploy,
    CallGet,
    CallFlip,
    #[clap(alias = "e")]
    Exit,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_cli() {
        use clap::CommandFactory;
        CliCommand::command().debug_assert()
    }
}