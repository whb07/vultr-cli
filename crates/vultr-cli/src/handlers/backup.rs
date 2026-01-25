//! Backup command handlers

use crate::commands::{BackupArgs, BackupCommands};
use vultr_api::VultrClient;
use vultr_config::OutputFormat;
use vultr_config::VultrResult;
use vultr_output::print_output;

pub async fn handle_backup(
    args: BackupArgs,
    client: &VultrClient,
    output: OutputFormat,
) -> VultrResult<()> {
    match args.command {
        BackupCommands::List(list_args) => {
            if list_args.all {
                let mut all = Vec::new();
                let mut cursor: Option<String> = None;
                loop {
                    let (page, meta) = client
                        .list_backups(Some(list_args.per_page), cursor.as_deref())
                        .await?;
                    all.extend(page);
                    cursor = meta.links.and_then(|l| l.next);
                    if cursor.is_none() {
                        break;
                    }
                }
                print_output(&all, output);
            } else {
                let (backups, _) = client
                    .list_backups(Some(list_args.per_page), list_args.cursor.as_deref())
                    .await?;
                print_output(&backups, output);
            }
        }

        BackupCommands::Get { id } => {
            let backup = client.get_backup(&id).await?;
            print_output(&backup, output);
        }
    }
    Ok(())
}
