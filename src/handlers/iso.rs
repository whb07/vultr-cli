//! ISO command handlers

use crate::api::VultrClient;
use crate::commands::{IsoArgs, IsoCommands};
use crate::config::OutputFormat;
use crate::error::{VultrError, VultrResult};
use crate::handlers::confirm_delete;
use crate::models::CreateIsoRequest;
use crate::output::{print_output, print_success};

pub async fn handle_iso(
    args: IsoArgs,
    client: &VultrClient,
    output: OutputFormat,
    skip_confirm: bool,
) -> VultrResult<()> {
    match args.command {
        IsoCommands::List(list_args) => {
            if list_args.all {
                let mut all = Vec::new();
                let mut cursor: Option<String> = None;
                loop {
                    let (page, meta) = client
                        .list_isos(Some(list_args.per_page), cursor.as_deref())
                        .await?;
                    all.extend(page);
                    cursor = meta.links.and_then(|l| l.next);
                    if cursor.is_none() {
                        break;
                    }
                }
                print_output(&all, output);
            } else {
                let (isos, _) = client
                    .list_isos(Some(list_args.per_page), list_args.cursor.as_deref())
                    .await?;
                print_output(&isos, output);
            }
        }

        IsoCommands::Get { id } => {
            let iso = client.get_iso(&id).await?;
            print_output(&iso, output);
        }

        IsoCommands::Create { url } => {
            let iso = client.create_iso(CreateIsoRequest { url }).await?;
            print_success(&format!("ISO {} creation initiated", iso.id));
            print_output(&iso, output);
        }

        IsoCommands::Delete { id } => {
            if !skip_confirm && !confirm_delete("ISO", &id)? {
                return Err(VultrError::Cancelled);
            }
            client.delete_iso(&id).await?;
            print_success(&format!("ISO {} deleted", id));
        }

        IsoCommands::Public => {
            let public_isos = client.list_public_isos().await?;
            print_output(&public_isos, output);
        }
    }
    Ok(())
}
