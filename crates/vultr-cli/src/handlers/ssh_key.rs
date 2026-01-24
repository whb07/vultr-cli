//! SSH key command handlers

use vultr_api::{VultrClient, WaitOptions};
use crate::commands::{SshKeyArgs, SshKeyCommands};
use vultr_config::OutputFormat;
use vultr_config::{VultrError, VultrResult};
use crate::handlers::{confirm_delete, read_file_or_string};
use vultr_models::{CreateSshKeyRequest, UpdateSshKeyRequest};
use vultr_output::{print_output, print_success};

pub async fn handle_ssh_key(
    args: SshKeyArgs,
    client: &VultrClient,
    output: OutputFormat,
    skip_confirm: bool,
    wait: bool,
    wait_opts: &WaitOptions,
) -> VultrResult<()> {
    match args.command {
        SshKeyCommands::List(list_args) => {
            if list_args.all {
                let mut all = Vec::new();
                let mut cursor: Option<String> = None;
                loop {
                    let (page, meta) = client
                        .list_ssh_keys(Some(list_args.per_page), cursor.as_deref())
                        .await?;
                    all.extend(page);
                    cursor = meta.links.and_then(|l| l.next);
                    if cursor.is_none() {
                        break;
                    }
                }
                print_output(&all, output);
            } else {
                let (keys, _) = client
                    .list_ssh_keys(Some(list_args.per_page), list_args.cursor.as_deref())
                    .await?;
                print_output(&keys, output);
            }
        }

        SshKeyCommands::Get { id } => {
            let key = client.get_ssh_key(&id).await?;
            print_output(&key, output);
        }

        SshKeyCommands::Create { name, key } => {
            let ssh_key = read_file_or_string(&key)?;
            let key = client
                .create_ssh_key(CreateSshKeyRequest { name, ssh_key })
                .await?;
            print_success(&format!("SSH key {} created", key.id));
            print_output(&key, output);
        }

        SshKeyCommands::Update { id, name, key } => {
            client
                .update_ssh_key(
                    &id,
                    UpdateSshKeyRequest {
                        name,
                        ssh_key: key.map(|k| read_file_or_string(&k)).transpose()?,
                    },
                )
                .await?;
            print_success(&format!("SSH key {} updated", id));
        }

        SshKeyCommands::Delete { id } => {
            if !skip_confirm && !confirm_delete("SSH key", &id)? {
                return Err(VultrError::Cancelled);
            }
            client.delete_ssh_key(&id).await?;
            print_success(&format!("SSH key {} deletion initiated", id));
            if wait {
                vultr_api::verify_ssh_key_deleted(client, &id, wait_opts).await?;
            }
        }
    }
    Ok(())
}
