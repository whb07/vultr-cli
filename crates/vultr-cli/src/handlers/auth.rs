//! Authentication command handlers

use crate::commands::{AuthArgs, AuthCommands};
use vultr_api::VultrClient;
use vultr_config::SecureStorage;
use vultr_config::{VultrError, VultrResult};
use vultr_output::{print_info, print_success, print_warning};

pub async fn handle_auth(args: AuthArgs, profile: &str) -> VultrResult<()> {
    let cfg = vultr_config::Config::load().unwrap_or_default();

    match args.command {
        AuthCommands::Login(login_args) => {
            let api_key = if let Some(key) = login_args.api_key {
                key
            } else {
                let key = dialoguer::Input::<String>::new()
                    .with_prompt("Enter your Vultr API key")
                    .interact_text()?;
                key.trim().to_string()
            };

            if api_key.is_empty() {
                return Err(VultrError::InvalidApiKey);
            }

            let client = VultrClient::new(
                api_key.clone(),
                cfg.settings.http.timeout,
                cfg.settings.http.max_retries,
                cfg.settings.http.backoff_initial_ms,
                cfg.settings.http.backoff_max_ms,
            )?;

            // Validate the API key with a light call
            client.list_regions().await?;

            SecureStorage::store_token(profile, &api_key)?;
            print_success(&format!(
                "API key stored securely for profile '{}'",
                profile
            ));
            Ok(())
        }

        AuthCommands::Logout => {
            SecureStorage::delete_token(profile)?;
            print_success(&format!("API key removed for profile '{}'", profile));
            Ok(())
        }

        AuthCommands::Status => {
            match SecureStorage::get_token(profile)? {
                Some(_) => print_success(&format!("Authenticated (profile: {})", profile)),
                None => {
                    if std::env::var("VULTR_API_KEY").is_ok() {
                        print_info("Using API key from VULTR_API_KEY environment variable");
                    } else {
                        print_warning("Not authenticated. Run 'vultr auth login' to authenticate.");
                    }
                }
            }
            Ok(())
        }
    }
}
