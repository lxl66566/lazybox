use clap::Command;
mod test;
mod to_utf8;

pub fn functions(cmd: Command) -> Command {
    cmd.subcommand(test::command())
}

#[macro_export]
macro_rules! handle_matches {
    ($command:ident, $matches:ident, { $($cmd:ident),* }) => {
        match $matches.subcommand() {
            $(Some((stringify!($cmd), sub_matches)) => {
                $cmd::execute(sub_matches);
            })*
            _ => {},
        }
    };
}

pub fn execute(cmd: Command) {
    let matched = cmd.get_matches();
    handle_matches!(cmd, matched, { test });
}
