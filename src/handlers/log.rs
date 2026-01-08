//! Log command handlers

use crate::api::VultrClient;
use crate::commands::LogsArgs;
use crate::config::OutputFormat;
use crate::error::VultrResult;
use crate::output::print_output;

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
        )
        .await?;
    print_output(&logs, output);
    Ok(())
}
