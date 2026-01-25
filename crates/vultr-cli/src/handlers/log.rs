//! Log command handlers

use crate::commands::LogsArgs;
use vultr_api::VultrClient;
use vultr_config::OutputFormat;
use vultr_config::VultrResult;
use vultr_output::print_output;

pub async fn handle_logs(
    args: LogsArgs,
    client: &VultrClient,
    output: OutputFormat,
) -> VultrResult<()> {
    let logs = client
        .list_logs(
            args.start_time.as_deref(),
            args.end_time.as_deref(),
            args.log_level.as_deref(),
            args.resource_type.as_deref(),
            args.resource_id.as_deref(),
            args.continue_time.as_deref(),
        )
        .await?;
    print_output(&logs, output);
    Ok(())
}
