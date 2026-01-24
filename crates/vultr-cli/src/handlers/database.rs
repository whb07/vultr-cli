//! Database command handlers

use vultr_api::VultrClient;
use crate::commands::*;
use vultr_config::OutputFormat;
use vultr_config::VultrResult;
use vultr_models::*;
use vultr_output::{print_json, print_output, print_success};

use super::confirm_delete;

pub async fn handle_database(
    args: DatabaseArgs,
    client: &VultrClient,
    output: OutputFormat,
    skip_confirm: bool,
) -> VultrResult<()> {
    match args.command {
        DatabaseCommands::List => {
            let databases = client.list_databases().await?;
            print_output(&databases, output);
        }
        DatabaseCommands::Get { id } => {
            let database = client.get_database(&id).await?;
            print_output(&database, output);
        }
        DatabaseCommands::Create(create_args) => {
            let request = CreateDatabaseRequest {
                database_engine: create_args.engine,
                database_engine_version: create_args.version,
                region: create_args.region,
                plan: create_args.plan,
                label: create_args.label,
                tag: create_args.tag,
                vpc_id: create_args.vpc_id,
                maintenance_dow: create_args.maintenance_dow,
                maintenance_time: create_args.maintenance_time,
                backup_hour: create_args.backup_hour,
                backup_minute: create_args.backup_minute,
                trusted_ips: create_args.trusted_ips,
                mysql_sql_modes: create_args.mysql_sql_modes,
                mysql_require_primary_key: create_args.mysql_require_primary_key,
                eviction_policy: create_args.eviction_policy,
                ..Default::default()
            };
            let database = client.create_database(request).await?;
            print_output(&database, output);
        }
        DatabaseCommands::Update(update_args) => {
            let request = UpdateDatabaseRequest {
                plan: update_args.plan,
                label: update_args.label,
                tag: update_args.tag,
                vpc_id: update_args.vpc_id,
                maintenance_dow: update_args.maintenance_dow,
                maintenance_time: update_args.maintenance_time,
                backup_hour: update_args.backup_hour,
                backup_minute: update_args.backup_minute,
                trusted_ips: update_args.trusted_ips,
                cluster_time_zone: update_args.cluster_time_zone,
                eviction_policy: update_args.eviction_policy,
                ..Default::default()
            };
            let database = client.update_database(&update_args.id, request).await?;
            print_output(&database, output);
        }
        DatabaseCommands::Delete { id } => {
            if skip_confirm || confirm_delete("database", &id)? {
                client.delete_database(&id).await?;
                print_success(&format!("Database {} deleted", id));
            }
        }
        DatabaseCommands::Plans {
            engine,
            nodes,
            region,
        } => {
            let plans = client
                .list_database_plans(engine.as_deref(), nodes, region.as_deref())
                .await?;
            print_output(&plans, output);
        }
        DatabaseCommands::Usage { id } => {
            let usage = client.get_database_usage(&id).await?;
            print_output(&usage, output);
        }
        DatabaseCommands::Alerts { id } => {
            let alerts = client.get_database_alerts(&id).await?;
            print_output(&alerts, output);
        }
        DatabaseCommands::Backups { id } => {
            let backups = client.get_database_backups(&id).await?;
            print_output(&backups, output);
        }
        DatabaseCommands::Restore(restore_args) => {
            let request = RestoreDatabaseRequest {
                label: restore_args.label,
                date: restore_args.date,
                time: restore_args.time,
                restore_type: restore_args.restore_type,
            };
            let database = client
                .restore_database(&restore_args.database_id, request)
                .await?;
            print_output(&database, output);
        }
        DatabaseCommands::Fork(fork_args) => {
            let request = ForkDatabaseRequest {
                label: fork_args.label,
                region: fork_args.region,
                plan: fork_args.plan,
                vpc_id: fork_args.vpc_id,
                date: fork_args.date,
                time: fork_args.time,
                fork_type: fork_args.fork_type,
            };
            let database = client
                .fork_database(&fork_args.database_id, request)
                .await?;
            print_output(&database, output);
        }
        DatabaseCommands::ReadReplica {
            database_id,
            label,
            region,
        } => {
            let request = CreateReadReplicaRequest { label, region };
            let database = client.create_read_replica(&database_id, request).await?;
            print_output(&database, output);
        }
        DatabaseCommands::Promote { id } => {
            client.promote_read_replica(&id).await?;
            print_success(&format!("Read replica {} promoted to standalone", id));
        }
        DatabaseCommands::Maintenance { id } => {
            let maintenance = client.get_database_maintenance(&id).await?;
            print_output(&maintenance, output);
        }
        DatabaseCommands::SetMaintenance {
            database_id,
            day,
            hour,
        } => {
            let request = UpdateMaintenanceRequest { day, hour };
            let maintenance = client
                .update_database_maintenance(&database_id, request)
                .await?;
            print_output(&maintenance, output);
        }
        DatabaseCommands::Upgrades { id } => {
            let versions = client.get_database_version_upgrades(&id).await?;
            print_output(&versions, output);
        }
        DatabaseCommands::Upgrade {
            database_id,
            version,
        } => {
            let request = UpgradeDatabaseVersionRequest { version };
            let database = client
                .upgrade_database_version(&database_id, request)
                .await?;
            print_output(&database, output);
        }
        DatabaseCommands::User(user_args) => {
            handle_database_user(user_args, client, output, skip_confirm).await?;
        }
        DatabaseCommands::Db(db_args) => {
            handle_database_db(db_args, client, output, skip_confirm).await?;
        }
        DatabaseCommands::Pool(pool_args) => {
            handle_database_pool(pool_args, client, output, skip_confirm).await?;
        }
        DatabaseCommands::Topic(topic_args) => {
            handle_database_topic(topic_args, client, output, skip_confirm).await?;
        }
        DatabaseCommands::Connector(connector_args) => {
            handle_database_connector(connector_args, client, output, skip_confirm).await?;
        }
        DatabaseCommands::Migration(migration_args) => {
            handle_database_migration(migration_args, client, output).await?;
        }
        DatabaseCommands::AdvancedOptions { id } => {
            let options = client.get_database_advanced_options(&id).await?;
            print_json(&options);
        }
        DatabaseCommands::SetAdvancedOptions {
            database_id,
            options,
        } => {
            let parsed: serde_json::Value = serde_json::from_str(&options).map_err(|e| {
                vultr_config::VultrError::InvalidInput(format!("Invalid JSON: {}", e))
            })?;
            let result = client
                .update_database_advanced_options(&database_id, parsed)
                .await?;
            print_json(&result);
        }
        DatabaseCommands::AdvancedOptionsKafkaRest { id } => {
            let options = client.get_kafka_rest_advanced_options(&id).await?;
            print_json(&options);
        }
        DatabaseCommands::SetAdvancedOptionsKafkaRest {
            database_id,
            options,
        } => {
            let parsed: serde_json::Value = serde_json::from_str(&options).map_err(|e| {
                vultr_config::VultrError::InvalidInput(format!("Invalid JSON: {}", e))
            })?;
            let result = client
                .update_kafka_rest_advanced_options(&database_id, parsed)
                .await?;
            print_json(&result);
        }
        DatabaseCommands::AdvancedOptionsSchemaRegistry { id } => {
            let options = client.get_schema_registry_advanced_options(&id).await?;
            print_json(&options);
        }
        DatabaseCommands::SetAdvancedOptionsSchemaRegistry {
            database_id,
            options,
        } => {
            let parsed: serde_json::Value = serde_json::from_str(&options).map_err(|e| {
                vultr_config::VultrError::InvalidInput(format!("Invalid JSON: {}", e))
            })?;
            let result = client
                .update_schema_registry_advanced_options(&database_id, parsed)
                .await?;
            print_json(&result);
        }
        DatabaseCommands::AdvancedOptionsKafkaConnect { id } => {
            let options = client.get_kafka_connect_advanced_options(&id).await?;
            print_json(&options);
        }
        DatabaseCommands::SetAdvancedOptionsKafkaConnect {
            database_id,
            options,
        } => {
            let parsed: serde_json::Value = serde_json::from_str(&options).map_err(|e| {
                vultr_config::VultrError::InvalidInput(format!("Invalid JSON: {}", e))
            })?;
            let result = client
                .update_kafka_connect_advanced_options(&database_id, parsed)
                .await?;
            print_json(&result);
        }
        DatabaseCommands::Quota(quota_args) => {
            handle_database_quota(quota_args, client, output, skip_confirm).await?;
        }
    }
    Ok(())
}

async fn handle_database_user(
    args: DatabaseUserArgs,
    client: &VultrClient,
    output: OutputFormat,
    skip_confirm: bool,
) -> VultrResult<()> {
    match args.command {
        DatabaseUserCommands::List { database_id } => {
            let users = client.list_database_users(&database_id).await?;
            print_output(&users, output);
        }
        DatabaseUserCommands::Get {
            database_id,
            username,
        } => {
            let user = client.get_database_user(&database_id, &username).await?;
            print_output(&user, output);
        }
        DatabaseUserCommands::Create {
            database_id,
            username,
            password,
            encryption,
            permission,
        } => {
            let request = CreateDatabaseUserRequest {
                username,
                password,
                encryption,
                permission,
            };
            let user = client.create_database_user(&database_id, request).await?;
            print_output(&user, output);
        }
        DatabaseUserCommands::Update {
            database_id,
            username,
            password,
        } => {
            let request = UpdateDatabaseUserRequest {
                password: Some(password),
            };
            let user = client
                .update_database_user(&database_id, &username, request)
                .await?;
            print_output(&user, output);
        }
        DatabaseUserCommands::Delete {
            database_id,
            username,
        } => {
            if skip_confirm || confirm_delete("database user", &username)? {
                client.delete_database_user(&database_id, &username).await?;
                print_success(&format!("User {} deleted", username));
            }
        }
        DatabaseUserCommands::AccessControl {
            database_id,
            username,
            acl_categories,
            acl_channels,
            acl_commands,
            acl_keys,
        } => {
            let request = UpdateUserAccessControlRequest {
                acl_categories: acl_categories
                    .map(|s| s.split(',').map(|x| x.trim().to_string()).collect()),
                acl_channels: acl_channels
                    .map(|s| s.split(',').map(|x| x.trim().to_string()).collect()),
                acl_commands: acl_commands
                    .map(|s| s.split(',').map(|x| x.trim().to_string()).collect()),
                acl_keys: acl_keys.map(|s| s.split(',').map(|x| x.trim().to_string()).collect()),
            };
            let user = client
                .update_user_access_control(&database_id, &username, request)
                .await?;
            print_output(&user, output);
        }
        DatabaseUserCommands::Permissions {
            database_id,
            username,
            permission,
        } => {
            let request = KafkaPermissions {
                permission: Some(permission),
            };
            let user = client
                .update_kafka_permissions(&database_id, &username, request)
                .await?;
            print_output(&user, output);
        }
    }
    Ok(())
}

async fn handle_database_db(
    args: DatabaseDbArgs,
    client: &VultrClient,
    output: OutputFormat,
    skip_confirm: bool,
) -> VultrResult<()> {
    match args.command {
        DatabaseDbCommands::List { database_id } => {
            let dbs = client.list_logical_databases(&database_id).await?;
            print_output(&dbs, output);
        }
        DatabaseDbCommands::Get { database_id, name } => {
            let db = client.get_logical_database(&database_id, &name).await?;
            print_output(&db, output);
        }
        DatabaseDbCommands::Create { database_id, name } => {
            let request = CreateLogicalDatabaseRequest { name: name.clone() };
            let db = client
                .create_logical_database(&database_id, request)
                .await?;
            print_output(&db, output);
        }
        DatabaseDbCommands::Delete { database_id, name } => {
            if skip_confirm || confirm_delete("logical database", &name)? {
                client.delete_logical_database(&database_id, &name).await?;
                print_success(&format!("Logical database {} deleted", name));
            }
        }
    }
    Ok(())
}

async fn handle_database_pool(
    args: DatabasePoolArgs,
    client: &VultrClient,
    output: OutputFormat,
    skip_confirm: bool,
) -> VultrResult<()> {
    match args.command {
        DatabasePoolCommands::List { database_id } => {
            let pools = client.list_connection_pools(&database_id).await?;
            print_output(&pools, output);
        }
        DatabasePoolCommands::Get { database_id, name } => {
            let pool = client.get_connection_pool(&database_id, &name).await?;
            print_output(&pool, output);
        }
        DatabasePoolCommands::Create {
            database_id,
            name,
            database,
            username,
            mode,
            size,
        } => {
            let request = CreateConnectionPoolRequest {
                name: name.clone(),
                database,
                username,
                mode,
                size,
            };
            let pool = client.create_connection_pool(&database_id, request).await?;
            print_output(&pool, output);
        }
        DatabasePoolCommands::Update {
            database_id,
            name,
            database,
            username,
            mode,
            size,
        } => {
            let request = UpdateConnectionPoolRequest {
                database,
                username,
                mode,
                size,
            };
            let pool = client
                .update_connection_pool(&database_id, &name, request)
                .await?;
            print_output(&pool, output);
        }
        DatabasePoolCommands::Delete { database_id, name } => {
            if skip_confirm || confirm_delete("connection pool", &name)? {
                client.delete_connection_pool(&database_id, &name).await?;
                print_success(&format!("Connection pool {} deleted", name));
            }
        }
    }
    Ok(())
}

async fn handle_database_topic(
    args: DatabaseTopicArgs,
    client: &VultrClient,
    output: OutputFormat,
    skip_confirm: bool,
) -> VultrResult<()> {
    match args.command {
        DatabaseTopicCommands::List { database_id } => {
            let topics = client.list_kafka_topics(&database_id).await?;
            print_output(&topics, output);
        }
        DatabaseTopicCommands::Get { database_id, name } => {
            let topic = client.get_kafka_topic(&database_id, &name).await?;
            print_output(&topic, output);
        }
        DatabaseTopicCommands::Create {
            database_id,
            name,
            partitions,
            replication,
            retention_hours,
            retention_bytes,
        } => {
            let request = CreateKafkaTopicRequest {
                name: name.clone(),
                partitions,
                replication,
                retention_hours,
                retention_bytes,
            };
            let topic = client.create_kafka_topic(&database_id, request).await?;
            print_output(&topic, output);
        }
        DatabaseTopicCommands::Update {
            database_id,
            name,
            partitions,
            retention_hours,
            retention_bytes,
        } => {
            let request = UpdateKafkaTopicRequest {
                partitions,
                retention_hours,
                retention_bytes,
            };
            let topic = client
                .update_kafka_topic(&database_id, &name, request)
                .await?;
            print_output(&topic, output);
        }
        DatabaseTopicCommands::Delete { database_id, name } => {
            if skip_confirm || confirm_delete("Kafka topic", &name)? {
                client.delete_kafka_topic(&database_id, &name).await?;
                print_success(&format!("Kafka topic {} deleted", name));
            }
        }
    }
    Ok(())
}

async fn handle_database_connector(
    args: DatabaseConnectorArgs,
    client: &VultrClient,
    output: OutputFormat,
    skip_confirm: bool,
) -> VultrResult<()> {
    match args.command {
        DatabaseConnectorCommands::Available { database_id } => {
            let connectors = client.list_available_connectors(&database_id).await?;
            print_output(&connectors, output);
        }
        DatabaseConnectorCommands::ConfigSchema {
            database_id,
            connector_class,
        } => {
            let schema = client
                .get_connector_configuration_schema(&database_id, &connector_class)
                .await?;
            print_output(&schema, output);
        }
        DatabaseConnectorCommands::List { database_id } => {
            let connectors = client.list_kafka_connectors(&database_id).await?;
            print_output(&connectors, output);
        }
        DatabaseConnectorCommands::Get { database_id, name } => {
            let connector = client.get_kafka_connector(&database_id, &name).await?;
            print_output(&connector, output);
        }
        DatabaseConnectorCommands::Create {
            database_id,
            name,
            class,
            topics,
        } => {
            let request = CreateKafkaConnectorRequest {
                name: name.clone(),
                class,
                topics,
                config: None,
            };
            let connector = client.create_kafka_connector(&database_id, request).await?;
            print_output(&connector, output);
        }
        DatabaseConnectorCommands::Delete { database_id, name } => {
            if skip_confirm || confirm_delete("Kafka connector", &name)? {
                client.delete_kafka_connector(&database_id, &name).await?;
                print_success(&format!("Kafka connector {} deleted", name));
            }
        }
        DatabaseConnectorCommands::Status { database_id, name } => {
            let status = client.get_connector_status(&database_id, &name).await?;
            print_output(&status, output);
        }
        DatabaseConnectorCommands::Pause { database_id, name } => {
            client.pause_kafka_connector(&database_id, &name).await?;
            print_success(&format!("Connector {} paused", name));
        }
        DatabaseConnectorCommands::Resume { database_id, name } => {
            client.resume_kafka_connector(&database_id, &name).await?;
            print_success(&format!("Connector {} resumed", name));
        }
        DatabaseConnectorCommands::Restart { database_id, name } => {
            client.restart_kafka_connector(&database_id, &name).await?;
            print_success(&format!("Connector {} restarted", name));
        }
        DatabaseConnectorCommands::RestartTask {
            database_id,
            connector_name,
            task_id,
        } => {
            client
                .restart_connector_task(&database_id, &connector_name, &task_id)
                .await?;
            print_success(&format!("Connector task {} restarted", task_id));
        }
    }
    Ok(())
}

async fn handle_database_migration(
    args: DatabaseMigrationArgs,
    client: &VultrClient,
    output: OutputFormat,
) -> VultrResult<()> {
    match args.command {
        DatabaseMigrationCommands::Status { database_id } => {
            let migration = client.get_database_migration(&database_id).await?;
            print_output(&migration, output);
        }
        DatabaseMigrationCommands::Start {
            database_id,
            host,
            port,
            username,
            password,
            database,
            ignored_databases,
            ssl,
        } => {
            let request = StartMigrationRequest {
                host,
                port,
                username,
                password,
                database,
                ignored_databases,
                ssl: Some(ssl),
            };
            let migration = client
                .start_database_migration(&database_id, request)
                .await?;
            print_output(&migration, output);
        }
        DatabaseMigrationCommands::Detach { database_id } => {
            client.detach_database_migration(&database_id).await?;
            print_success("Migration detached");
        }
    }
    Ok(())
}

async fn handle_database_quota(
    args: DatabaseQuotaArgs,
    client: &VultrClient,
    output: OutputFormat,
    skip_confirm: bool,
) -> VultrResult<()> {
    match args.command {
        DatabaseQuotaCommands::List { database_id } => {
            let quotas = client.list_database_quotas(&database_id).await?;
            print_output(&quotas, output);
        }
        DatabaseQuotaCommands::Create {
            database_id,
            client_id,
            username,
            consumer_byte_rate,
            producer_byte_rate,
            request_percentage,
        } => {
            let request = CreateDatabaseQuotaRequest {
                client_id,
                consumer_byte_rate,
                producer_byte_rate,
                request_percentage,
            };
            client
                .create_database_quota(&database_id, &username, request)
                .await?;
            print_success(&format!("Quota created for user {}", username));
        }
        DatabaseQuotaCommands::Delete {
            database_id,
            client_id,
            username,
        } => {
            if skip_confirm
                || confirm_delete("database quota", &format!("{}/{}", client_id, username))?
            {
                client
                    .delete_database_quota(&database_id, &client_id, &username)
                    .await?;
                print_success(&format!("Quota deleted for {}/{}", client_id, username));
            }
        }
    }
    Ok(())
}
