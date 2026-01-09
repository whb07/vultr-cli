//! CDN command handlers

use crate::api::VultrClient;
use crate::commands::{
    CdnArgs, CdnCommands, CdnPullZoneArgs, CdnPullZoneCommands, CdnPushZoneArgs,
    CdnPushZoneCommands, CdnPushZoneFileArgs, CdnPushZoneFileCommands,
};
use crate::config::OutputFormat;
use crate::error::{VultrError, VultrResult};
use crate::handlers::confirm_delete;
use crate::models::{
    CreateFileEndpointRequest, CreatePullZoneRequest, CreatePushZoneRequest, UpdatePullZoneRequest,
    UpdatePushZoneRequest,
};
use crate::output::{print_output, print_success};

pub async fn handle_cdn(
    args: CdnArgs,
    client: &VultrClient,
    output: OutputFormat,
    skip_confirm: bool,
) -> VultrResult<()> {
    match args.command {
        CdnCommands::PullZone(pull_args) => {
            handle_pull_zone(pull_args, client, output, skip_confirm).await
        }
        CdnCommands::PushZone(push_args) => {
            handle_push_zone(push_args, client, output, skip_confirm).await
        }
    }
}

async fn handle_pull_zone(
    args: CdnPullZoneArgs,
    client: &VultrClient,
    output: OutputFormat,
    skip_confirm: bool,
) -> VultrResult<()> {
    match args.command {
        CdnPullZoneCommands::List(list_args) => {
            if list_args.all {
                let mut all = Vec::new();
                let mut cursor: Option<String> = None;
                loop {
                    let (page, meta) = client
                        .list_cdn_pull_zones(Some(list_args.per_page), cursor.as_deref())
                        .await?;
                    all.extend(page);
                    cursor = meta.links.and_then(|l| l.next);
                    if cursor.is_none() {
                        break;
                    }
                }
                print_output(&all, output);
            } else {
                let (zones, _) = client
                    .list_cdn_pull_zones(Some(list_args.per_page), list_args.cursor.as_deref())
                    .await?;
                print_output(&zones, output);
            }
        }

        CdnPullZoneCommands::Get { id } => {
            let zone = client.get_cdn_pull_zone(&id).await?;
            print_output(&zone, output);
        }

        CdnPullZoneCommands::Create {
            label,
            origin_domain,
            origin_scheme,
            vanity_domain,
            cors,
            gzip,
            block_ai,
            block_bad_bots,
            regions,
        } => {
            let zone = client
                .create_cdn_pull_zone(CreatePullZoneRequest {
                    label,
                    origin_domain,
                    origin_scheme: Some(origin_scheme),
                    vanity_domain,
                    ssl_cert: None,
                    ssl_cert_key: None,
                    cors: cors.then_some(true),
                    gzip: gzip.then_some(true),
                    block_ai: block_ai.then_some(true),
                    block_bad_bots: block_bad_bots.then_some(true),
                    regions,
                })
                .await?;
            print_success(&format!("CDN Pull Zone {} created", zone.id));
            print_output(&zone, output);
        }

        CdnPullZoneCommands::Update {
            id,
            label,
            origin_domain,
            origin_scheme,
            vanity_domain,
            cors,
            gzip,
            block_ai,
            block_bad_bots,
            regions,
        } => {
            let zone = client
                .update_cdn_pull_zone(
                    &id,
                    UpdatePullZoneRequest {
                        label,
                        origin_domain,
                        origin_scheme,
                        vanity_domain,
                        ssl_cert: None,
                        ssl_cert_key: None,
                        cors,
                        gzip,
                        block_ai,
                        block_bad_bots,
                        regions,
                    },
                )
                .await?;
            print_success(&format!("CDN Pull Zone {} updated", id));
            print_output(&zone, output);
        }

        CdnPullZoneCommands::Delete { id } => {
            if !skip_confirm && !confirm_delete("CDN Pull Zone", &id)? {
                return Err(VultrError::Cancelled);
            }
            client.delete_cdn_pull_zone(&id).await?;
            print_success(&format!("CDN Pull Zone {} deleted", id));
        }

        CdnPullZoneCommands::Purge { id } => {
            client.purge_cdn_pull_zone(&id).await?;
            print_success(&format!("CDN Pull Zone {} cache purged", id));
        }
    }
    Ok(())
}

async fn handle_push_zone(
    args: CdnPushZoneArgs,
    client: &VultrClient,
    output: OutputFormat,
    skip_confirm: bool,
) -> VultrResult<()> {
    match args.command {
        CdnPushZoneCommands::List(list_args) => {
            if list_args.all {
                let mut all = Vec::new();
                let mut cursor: Option<String> = None;
                loop {
                    let (page, meta) = client
                        .list_cdn_push_zones(Some(list_args.per_page), cursor.as_deref())
                        .await?;
                    all.extend(page);
                    cursor = meta.links.and_then(|l| l.next);
                    if cursor.is_none() {
                        break;
                    }
                }
                print_output(&all, output);
            } else {
                let (zones, _) = client
                    .list_cdn_push_zones(Some(list_args.per_page), list_args.cursor.as_deref())
                    .await?;
                print_output(&zones, output);
            }
        }

        CdnPushZoneCommands::Get { id } => {
            let zone = client.get_cdn_push_zone(&id).await?;
            print_output(&zone, output);
        }

        CdnPushZoneCommands::Create {
            label,
            vanity_domain,
            cors,
            gzip,
            block_ai,
            block_bad_bots,
            regions,
        } => {
            let zone = client
                .create_cdn_push_zone(CreatePushZoneRequest {
                    label,
                    vanity_domain,
                    ssl_cert: None,
                    ssl_cert_key: None,
                    cors: cors.then_some(true),
                    gzip: gzip.then_some(true),
                    block_ai: block_ai.then_some(true),
                    block_bad_bots: block_bad_bots.then_some(true),
                    regions,
                })
                .await?;
            print_success(&format!("CDN Push Zone {} created", zone.id));
            print_output(&zone, output);
        }

        CdnPushZoneCommands::Update {
            id,
            label,
            vanity_domain,
            cors,
            gzip,
            block_ai,
            block_bad_bots,
            regions,
        } => {
            let zone = client
                .update_cdn_push_zone(
                    &id,
                    UpdatePushZoneRequest {
                        label,
                        vanity_domain,
                        ssl_cert: None,
                        ssl_cert_key: None,
                        cors,
                        gzip,
                        block_ai,
                        block_bad_bots,
                        regions,
                    },
                )
                .await?;
            print_success(&format!("CDN Push Zone {} updated", id));
            print_output(&zone, output);
        }

        CdnPushZoneCommands::Delete { id } => {
            if !skip_confirm && !confirm_delete("CDN Push Zone", &id)? {
                return Err(VultrError::Cancelled);
            }
            client.delete_cdn_push_zone(&id).await?;
            print_success(&format!("CDN Push Zone {} deleted", id));
        }

        CdnPushZoneCommands::File(file_args) => {
            handle_push_zone_file(file_args, client, output, skip_confirm).await?;
        }
    }
    Ok(())
}

async fn handle_push_zone_file(
    args: CdnPushZoneFileArgs,
    client: &VultrClient,
    output: OutputFormat,
    skip_confirm: bool,
) -> VultrResult<()> {
    match args.command {
        CdnPushZoneFileCommands::List { pushzone_id, list } => {
            if list.all {
                let mut all = Vec::new();
                let mut cursor: Option<String> = None;
                loop {
                    let (page, meta) = client
                        .list_cdn_push_zone_files(
                            &pushzone_id,
                            Some(list.per_page),
                            cursor.as_deref(),
                        )
                        .await?;
                    all.extend(page);
                    cursor = meta.links.and_then(|l| l.next);
                    if cursor.is_none() {
                        break;
                    }
                }
                print_output(&all, output);
            } else {
                let (files, _) = client
                    .list_cdn_push_zone_files(
                        &pushzone_id,
                        Some(list.per_page),
                        list.cursor.as_deref(),
                    )
                    .await?;
                print_output(&files, output);
            }
        }

        CdnPushZoneFileCommands::Get {
            pushzone_id,
            file_name,
        } => {
            let file = client
                .get_cdn_push_zone_file(&pushzone_id, &file_name)
                .await?;
            print_output(&file, output);
        }

        CdnPushZoneFileCommands::CreateEndpoint {
            pushzone_id,
            name,
            size,
        } => {
            let endpoint = client
                .create_cdn_push_zone_file_endpoint(
                    &pushzone_id,
                    CreateFileEndpointRequest {
                        name: name.clone(),
                        size,
                    },
                )
                .await?;
            print_success(&format!("Upload endpoint created for file: {}", name));
            print_output(&endpoint, output);
        }

        CdnPushZoneFileCommands::Delete {
            pushzone_id,
            file_name,
        } => {
            if !skip_confirm && !confirm_delete("CDN Push Zone file", &file_name)? {
                return Err(VultrError::Cancelled);
            }
            client
                .delete_cdn_push_zone_file(&pushzone_id, &file_name)
                .await?;
            print_success(&format!(
                "File {} deleted from Push Zone {}",
                file_name, pushzone_id
            ));
        }
    }
    Ok(())
}
