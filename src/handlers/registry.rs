//! Container Registry command handlers

use crate::api::VultrClient;
use crate::commands::{
    RegistryArgs, RegistryArtifactCommands, RegistryCommands, RegistryReplicationCommands,
    RegistryRepositoryCommands, RegistryRetentionCommands, RegistryRetentionRuleCommands,
    RegistryRetentionScheduleCommands, RegistryRobotCommands,
};
use crate::config::OutputFormat;
use crate::error::{VultrError, VultrResult};
use crate::handlers::confirm_delete;
use crate::models::{
    CreateRegistryRequest, CreateReplicationRequest, CreateRetentionRuleRequest,
    CreateRetentionScopeSelectors, CreateRetentionTagSelector, CreateRobotRequest,
    UpdateRegistryRequest, UpdateRetentionRuleRequest, UpdateRetentionScheduleRequest,
    UpdateRobotRequest, UpdateUserPasswordRequest,
};
use crate::output::{print_info, print_output, print_success};
use std::collections::HashMap;

pub async fn handle_registry(
    args: RegistryArgs,
    client: &VultrClient,
    output: OutputFormat,
    skip_confirm: bool,
) -> VultrResult<()> {
    match args.command {
        RegistryCommands::List => {
            let registries = client.list_registries().await?;
            print_output(&registries, output);
        }

        RegistryCommands::Get { id } => {
            let registry = client.get_registry(&id).await?;
            print_output(&registry, output);
        }

        RegistryCommands::Create {
            name,
            region,
            plan,
            public,
        } => {
            let request = CreateRegistryRequest {
                name: name.clone(),
                region,
                plan,
                public: if public { Some(true) } else { None },
            };
            let registry = client.create_registry(request).await?;
            print_success(&format!("Container registry {} created", name));
            print_output(&registry, output);
        }

        RegistryCommands::Update { id, plan, public } => {
            let request = UpdateRegistryRequest { public, plan };
            let registry = client.update_registry(&id, request).await?;
            print_success(&format!("Container registry {} updated", id));
            print_output(&registry, output);
        }

        RegistryCommands::Delete { id } => {
            if !skip_confirm && !confirm_delete("container registry", &id)? {
                return Err(VultrError::Cancelled);
            }
            client.delete_registry(&id).await?;
            print_success(&format!("Container registry {} deleted", id));
        }

        RegistryCommands::Repository(repo_args) => {
            handle_repository(repo_args.command, client, output, skip_confirm).await?;
        }

        RegistryCommands::Robot(robot_args) => {
            handle_robot(robot_args.command, client, output, skip_confirm).await?;
        }

        RegistryCommands::Replication(repl_args) => {
            handle_replication(repl_args.command, client, output, skip_confirm).await?;
        }

        RegistryCommands::Retention(retention_args) => {
            handle_retention(retention_args.command, client, output, skip_confirm).await?;
        }

        RegistryCommands::DockerCredentials {
            id,
            expiry_seconds,
            read_write,
        } => {
            let creds = client
                .get_registry_docker_credentials(
                    &id,
                    expiry_seconds,
                    if read_write { Some(true) } else { None },
                )
                .await?;
            print_output(&creds, output);
        }

        RegistryCommands::KubernetesCredentials {
            id,
            expiry_seconds,
            read_write,
            base64_encode,
        } => {
            let creds = client
                .get_registry_kubernetes_credentials(
                    &id,
                    expiry_seconds,
                    if read_write { Some(true) } else { None },
                    if base64_encode { Some(true) } else { None },
                )
                .await?;
            print_output(&creds, output);
        }

        RegistryCommands::UpdatePassword {
            registry_id,
            password,
        } => {
            let request = UpdateUserPasswordRequest { password };
            client
                .update_registry_user_password(&registry_id, request)
                .await?;
            print_success(&format!(
                "Registry {} root user password updated",
                registry_id
            ));
        }

        RegistryCommands::Regions => {
            let regions = client.list_registry_regions().await?;
            print_output(&regions, output);
        }

        RegistryCommands::Plans => {
            let plans = client.list_registry_plans().await?;
            print_output(&plans, output);
        }
    }

    Ok(())
}

async fn handle_repository(
    cmd: RegistryRepositoryCommands,
    client: &VultrClient,
    output: OutputFormat,
    skip_confirm: bool,
) -> VultrResult<()> {
    match cmd {
        RegistryRepositoryCommands::List { registry_id } => {
            let repos = client.list_registry_repositories(&registry_id).await?;
            print_output(&repos, output);
        }

        RegistryRepositoryCommands::Get { registry_id, image } => {
            let repo = client.get_registry_repository(&registry_id, &image).await?;
            print_output(&repo, output);
        }

        RegistryRepositoryCommands::Delete { registry_id, image } => {
            if !skip_confirm && !confirm_delete("repository", &image)? {
                return Err(VultrError::Cancelled);
            }
            client
                .delete_registry_repository(&registry_id, &image)
                .await?;
            print_success(&format!("Repository {} deleted", image));
        }

        RegistryRepositoryCommands::Artifact(artifact_args) => {
            handle_artifact(artifact_args.command, client, output, skip_confirm).await?;
        }
    }

    Ok(())
}

async fn handle_artifact(
    cmd: RegistryArtifactCommands,
    client: &VultrClient,
    output: OutputFormat,
    skip_confirm: bool,
) -> VultrResult<()> {
    match cmd {
        RegistryArtifactCommands::List { registry_id, image } => {
            let artifacts = client.list_registry_artifacts(&registry_id, &image).await?;
            print_output(&artifacts, output);
        }

        RegistryArtifactCommands::Delete {
            registry_id,
            image,
            digest,
        } => {
            if !skip_confirm && !confirm_delete("artifact", &digest)? {
                return Err(VultrError::Cancelled);
            }
            client
                .delete_registry_artifact(&registry_id, &image, &digest)
                .await?;
            print_success(&format!("Artifact {} deleted", digest));
        }
    }

    Ok(())
}

async fn handle_robot(
    cmd: RegistryRobotCommands,
    client: &VultrClient,
    output: OutputFormat,
    skip_confirm: bool,
) -> VultrResult<()> {
    match cmd {
        RegistryRobotCommands::List { registry_id } => {
            let robots = client.list_registry_robots(&registry_id).await?;
            print_output(&robots, output);
        }

        RegistryRobotCommands::Get { registry_id, name } => {
            let robot = client.get_registry_robot(&registry_id, &name).await?;
            print_output(&robot, output);
        }

        RegistryRobotCommands::Create {
            registry_id,
            name,
            description,
            duration,
            disable: _,
        } => {
            let request = CreateRobotRequest {
                name: name.clone(),
                description,
                duration: Some(duration),
                permissions: None,
            };
            let robot = client.create_registry_robot(&registry_id, request).await?;
            print_success(&format!("Robot account {} created", name));
            print_output(&robot, output);
        }

        RegistryRobotCommands::Update {
            registry_id,
            name,
            description,
            duration,
            disable,
        } => {
            let request = UpdateRobotRequest {
                description,
                duration,
                disable,
                permissions: None,
            };
            let robot = client
                .update_registry_robot(&registry_id, &name, request)
                .await?;
            print_success(&format!("Robot account {} updated", name));
            print_output(&robot, output);
        }

        RegistryRobotCommands::Delete { registry_id, name } => {
            if !skip_confirm && !confirm_delete("robot account", &name)? {
                return Err(VultrError::Cancelled);
            }
            client.delete_registry_robot(&registry_id, &name).await?;
            print_success(&format!("Robot account {} deleted", name));
        }
    }

    Ok(())
}

async fn handle_replication(
    cmd: RegistryReplicationCommands,
    client: &VultrClient,
    output: OutputFormat,
    skip_confirm: bool,
) -> VultrResult<()> {
    match cmd {
        RegistryReplicationCommands::List { registry_id } => {
            let replications = client.list_registry_replications(&registry_id).await?;
            print_output(&replications, output);
        }

        RegistryReplicationCommands::Create {
            registry_id,
            region,
        } => {
            let request = CreateReplicationRequest {
                region: region.clone(),
            };
            client
                .create_registry_replication(&registry_id, request)
                .await?;
            print_success(&format!(
                "Replication to region {} created for registry {}",
                region, registry_id
            ));
        }

        RegistryReplicationCommands::Delete {
            registry_id,
            region,
        } => {
            if !skip_confirm && !confirm_delete("replication", &region)? {
                return Err(VultrError::Cancelled);
            }
            client
                .delete_registry_replication(&registry_id, &region)
                .await?;
            print_success(&format!("Replication to region {} deleted", region));
        }
    }

    Ok(())
}

async fn handle_retention(
    cmd: RegistryRetentionCommands,
    client: &VultrClient,
    output: OutputFormat,
    skip_confirm: bool,
) -> VultrResult<()> {
    match cmd {
        RegistryRetentionCommands::Schedule(schedule_args) => {
            handle_retention_schedule(schedule_args.command, client, output).await?;
        }

        RegistryRetentionCommands::Rule(rule_args) => {
            handle_retention_rule(rule_args.command, client, output, skip_confirm).await?;
        }

        RegistryRetentionCommands::Executions { registry_id } => {
            let executions = client
                .list_registry_retention_executions(&registry_id)
                .await?;
            print_output(&executions, output);
        }
    }

    Ok(())
}

async fn handle_retention_schedule(
    cmd: RegistryRetentionScheduleCommands,
    client: &VultrClient,
    output: OutputFormat,
) -> VultrResult<()> {
    match cmd {
        RegistryRetentionScheduleCommands::Get { registry_id } => {
            let schedule = client.get_registry_retention_schedule(&registry_id).await?;
            if let Some(sched) = schedule {
                print_output(&sched, output);
            } else {
                print_info("No retention schedule configured");
            }
        }

        RegistryRetentionScheduleCommands::Update {
            registry_id,
            schedule_type,
            cron,
        } => {
            let request = UpdateRetentionScheduleRequest {
                schedule_type: schedule_type.clone(),
                cron,
            };
            client
                .update_registry_retention_schedule(&registry_id, request)
                .await?;
            print_success(&format!(
                "Retention schedule updated to {} for registry {}",
                schedule_type, registry_id
            ));
        }
    }

    Ok(())
}

async fn handle_retention_rule(
    cmd: RegistryRetentionRuleCommands,
    client: &VultrClient,
    output: OutputFormat,
    skip_confirm: bool,
) -> VultrResult<()> {
    match cmd {
        RegistryRetentionRuleCommands::List { registry_id } => {
            let rules = client.list_registry_retention_rules(&registry_id).await?;
            print_output(&rules, output);
        }

        RegistryRetentionRuleCommands::Create {
            registry_id,
            template,
            tag_pattern,
            repo_pattern,
            count,
        } => {
            let mut params = HashMap::new();
            if let Some(c) = count {
                // Different templates use different param names
                if template.contains("Days") {
                    params.insert("nDays".to_string(), c);
                } else {
                    params.insert("latestPushedK".to_string(), c);
                }
            }

            let scope_selectors = repo_pattern.map(|pattern| CreateRetentionScopeSelectors {
                repository: Some(vec![crate::models::CreateRetentionRepositorySelector {
                    decoration: "repoMatches".to_string(),
                    kind: "doublestar".to_string(),
                    pattern,
                }]),
            });

            let tag_selectors = vec![CreateRetentionTagSelector {
                decoration: "matches".to_string(),
                kind: "doublestar".to_string(),
                pattern: tag_pattern,
            }];

            let request = CreateRetentionRuleRequest {
                action: Some("retain".to_string()),
                template: template.clone(),
                params: if params.is_empty() {
                    None
                } else {
                    Some(params)
                },
                scope_selectors,
                tag_selectors: Some(tag_selectors),
            };

            let rule = client
                .create_registry_retention_rule(&registry_id, request)
                .await?;
            print_success(&format!(
                "Retention rule created for registry {}",
                registry_id
            ));
            print_output(&rule, output);
        }

        RegistryRetentionRuleCommands::Update {
            registry_id,
            rule_id,
            template,
            disabled,
            count,
        } => {
            let params = count.map(|c| {
                let mut p = HashMap::new();
                p.insert("latestPushedK".to_string(), c);
                p
            });

            let request = UpdateRetentionRuleRequest {
                action: None,
                template,
                params,
                disabled,
                scope_selectors: None,
                tag_selectors: None,
            };

            let rule = client
                .update_registry_retention_rule(&registry_id, rule_id, request)
                .await?;
            print_success(&format!("Retention rule {} updated", rule_id));
            print_output(&rule, output);
        }

        RegistryRetentionRuleCommands::Delete {
            registry_id,
            rule_id,
        } => {
            if !skip_confirm && !confirm_delete("retention rule", &rule_id.to_string())? {
                return Err(VultrError::Cancelled);
            }
            client
                .delete_registry_retention_rule(&registry_id, rule_id)
                .await?;
            print_success(&format!("Retention rule {} deleted", rule_id));
        }
    }

    Ok(())
}
