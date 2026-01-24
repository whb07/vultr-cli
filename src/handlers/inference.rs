//! Inference command handlers

use crate::api::VultrClient;
use crate::commands::{InferenceArgs, InferenceCommands};
use crate::config::OutputFormat;
use crate::error::VultrResult;
use crate::handlers::confirm_delete;
use crate::models::{CreateInferenceRequest, UpdateInferenceRequest};
use crate::output::{print_output, print_success};

pub async fn handle_inference(
    args: InferenceArgs,
    client: &VultrClient,
    output: OutputFormat,
    skip_confirm: bool,
) -> VultrResult<()> {
    match args.command {
        InferenceCommands::List => {
            let subs = client.list_inference().await?;
            print_output(&subs, output);
        }
        InferenceCommands::Get { id } => {
            let sub = client.get_inference(&id).await?;
            print_output(&sub, output);
        }
        InferenceCommands::Create { label } => {
            let request = CreateInferenceRequest { label };
            let sub = client.create_inference(request).await?;
            print_output(&sub, output);
        }
        InferenceCommands::Update { id, label } => {
            let request = UpdateInferenceRequest { label: Some(label) };
            let sub = client.update_inference(&id, request).await?;
            print_output(&sub, output);
        }
        InferenceCommands::Delete { id } => {
            if skip_confirm || confirm_delete("inference subscription", &id)? {
                client.delete_inference(&id).await?;
                print_success(&format!("Inference subscription {} deleted", id));
            }
        }
        InferenceCommands::Usage { id } => {
            let usage = client.get_inference_usage(&id).await?;
            print_output(&usage, output);
        }
    }
    Ok(())
}
