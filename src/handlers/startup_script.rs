//! Startup script command handlers

use crate::api::{VultrClient, WaitOptions};
use crate::commands::{StartupScriptArgs, StartupScriptCommands};
use crate::config::OutputFormat;
use crate::error::{VultrError, VultrResult};
use crate::handlers::{confirm_delete, read_file_or_string};
use crate::models::{CreateStartupScriptRequest, UpdateStartupScriptRequest};
use crate::output::{print_output, print_success};

pub async fn handle_startup_script(
    args: StartupScriptArgs,
    client: &VultrClient,
    output: OutputFormat,
    skip_confirm: bool,
    wait_opts: &WaitOptions,
) -> VultrResult<()> {
    match args.command {
        StartupScriptCommands::List(list_args) => {
            if list_args.all {
                let mut all = Vec::new();
                let mut cursor: Option<String> = None;
                loop {
                    let (page, meta) = client
                        .list_startup_scripts(Some(list_args.per_page), cursor.as_deref())
                        .await?;
                    all.extend(page);
                    cursor = meta.links.and_then(|l| l.next);
                    if cursor.is_none() {
                        break;
                    }
                }
                print_output(&all, output);
            } else {
                let (scripts, _) = client
                    .list_startup_scripts(Some(list_args.per_page), list_args.cursor.as_deref())
                    .await?;
                print_output(&scripts, output);
            }
        }

        StartupScriptCommands::Get { id } => {
            let script = client.get_startup_script(&id).await?;
            print_output(&script, output);
        }

        StartupScriptCommands::Create {
            name,
            script,
            script_type,
        } => {
            let content = read_file_or_string(&script)?;
            let req = CreateStartupScriptRequest::new(
                name,
                &content,
                Some(
                    script_type
                        .parse()
                        .map_err(|e: String| VultrError::InvalidInput(e))?,
                ),
            );
            let script = client.create_startup_script(req).await?;
            print_success(&format!("Startup script {} created", script.id));
            print_output(&script, output);
        }

        StartupScriptCommands::Update {
            id,
            name,
            script,
            script_type,
        } => {
            let mut req = UpdateStartupScriptRequest {
                name,
                script_type,
                ..Default::default()
            };
            if let Some(s) = script {
                req = req.with_raw_script(&read_file_or_string(&s)?);
            }
            client.update_startup_script(&id, req).await?;
            print_success(&format!("Startup script {} updated", id));
        }

        StartupScriptCommands::Delete { id } => {
            if !skip_confirm && !confirm_delete("startup script", &id)? {
                return Err(VultrError::Cancelled);
            }
            client.delete_startup_script(&id).await?;
            print_success(&format!("Startup script {} deletion initiated", id));
            crate::api::verify_startup_script_deleted(client, &id, wait_opts).await?;
        }
    }
    Ok(())
}
