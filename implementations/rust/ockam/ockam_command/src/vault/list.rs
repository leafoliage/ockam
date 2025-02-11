use clap::Args;

use ockam_api::cli_state::traits::StateDirTrait;

use crate::{docs, CommandGlobalOpts};

const LONG_ABOUT: &str = include_str!("./static/list/long_about.txt");
const AFTER_LONG_HELP: &str = include_str!("./static/list/after_long_help.txt");

/// List vaults
#[derive(Clone, Debug, Args)]
#[command(
    long_about = docs::about(LONG_ABOUT),
    after_long_help = docs::after_help(AFTER_LONG_HELP)
)]
pub struct ListCommand;

impl ListCommand {
    pub fn run(self, opts: CommandGlobalOpts) {
        if let Err(e) = run_impl(opts) {
            eprintln!("{e}");
            std::process::exit(e.code());
        }
    }
}

fn run_impl(opts: CommandGlobalOpts) -> crate::Result<()> {
    let vaults = opts.state.vaults.list()?;
    let list = opts
        .terminal
        .build_list(&vaults, "Vaults", "No vaults found on this system.")?;
    opts.terminal.stdout().plain(list).write_line()?;
    Ok(())
}
