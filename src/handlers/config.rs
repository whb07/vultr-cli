//! Configuration handler

use crate::commands::{ConfigArgs, ConfigCommands};
use crate::config::{Config, OutputFormat};
use crate::error::{VultrError, VultrResult};
use crate::output::print_json;

pub fn handle_config(args: ConfigArgs, current_profile: &str) -> VultrResult<()> {
    match args.command {
        ConfigCommands::Show => {
            let config = Config::load()?;
            print_json(&config);
            Ok(())
        }
        ConfigCommands::Get { key } => {
            let config = Config::load()?;
            let value: Option<String> = match key.as_str() {
                "output_format" => Some(config.settings.output_format.to_string()),
                "confirm_destructive" => Some(config.settings.confirm_destructive.to_string()),
                "wait_timeout" => Some(config.settings.wait_timeout.to_string()),
                "poll_interval" => Some(config.settings.poll_interval.to_string()),
                "default_profile" => Some(config.default_profile.clone()),
                "http.timeout" => Some(config.settings.http.timeout.to_string()),
                "http.max_retries" => Some(config.settings.http.max_retries.to_string()),
                "http.backoff_initial_ms" => {
                    Some(config.settings.http.backoff_initial_ms.to_string())
                }
                "http.backoff_max_ms" => Some(config.settings.http.backoff_max_ms.to_string()),
                _ => {
                    return Err(VultrError::InvalidInput(format!(
                        "Unknown config key: {}. Valid keys: output_format, confirm_destructive, \
                         wait_timeout, poll_interval, default_profile, \
                         http.timeout, http.max_retries, http.backoff_initial_ms, http.backoff_max_ms",
                        key
                    )));
                }
            };

            match value {
                Some(v) => println!("{}", v),
                None => println!("(not set)"),
            }
            Ok(())
        }
        ConfigCommands::Set { key, value } => {
            let mut config = Config::load()?;

            match key.as_str() {
                "output_format" => {
                    config.settings.output_format = value
                        .parse::<OutputFormat>()
                        .map_err(|e| VultrError::InvalidInput(e))?;
                }
                "confirm_destructive" => {
                    config.settings.confirm_destructive = value.parse::<bool>().map_err(|_| {
                        VultrError::InvalidInput("Value must be 'true' or 'false'".to_string())
                    })?;
                }
                "wait_timeout" => {
                    config.settings.wait_timeout = value.parse::<u64>().map_err(|_| {
                        VultrError::InvalidInput("Value must be a positive integer".to_string())
                    })?;
                }
                "poll_interval" => {
                    config.settings.poll_interval = value.parse::<u64>().map_err(|_| {
                        VultrError::InvalidInput("Value must be a positive integer".to_string())
                    })?;
                }
                "default_profile" => {
                    config.default_profile = value;
                }
                "http.timeout" => {
                    config.settings.http.timeout = value.parse::<u64>().map_err(|_| {
                        VultrError::InvalidInput("Value must be a positive integer".to_string())
                    })?;
                }
                "http.max_retries" => {
                    config.settings.http.max_retries = value.parse::<u32>().map_err(|_| {
                        VultrError::InvalidInput("Value must be a positive integer".to_string())
                    })?;
                }
                "http.backoff_initial_ms" => {
                    config.settings.http.backoff_initial_ms =
                        value.parse::<u64>().map_err(|_| {
                            VultrError::InvalidInput("Value must be a positive integer".to_string())
                        })?;
                }
                "http.backoff_max_ms" => {
                    config.settings.http.backoff_max_ms = value.parse::<u64>().map_err(|_| {
                        VultrError::InvalidInput("Value must be a positive integer".to_string())
                    })?;
                }
                _ => {
                    return Err(VultrError::InvalidInput(format!(
                        "Unknown config key: {}. Valid keys: output_format, confirm_destructive, \
                         wait_timeout, poll_interval, default_profile, \
                         http.timeout, http.max_retries, http.backoff_initial_ms, http.backoff_max_ms",
                        key
                    )));
                }
            }

            config.save()?;
            println!("Configuration saved.");
            Ok(())
        }
        ConfigCommands::Profile => {
            let config = Config::load()?;
            let profile_name = if config.default_profile.is_empty() {
                "default"
            } else {
                &config.default_profile
            };
            println!("Current profile: {}", current_profile);
            println!("Default profile: {}", profile_name);

            if let Some(profile) = config.profiles.get(current_profile) {
                println!("Settings:");
                if let Some(ref format) = profile.output_format {
                    println!("  output_format: {}", format);
                }
            } else {
                println!("(No profile settings configured)");
            }
            Ok(())
        }
    }
}
