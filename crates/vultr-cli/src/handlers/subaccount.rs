//! Subaccount command handlers

use vultr_api::VultrClient;
use crate::commands::{SubaccountArgs, SubaccountCommands};
use vultr_config::OutputFormat;
use vultr_config::VultrResult;
use vultr_models::CreateSubaccountRequest;
use vultr_output::print_output;

pub async fn handle_subaccount(
    args: SubaccountArgs,
    client: &VultrClient,
    output: OutputFormat,
    _skip_confirm: bool,
) -> VultrResult<()> {
    match args.command {
        SubaccountCommands::List(list_args) => {
            if list_args.all {
                let mut all = Vec::new();
                let mut cursor: Option<String> = None;
                loop {
                    let (page, meta) = client
                        .list_subaccounts(Some(list_args.per_page), cursor.as_deref())
                        .await?;
                    all.extend(page);
                    cursor = meta.and_then(|m| m.links.and_then(|l| l.next));
                    if cursor.is_none() {
                        break;
                    }
                }
                print_output(&all, output);
            } else {
                let (subs, _) = client
                    .list_subaccounts(Some(list_args.per_page), list_args.cursor.as_deref())
                    .await?;
                print_output(&subs, output);
            }
        }
        SubaccountCommands::Create {
            email,
            name,
            subaccount_id,
        } => {
            let request = CreateSubaccountRequest {
                email,
                subaccount_name: name,
                subaccount_id,
            };
            let subaccount = client.create_subaccount(request).await?;
            print_output(&subaccount, output);
        }
    }
    Ok(())
}
