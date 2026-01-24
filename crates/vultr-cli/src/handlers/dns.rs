//! DNS command handlers

use vultr_api::VultrClient;
use crate::commands::{DnsArgs, DnsCommands, DnsRecordCommands};
use vultr_config::OutputFormat;
use vultr_config::{VultrError, VultrResult};
use crate::handlers::confirm_delete;
use vultr_models::{
    CreateDomainRequest, CreateRecordRequest, UpdateDomainRequest, UpdateRecordRequest,
    UpdateSoaRequest,
};
use vultr_output::{print_output, print_success};

pub async fn handle_dns(
    args: DnsArgs,
    client: &VultrClient,
    output: OutputFormat,
    skip_confirm: bool,
) -> VultrResult<()> {
    match args.command {
        DnsCommands::List(list_args) => {
            if list_args.all {
                let mut all = Vec::new();
                let mut cursor: Option<String> = None;
                loop {
                    let (page, meta) = client
                        .list_dns_domains(Some(list_args.per_page), cursor.as_deref())
                        .await?;
                    all.extend(page);
                    cursor = meta.links.and_then(|l| l.next);
                    if cursor.is_none() {
                        break;
                    }
                }
                print_output(&all, output);
            } else {
                let (domains, _) = client
                    .list_dns_domains(Some(list_args.per_page), list_args.cursor.as_deref())
                    .await?;
                print_output(&domains, output);
            }
        }

        DnsCommands::Get { domain } => {
            let dns_domain = client.get_dns_domain(&domain).await?;
            print_output(&dns_domain, output);
        }

        DnsCommands::Create {
            domain,
            ip,
            dns_sec,
        } => {
            let created = client
                .create_dns_domain(CreateDomainRequest {
                    domain,
                    ip,
                    dns_sec,
                })
                .await?;
            print_success(&format!("DNS domain {} created", created.domain));
            print_output(&created, output);
        }

        DnsCommands::Update { domain, dns_sec } => {
            client
                .update_dns_domain(
                    &domain,
                    UpdateDomainRequest {
                        dns_sec: Some(dns_sec),
                    },
                )
                .await?;
            print_success(&format!("DNS domain {} updated", domain));
        }

        DnsCommands::Delete { domain } => {
            if !skip_confirm && !confirm_delete("DNS domain", &domain)? {
                return Err(VultrError::Cancelled);
            }
            client.delete_dns_domain(&domain).await?;
            print_success(&format!("DNS domain {} deleted", domain));
        }

        DnsCommands::Soa { domain } => {
            let soa = client.get_dns_soa(&domain).await?;
            print_output(&soa, output);
        }

        DnsCommands::UpdateSoa {
            domain,
            nsprimary,
            email,
        } => {
            client
                .update_dns_soa(&domain, UpdateSoaRequest { nsprimary, email })
                .await?;
            print_success(&format!("SOA for {} updated", domain));
        }

        DnsCommands::Dnssec { domain } => {
            let dnssec = client.get_dns_dnssec(&domain).await?;
            print_output(&dnssec, output);
        }

        DnsCommands::Record(record_args) => {
            handle_dns_record(record_args.command, client, output, skip_confirm).await?;
        }
    }
    Ok(())
}

async fn handle_dns_record(
    command: DnsRecordCommands,
    client: &VultrClient,
    output: OutputFormat,
    skip_confirm: bool,
) -> VultrResult<()> {
    match command {
        DnsRecordCommands::List { domain, list } => {
            if list.all {
                let mut all = Vec::new();
                let mut cursor: Option<String> = None;
                loop {
                    let (page, meta) = client
                        .list_dns_records(&domain, Some(list.per_page), cursor.as_deref())
                        .await?;
                    all.extend(page);
                    cursor = meta.links.and_then(|l| l.next);
                    if cursor.is_none() {
                        break;
                    }
                }
                print_output(&all, output);
            } else {
                let (records, _) = client
                    .list_dns_records(&domain, Some(list.per_page), list.cursor.as_deref())
                    .await?;
                print_output(&records, output);
            }
        }

        DnsRecordCommands::Get { domain, id } => {
            let record = client.get_dns_record(&domain, &id).await?;
            print_output(&record, output);
        }

        DnsRecordCommands::Create {
            domain,
            name,
            record_type,
            data,
            ttl,
            priority,
        } => {
            let record = client
                .create_dns_record(
                    &domain,
                    CreateRecordRequest {
                        name,
                        record_type,
                        data,
                        ttl,
                        priority,
                    },
                )
                .await?;
            print_success(&format!("DNS record {} created", record.id));
            print_output(&record, output);
        }

        DnsRecordCommands::Update {
            domain,
            id,
            name,
            data,
            ttl,
            priority,
        } => {
            client
                .update_dns_record(
                    &domain,
                    &id,
                    UpdateRecordRequest {
                        name,
                        data,
                        ttl,
                        priority,
                    },
                )
                .await?;
            print_success(&format!("DNS record {} updated", id));
        }

        DnsRecordCommands::Delete { domain, id } => {
            if !skip_confirm && !confirm_delete("DNS record", &id)? {
                return Err(VultrError::Cancelled);
            }
            client.delete_dns_record(&domain, &id).await?;
            print_success(&format!("DNS record {} deleted", id));
        }
    }
    Ok(())
}
