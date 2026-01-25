//! User command handlers

use crate::commands::{UserArgs, UserCommands};
use crate::handlers::confirm_delete;
use vultr_api::VultrClient;
use vultr_config::OutputFormat;
use vultr_config::{VultrError, VultrResult};
use vultr_models::{
    AddIpWhitelistRequest, CreateApiKeyRequest, CreateUserRequest, DeleteIpWhitelistRequest,
    UpdateUserRequest,
};
use vultr_output::{print_output, print_success};

pub async fn handle_user(
    args: UserArgs,
    client: &VultrClient,
    output: OutputFormat,
    skip_confirm: bool,
) -> VultrResult<()> {
    match args.command {
        UserCommands::List(list_args) => {
            if list_args.all {
                let mut all = Vec::new();
                let mut cursor: Option<String> = None;
                loop {
                    let (page, meta) = client
                        .list_users(Some(list_args.per_page), cursor.as_deref())
                        .await?;
                    all.extend(page);
                    cursor = meta.and_then(|m| m.links.and_then(|l| l.next));
                    if cursor.is_none() {
                        break;
                    }
                }
                print_output(&all, output);
            } else {
                let (users, _) = client
                    .list_users(Some(list_args.per_page), list_args.cursor.as_deref())
                    .await?;
                print_output(&users, output);
            }
        }

        UserCommands::Get { id } => {
            let user = client.get_user(&id).await?;
            print_output(&user, output);
        }

        UserCommands::Create {
            email,
            name,
            password,
            api_enabled,
            acls,
        } => {
            let user = client
                .create_user(CreateUserRequest {
                    email,
                    name,
                    password,
                    api_enabled,
                    acls,
                })
                .await?;
            print_success(&format!("User {} created", user.id));
            print_output(&user, output);
        }

        UserCommands::Update {
            id,
            name,
            email,
            password,
            api_enabled,
            acls,
        } => {
            client
                .update_user(
                    &id,
                    UpdateUserRequest {
                        name,
                        email,
                        password,
                        api_enabled,
                        acls,
                    },
                )
                .await?;
            print_success(&format!("User {} updated", id));
        }

        UserCommands::Delete { id } => {
            if !skip_confirm && !confirm_delete("user", &id)? {
                return Err(VultrError::Cancelled);
            }
            client.delete_user(&id).await?;
            print_success(&format!("User {} deleted", id));
        }

        UserCommands::ApiKeys(api_key_args) => match api_key_args.command {
            crate::commands::UserApiKeyCommands::List(list_args) => {
                if list_args.list_args.all {
                    let mut all = Vec::new();
                    let mut cursor: Option<String> = None;
                    loop {
                        let (page, meta) = client
                            .list_user_api_keys(
                                &list_args.user_id,
                                Some(list_args.list_args.per_page),
                                cursor.as_deref(),
                            )
                            .await?;
                        all.extend(page);
                        cursor = meta.and_then(|m| m.links.and_then(|l| l.next));
                        if cursor.is_none() {
                            break;
                        }
                    }
                    print_output(&all, output);
                } else {
                    let (keys, _) = client
                        .list_user_api_keys(
                            &list_args.user_id,
                            Some(list_args.list_args.per_page),
                            list_args.list_args.cursor.as_deref(),
                        )
                        .await?;
                    print_output(&keys, output);
                }
            }

            crate::commands::UserApiKeyCommands::Create { user_id, name } => {
                let key = client
                    .create_user_api_key(&user_id, CreateApiKeyRequest { name })
                    .await?;
                print_success("API key created");
                print_output(&key, output);
            }

            crate::commands::UserApiKeyCommands::Delete {
                user_id,
                api_key_id,
            } => {
                if !skip_confirm && !confirm_delete("API key", &api_key_id)? {
                    return Err(VultrError::Cancelled);
                }
                client.delete_user_api_key(&user_id, &api_key_id).await?;
                print_success(&format!("API key {} deleted", api_key_id));
            }
        },

        UserCommands::IpWhitelist(ip_args) => match ip_args.command {
            crate::commands::UserIpWhitelistCommands::List { user_id } => {
                let entries = client.list_user_ip_whitelist(&user_id).await?;
                print_output(&entries, output);
            }

            crate::commands::UserIpWhitelistCommands::Get {
                user_id,
                subnet,
                subnet_size,
            } => {
                let entry = client
                    .get_user_ip_whitelist_entry(&user_id, &subnet, subnet_size)
                    .await?;
                print_output(&entry, output);
            }

            crate::commands::UserIpWhitelistCommands::Add {
                user_id,
                subnet,
                subnet_size,
            } => {
                client
                    .add_user_ip_whitelist(
                        &user_id,
                        AddIpWhitelistRequest {
                            subnet,
                            subnet_size,
                        },
                    )
                    .await?;
                print_success("IP added to whitelist");
            }

            crate::commands::UserIpWhitelistCommands::Delete {
                user_id,
                subnet,
                subnet_size,
            } => {
                client
                    .delete_user_ip_whitelist(
                        &user_id,
                        DeleteIpWhitelistRequest {
                            subnet,
                            subnet_size,
                        },
                    )
                    .await?;
                print_success("IP removed from whitelist");
            }
        },
    }
    Ok(())
}
