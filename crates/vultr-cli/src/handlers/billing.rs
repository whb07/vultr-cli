//! Billing command handlers

use crate::commands::{BillingArgs, BillingCommands};
use vultr_api::VultrClient;
use vultr_config::OutputFormat;
use vultr_config::VultrResult;
use vultr_output::print_output;

pub async fn handle_billing(
    args: BillingArgs,
    client: &VultrClient,
    output: OutputFormat,
) -> VultrResult<()> {
    match args.command {
        BillingCommands::History(list_args) => {
            if list_args.all {
                let mut all = Vec::new();
                let mut cursor: Option<String> = None;
                loop {
                    let (page, meta) = client
                        .list_billing_history(Some(list_args.per_page), cursor.as_deref())
                        .await?;
                    all.extend(page);
                    cursor = meta.and_then(|m| m.links.and_then(|l| l.next));
                    if cursor.is_none() {
                        break;
                    }
                }
                print_output(&all, output);
            } else {
                let (history, _) = client
                    .list_billing_history(Some(list_args.per_page), list_args.cursor.as_deref())
                    .await?;
                print_output(&history, output);
            }
        }

        BillingCommands::Invoices(list_args) => {
            if list_args.all {
                let mut all = Vec::new();
                let mut cursor: Option<String> = None;
                loop {
                    let (page, meta) = client
                        .list_invoices(Some(list_args.per_page), cursor.as_deref())
                        .await?;
                    all.extend(page);
                    cursor = meta.and_then(|m| m.links.and_then(|l| l.next));
                    if cursor.is_none() {
                        break;
                    }
                }
                print_output(&all, output);
            } else {
                let (invoices, _) = client
                    .list_invoices(Some(list_args.per_page), list_args.cursor.as_deref())
                    .await?;
                print_output(&invoices, output);
            }
        }

        BillingCommands::Invoice { id } => {
            let invoice = client.get_invoice(id).await?;
            print_output(&invoice, output);
        }

        BillingCommands::InvoiceItems(args) => {
            if args.list_args.all {
                let mut all = Vec::new();
                let mut cursor: Option<String> = None;
                loop {
                    let (page, meta) = client
                        .list_invoice_items(
                            args.invoice_id,
                            Some(args.list_args.per_page),
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
                let (items, _) = client
                    .list_invoice_items(
                        args.invoice_id,
                        Some(args.list_args.per_page),
                        args.list_args.cursor.as_deref(),
                    )
                    .await?;
                print_output(&items, output);
            }
        }

        BillingCommands::PendingCharges => {
            let charges = client.list_pending_charges().await?;
            print_output(&charges, output);
        }

        BillingCommands::PendingChargesCsv => {
            let csv = client.get_pending_charges_csv().await?;
            println!("{}", csv);
        }
    }
    Ok(())
}
