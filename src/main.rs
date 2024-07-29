pub mod functions;

use clap::Command;

fn main() -> anyhow::Result<()> {
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .format_target(false)
        .format_timestamp(None)
        .init();

    let mut cmd = Command::new(env!("CARGO_PKG_NAME"))
        .infer_subcommands(true)
        .subcommand_required(true);
    cmd = functions::functions(cmd);
    functions::execute(cmd);
    Ok(())
}
