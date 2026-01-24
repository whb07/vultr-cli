//! Load Balancer command handlers

use vultr_api::VultrClient;
use crate::commands::*;
use vultr_config::OutputFormat;
use vultr_config::VultrResult;
use vultr_models::*;
use vultr_output::{print_output, print_success};

use super::confirm_delete;

pub async fn handle_load_balancer(
    args: LoadBalancerArgs,
    client: &VultrClient,
    output: OutputFormat,
    skip_confirm: bool,
) -> VultrResult<()> {
    match args.command {
        LoadBalancerCommands::List(list_args) => {
            let (load_balancers, _meta) = client
                .list_load_balancers(Some(list_args.per_page), list_args.cursor.as_deref())
                .await?;
            print_output(&load_balancers, output);
        }
        LoadBalancerCommands::Get { id } => {
            let lb = client.get_load_balancer(&id).await?;
            print_output(&lb, output);
        }
        LoadBalancerCommands::Create(create_args) => {
            let health_check = if create_args.health_check_protocol.is_some()
                || create_args.health_check_port.is_some()
            {
                Some(HealthCheck {
                    protocol: create_args.health_check_protocol,
                    port: create_args.health_check_port,
                    path: create_args.health_check_path,
                    check_interval: create_args.health_check_interval,
                    response_timeout: create_args.health_check_timeout,
                    unhealthy_threshold: create_args.health_check_unhealthy_threshold,
                    healthy_threshold: create_args.health_check_healthy_threshold,
                })
            } else {
                None
            };

            let sticky_session =
                create_args
                    .sticky_session_cookie
                    .map(|cookie_name| StickySessions {
                        cookie_name: Some(cookie_name),
                    });

            let request = CreateLoadBalancerRequest {
                region: create_args.region,
                label: create_args.label,
                balancing_algorithm: create_args.balancing_algorithm,
                ssl_redirect: if create_args.ssl_redirect {
                    Some(true)
                } else {
                    None
                },
                http2: if create_args.http2 { Some(true) } else { None },
                http3: if create_args.http3 { Some(true) } else { None },
                nodes: create_args.nodes,
                proxy_protocol: if create_args.proxy_protocol {
                    Some(true)
                } else {
                    None
                },
                timeout: create_args.timeout,
                vpc: create_args.vpc,
                health_check,
                sticky_session,
                instances: create_args.instances,
                ..Default::default()
            };
            let lb = client.create_load_balancer(request).await?;
            print_output(&lb, output);
        }
        LoadBalancerCommands::Update(update_args) => {
            let health_check = if update_args.health_check_protocol.is_some()
                || update_args.health_check_port.is_some()
            {
                Some(HealthCheck {
                    protocol: update_args.health_check_protocol,
                    port: update_args.health_check_port,
                    path: update_args.health_check_path,
                    check_interval: update_args.health_check_interval,
                    response_timeout: update_args.health_check_timeout,
                    unhealthy_threshold: update_args.health_check_unhealthy_threshold,
                    healthy_threshold: update_args.health_check_healthy_threshold,
                })
            } else {
                None
            };

            let sticky_session =
                update_args
                    .sticky_session_cookie
                    .map(|cookie_name| StickySessions {
                        cookie_name: Some(cookie_name),
                    });

            let request = UpdateLoadBalancerRequest {
                label: update_args.label,
                balancing_algorithm: update_args.balancing_algorithm,
                ssl_redirect: update_args.ssl_redirect,
                http2: update_args.http2,
                http3: update_args.http3,
                nodes: update_args.nodes,
                proxy_protocol: update_args.proxy_protocol,
                timeout: update_args.timeout,
                vpc: update_args.vpc,
                health_check,
                sticky_session,
                instances: update_args.instances,
                ..Default::default()
            };
            let lb = client
                .update_load_balancer(&update_args.id, request)
                .await?;
            print_output(&lb, output);
        }
        LoadBalancerCommands::Delete { id } => {
            if skip_confirm || confirm_delete("load balancer", &id)? {
                client.delete_load_balancer(&id).await?;
                print_success(&format!("Load balancer {} deleted", id));
            }
        }
        LoadBalancerCommands::Ssl(ssl_args) => {
            handle_load_balancer_ssl(ssl_args, client, skip_confirm).await?;
        }
        LoadBalancerCommands::ForwardingRule(rule_args) => {
            handle_load_balancer_forwarding_rule(rule_args, client, output, skip_confirm).await?;
        }
        LoadBalancerCommands::FirewallRule(rule_args) => {
            handle_load_balancer_firewall_rule(rule_args, client, output, skip_confirm).await?;
        }
        LoadBalancerCommands::ReverseDns(rdns_args) => {
            handle_load_balancer_reverse_dns(rdns_args, client, output).await?;
        }
    }
    Ok(())
}

async fn handle_load_balancer_ssl(
    args: LoadBalancerSslArgs,
    client: &VultrClient,
    skip_confirm: bool,
) -> VultrResult<()> {
    match args.command {
        LoadBalancerSslCommands::Add {
            lb_id,
            private_key,
            certificate,
            chain,
            private_key_b64,
            certificate_b64,
            chain_b64,
        } => {
            let ssl = SSLConfig {
                private_key,
                certificate,
                chain,
                private_key_b64,
                certificate_b64,
                chain_b64,
            };
            client.create_load_balancer_ssl(&lb_id, ssl).await?;
            print_success(&format!("SSL certificate added to load balancer {}", lb_id));
        }
        LoadBalancerSslCommands::Delete { lb_id } => {
            if skip_confirm || confirm_delete("SSL certificate", &lb_id)? {
                client.delete_load_balancer_ssl(&lb_id).await?;
                print_success(&format!(
                    "SSL certificate deleted from load balancer {}",
                    lb_id
                ));
            }
        }
        LoadBalancerSslCommands::DisableAutoSsl { lb_id } => {
            client.disable_load_balancer_auto_ssl(&lb_id).await?;
            print_success(&format!("Auto SSL disabled for load balancer {}", lb_id));
        }
    }
    Ok(())
}

async fn handle_load_balancer_forwarding_rule(
    args: LoadBalancerForwardingRuleArgs,
    client: &VultrClient,
    output: OutputFormat,
    skip_confirm: bool,
) -> VultrResult<()> {
    match args.command {
        LoadBalancerForwardingRuleCommands::List { lb_id } => {
            let (rules, _meta) = client
                .list_load_balancer_forwarding_rules(&lb_id, None, None)
                .await?;
            print_output(&rules, output);
        }
        LoadBalancerForwardingRuleCommands::Get { lb_id, rule_id } => {
            let rule = client
                .get_load_balancer_forwarding_rule(&lb_id, &rule_id)
                .await?;
            print_output(&rule, output);
        }
        LoadBalancerForwardingRuleCommands::Create {
            lb_id,
            frontend_protocol,
            frontend_port,
            backend_protocol,
            backend_port,
        } => {
            let request = CreateForwardingRuleRequest {
                frontend_protocol,
                frontend_port,
                backend_protocol,
                backend_port,
            };
            client
                .create_load_balancer_forwarding_rule(&lb_id, request)
                .await?;
            print_success(&format!(
                "Forwarding rule created for load balancer {}",
                lb_id
            ));
        }
        LoadBalancerForwardingRuleCommands::Delete { lb_id, rule_id } => {
            if skip_confirm || confirm_delete("forwarding rule", &rule_id)? {
                client
                    .delete_load_balancer_forwarding_rule(&lb_id, &rule_id)
                    .await?;
                print_success(&format!("Forwarding rule {} deleted", rule_id));
            }
        }
    }
    Ok(())
}

async fn handle_load_balancer_firewall_rule(
    args: LoadBalancerFirewallRuleArgs,
    client: &VultrClient,
    output: OutputFormat,
    skip_confirm: bool,
) -> VultrResult<()> {
    match args.command {
        LoadBalancerFirewallRuleCommands::List { lb_id } => {
            let (rules, _meta) = client
                .list_load_balancer_firewall_rules(&lb_id, None, None)
                .await?;
            print_output(&rules, output);
        }
        LoadBalancerFirewallRuleCommands::Get { lb_id, rule_id } => {
            let rule = client
                .get_load_balancer_firewall_rule(&lb_id, &rule_id)
                .await?;
            print_output(&rule, output);
        }
        LoadBalancerFirewallRuleCommands::Create {
            lb_id,
            port,
            source,
            ip_type,
        } => {
            let request = CreateLBFirewallRuleRequest {
                port,
                source,
                ip_type,
            };
            let rule = client
                .create_load_balancer_firewall_rule(&lb_id, request)
                .await?;
            print_output(&rule, output);
        }
        LoadBalancerFirewallRuleCommands::Delete { lb_id, rule_id } => {
            if skip_confirm || confirm_delete("firewall rule", &rule_id)? {
                client
                    .delete_load_balancer_firewall_rule(&lb_id, &rule_id)
                    .await?;
                print_success(&format!("Firewall rule {} deleted", rule_id));
            }
        }
    }
    Ok(())
}

async fn handle_load_balancer_reverse_dns(
    args: LoadBalancerReverseDnsArgs,
    client: &VultrClient,
    output: OutputFormat,
) -> VultrResult<()> {
    match args.command {
        LoadBalancerReverseDnsCommands::Get { lb_id } => {
            let rdns = client.get_load_balancer_reverse_dns(&lb_id).await?;
            print_output(&rdns, output);
        }
        LoadBalancerReverseDnsCommands::UpdateIpv4 { lb_id, domain } => {
            let request = UpdateReverseDNSv4Request { v4: domain };
            client
                .update_load_balancer_reverse_dns_ipv4(&lb_id, request)
                .await?;
            print_success(&format!(
                "IPv4 reverse DNS updated for load balancer {}",
                lb_id
            ));
        }
        LoadBalancerReverseDnsCommands::CreateIpv6 { lb_id, ip, domain } => {
            let request = CreateReverseDNSv6Request {
                v6: vec![ReverseIPv6Entry { ip, domain }],
            };
            client
                .create_load_balancer_reverse_dns_ipv6(&lb_id, request)
                .await?;
            print_success(&format!(
                "IPv6 reverse DNS created for load balancer {}",
                lb_id
            ));
        }
    }
    Ok(())
}
