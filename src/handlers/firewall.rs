//! Firewall command handlers

use dialoguer::Confirm;

use crate::api::{self, VultrClient, WaitOptions};
use crate::commands::{
    FirewallArgs, FirewallCommands, FirewallGroupArgs, FirewallGroupCommands, FirewallRuleArgs,
    FirewallRuleCommands,
};
use crate::config::OutputFormat;
use crate::error::{VultrError, VultrResult};
use crate::handlers::confirm_delete;
use crate::models::{
    CreateFirewallGroupRequest, CreateFirewallRuleRequest, UpdateFirewallGroupRequest,
};
use crate::output::{print_output, print_success};

pub async fn handle_firewall(
    args: FirewallArgs,
    client: &VultrClient,
    output: OutputFormat,
    skip_confirm: bool,
    wait: bool,
    wait_opts: &WaitOptions,
) -> VultrResult<()> {
    match args.command {
        FirewallCommands::Group(g) => {
            handle_firewall_group(g, client, output, skip_confirm, wait, wait_opts).await
        }
        FirewallCommands::Rule(r) => {
            handle_firewall_rule(r, client, output, skip_confirm, wait, wait_opts).await
        }
    }
}

async fn handle_firewall_group(
    args: FirewallGroupArgs,
    client: &VultrClient,
    output: OutputFormat,
    skip_confirm: bool,
    wait: bool,
    wait_opts: &WaitOptions,
) -> VultrResult<()> {
    match args.command {
        FirewallGroupCommands::List(list_args) => {
            if list_args.all {
                let mut all = Vec::new();
                let mut cursor: Option<String> = None;
                loop {
                    let (page, meta) = client
                        .list_firewall_groups(Some(list_args.per_page), cursor.as_deref())
                        .await?;
                    all.extend(page);
                    cursor = meta.links.and_then(|l| l.next);
                    if cursor.is_none() {
                        break;
                    }
                }
                print_output(&all, output);
            } else {
                let (groups, _) = client
                    .list_firewall_groups(Some(list_args.per_page), list_args.cursor.as_deref())
                    .await?;
                print_output(&groups, output);
            }
        }

        FirewallGroupCommands::Get { id } => {
            let group = client.get_firewall_group(&id).await?;
            print_output(&group, output);
        }

        FirewallGroupCommands::Create { description } => {
            let group = client
                .create_firewall_group(CreateFirewallGroupRequest { description })
                .await?;
            print_success(&format!("Firewall group {} created", group.id));
            print_output(&group, output);
        }

        FirewallGroupCommands::Update { id, description } => {
            client
                .update_firewall_group(
                    &id,
                    UpdateFirewallGroupRequest {
                        description: Some(description),
                    },
                )
                .await?;
            print_success(&format!("Firewall group {} updated", id));
        }

        FirewallGroupCommands::Delete { id } => {
            if !skip_confirm && !confirm_delete("firewall group", &id)? {
                return Err(VultrError::Cancelled);
            }
            client.delete_firewall_group(&id).await?;
            print_success(&format!("Firewall group {} deletion initiated", id));
            if wait {
                api::verify_firewall_group_deleted(client, &id, wait_opts).await?;
            }
        }
    }
    Ok(())
}

async fn handle_firewall_rule(
    args: FirewallRuleArgs,
    client: &VultrClient,
    output: OutputFormat,
    skip_confirm: bool,
    wait: bool,
    wait_opts: &WaitOptions,
) -> VultrResult<()> {
    match args.command {
        FirewallRuleCommands::List { group_id, list } => {
            if list.all {
                let mut all = Vec::new();
                let mut cursor: Option<String> = None;
                loop {
                    let (page, meta) = client
                        .list_firewall_rules(&group_id, Some(list.per_page), cursor.as_deref())
                        .await?;
                    all.extend(page);
                    cursor = meta.links.and_then(|l| l.next);
                    if cursor.is_none() {
                        break;
                    }
                }
                print_output(&all, output);
            } else {
                let (rules, _) = client
                    .list_firewall_rules(&group_id, Some(list.per_page), list.cursor.as_deref())
                    .await?;
                print_output(&rules, output);
            }
        }

        FirewallRuleCommands::Get { group_id, rule_id } => {
            let rule = client.get_firewall_rule(&group_id, rule_id).await?;
            print_output(&rule, output);
        }

        FirewallRuleCommands::Create {
            group_id,
            ip_type,
            protocol,
            subnet,
            subnet_size,
            port,
            source,
            notes,
        } => {
            let rule = client
                .create_firewall_rule(
                    &group_id,
                    CreateFirewallRuleRequest {
                        ip_type,
                        protocol,
                        subnet,
                        subnet_size,
                        port,
                        source,
                        notes,
                    },
                )
                .await?;
            print_success(&format!(
                "Firewall rule {} created in group {}",
                rule.id, group_id
            ));
            print_output(&rule, output);
        }

        FirewallRuleCommands::Delete { group_id, rule_id } => {
            if !skip_confirm
                && !Confirm::new()
                    .with_prompt(format!("Delete rule {} from group {}?", rule_id, group_id))
                    .default(false)
                    .interact()
                    .unwrap_or(false)
            {
                return Err(VultrError::Cancelled);
            }
            client.delete_firewall_rule(&group_id, rule_id).await?;
            print_success(&format!(
                "Firewall rule {} deletion initiated from group {}",
                rule_id, group_id
            ));
            if wait {
                api::verify_firewall_rule_deleted(client, &group_id, rule_id, wait_opts).await?;
            }
        }
    }
    Ok(())
}
