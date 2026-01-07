use std::collections::{BTreeSet, HashMap, HashSet};
use std::fs;
use std::path::Path;

use serde_json::Value;
use syn::Item;
use walkdir::WalkDir;

fn collect_rust_types() -> HashSet<String> {
    let mut types = HashSet::new();
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("src/models");

    for entry in WalkDir::new(root).into_iter().filter_map(Result::ok) {
        if !entry.file_type().is_file() {
            continue;
        }
        if entry.path().extension().and_then(|s| s.to_str()) != Some("rs") {
            continue;
        }

        let content = fs::read_to_string(entry.path())
            .unwrap_or_else(|e| panic!("Failed to read {}: {}", entry.path().display(), e));
        let file = syn::parse_file(&content)
            .unwrap_or_else(|e| panic!("Failed to parse {}: {}", entry.path().display(), e));

        for item in file.items {
            match item {
                Item::Struct(item) => {
                    types.insert(item.ident.to_string());
                }
                Item::Enum(item) => {
                    types.insert(item.ident.to_string());
                }
                Item::Type(item) => {
                    types.insert(item.ident.to_string());
                }
                _ => {}
            }
        }
    }

    types
}

fn to_pascal_case(name: &str) -> String {
    name.split(|c: char| !c.is_ascii_alphanumeric())
        .filter(|part| !part.is_empty())
        .map(|part| segment_case(part))
        .collect::<Vec<_>>()
        .join("")
}

fn segment_case(segment: &str) -> String {
    let lower = segment.to_ascii_lowercase();
    let mapped = match lower.as_str() {
        "ip" => "Ip",
        "ipv4" => "Ipv4",
        "ipv6" => "Ipv6",
        "api" => "Api",
        "apikey" => "ApiKey",
        "ssh" => "Ssh",
        "os" => "Os",
        "cpu" => "Cpu",
        "ram" => "Ram",
        "vpc" => "Vpc",
        "vpc2" => "Vpc2",
        "vfs" => "Vfs",
        "vke" => "Vke",
        "lb" => "Lb",
        "bgp" => "Bgp",
        "dns" => "Dns",
        "ssl" => "Ssl",
        "tls" => "Tls",
        "id" => "Id",
        "uuid" => "Uuid",
        "vm" => "Vm",
        "gpu" => "Gpu",
        "baremetal" => "BareMetal",
        "loadbalancer" => "LoadBalancer",
        "blockstorage" => "BlockStorage",
        "objectstorage" => "ObjectStorage",
        "pushzonefilemeta" => "PushZoneFileMeta",
        "pushzonefile" => "PushZoneFile",
        "pullzone" => "PullZone",
        "pushzone" => "PushZone",
        _ => {
            let mut chars = lower.chars();
            let mut out = String::new();
            if let Some(first) = chars.next() {
                out.extend(first.to_uppercase());
                out.push_str(chars.as_str());
            }
            return out;
        }
    };
    mapped.to_string()
}

fn schema_aliases() -> HashMap<&'static str, &'static [&'static str]> {
    let mut aliases = HashMap::new();
    aliases.insert("access-control", &["UserAccessControl"][..]);
    aliases.insert("account-bgp", &["BgpInfo"][..]);
    aliases.insert("attached-vpcs", &["AttachedVpc"][..]);
    aliases.insert("baremetal-get", &["BareMetal"][..]);
    aliases.insert("billing", &["BillingHistory"][..]);
    aliases.insert("database-available-connector", &["AvailableConnector"][..]);
    aliases.insert("database-connector", &["KafkaConnector"][..]);
    aliases.insert("database-connector-status", &["ConnectorStatus"][..]);
    aliases.insert("database-connector-status-task", &["ConnectorTaskStatus"][..]);
    aliases.insert("database-db", &["LogicalDatabase"][..]);
    aliases.insert("database-latest-backup", &["DatabaseBackup"][..]);
    aliases.insert("database-oldest-backup", &["DatabaseBackup"][..]);
    aliases.insert("database-topic", &["KafkaTopic"][..]);
    aliases.insert("dbaas-alerts", &["DatabaseAlert"][..]);
    aliases.insert("dbaas-migration", &["DatabaseMigration"][..]);
    aliases.insert("dbaas-plan", &["DatabasePlan"][..]);
    aliases.insert("instance-get", &["Instance"][..]);
    aliases.insert("iso-public", &["PublicIso"][..]);
    aliases.insert("loadbalancer-firewall-rule", &["LBFirewallRule"][..]);
    aliases.insert("nodepool-instances", &["KubeNode"][..]);
    aliases.insert(
        "nodepool-label-req",
        &["CreateNodePoolLabelRequest", "NodePoolLabel"][..],
    );
    aliases.insert("nodepool-taint-req", &["NodePoolTaintRequest"][..]);
    aliases.insert("plans-metal", &["BareMetalPlan"][..]);
    aliases.insert("private-networks", &["Vpc"][..]);
    aliases.insert("pullzone", &["CdnPullZone"][..]);
    aliases.insert("pushzone", &["CdnPushZone"][..]);
    aliases.insert("pushzonefile", &["CdnPushZoneFile"][..]);
    aliases.insert("pushzonefilemeta", &["CdnPushZoneFileMeta"][..]);
    aliases.insert(
        "registry-docker-credentials",
        &["RegistryDockerCredentials"][..],
    );
    aliases.insert(
        "registry-kubernetes-docker-credentials",
        &["RegistryKubernetesCredentials"][..],
    );
    aliases.insert("registry-plan", &["RegistryPlan"][..]);
    aliases.insert("registry-region", &["RegistryRegion"][..]);
    aliases.insert("registry-repository", &["RegistryRepository"][..]);
    aliases.insert(
        "registry-repository-artifact",
        &["RegistryArtifact"][..],
    );
    aliases.insert("registry-robot", &["RegistryRobot"][..]);
    aliases.insert("registry-storage", &["RegistryStorage"][..]);
    aliases.insert("registry-user", &["RegistryUser"][..]);
    aliases.insert("registry-user-current", &["RegistryUser"][..]);
    aliases.insert("replication", &["RegistryReplication"][..]);
    aliases.insert("tiers", &["ObjectStorageTier"][..]);
    aliases.insert("uploadendpoint", &["CdnUploadEndpoint"][..]);
    aliases.insert("uploadendpoint-inputs", &["CdnUploadEndpointInputs"][..]);
    aliases.insert("vke-cluster", &["KubernetesCluster"][..]);
    aliases.insert("ssh", &["SshKey"][..]);
    aliases.insert("startup", &["StartupScript"][..]);
    aliases
}

#[test]
fn openapi_schema_coverage() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let openapi_path = manifest_dir.join("openapi.json");
    let openapi = fs::read_to_string(&openapi_path)
        .unwrap_or_else(|e| panic!("Failed to read {}: {}", openapi_path.display(), e));
    let doc: Value =
        serde_json::from_str(&openapi).expect("Failed to parse openapi.json as JSON");

    let schemas = doc
        .get("components")
        .and_then(|c| c.get("schemas"))
        .and_then(|s| s.as_object())
        .expect("openapi.json missing components.schemas");

    let rust_types = collect_rust_types();
    let rust_lower: HashSet<String> = rust_types
        .iter()
        .map(|name| name.to_ascii_lowercase())
        .collect();

    let aliases = schema_aliases();
    let mut missing = BTreeSet::new();

    for schema_name in schemas.keys() {
        if let Some(alias_list) = aliases.get(schema_name.as_str()) {
            if alias_list
                .iter()
                .any(|alias| rust_lower.contains(&alias.to_ascii_lowercase()))
            {
                continue;
            }
        }

        let pascal = to_pascal_case(schema_name);
        let mut candidates = vec![
            pascal.clone(),
            format!("{pascal}Response"),
            format!("{pascal}sResponse"),
            format!("{pascal}ResponseWithMeta"),
            format!("{pascal}sResponseWithMeta"),
        ];

        if pascal.ends_with("Get") {
            candidates.push(pascal.trim_end_matches("Get").to_string());
        }

        if candidates
            .iter()
            .all(|cand| !rust_lower.contains(&cand.to_ascii_lowercase()))
        {
            missing.insert(schema_name.clone());
        }
    }

    let expected_missing: BTreeSet<String> = KNOWN_MISSING_SCHEMAS
        .iter()
        .map(|name| name.to_string())
        .collect();

    let new_missing: Vec<_> = missing
        .difference(&expected_missing)
        .cloned()
        .collect();
    let resolved_missing: Vec<_> = expected_missing
        .difference(&missing)
        .cloned()
        .collect();

    assert!(
        new_missing.is_empty() && resolved_missing.is_empty(),
        "OpenAPI schema coverage drift detected.\nNew missing: {new_missing:?}\nResolved missing: {resolved_missing:?}"
    );
}

const KNOWN_MISSING_SCHEMAS: &[&str] = &[
    "app-variable",
    "database-connections",
    "database-connector-configuration-schema",
    "dbaas-available-options",
    "dbaas-meta",
    "inference-subscription",
    "inference-usage",
    "kafka-advanced-options",
    "kafka-connect-advanced-options",
    "kafka-permissions",
    "kafka-rest-advanced-options",
    "log",
    "log-meta",
    "mysql-advanced-options",
    "network",
    "pg-advanced-options",
    "schema-registry-advanced-options",
    "storage-gateway",
    "storage-gateway-export",
    "storage-gateway-network",
    "subaccount",
    "vfs",
    "vfs_attachment",
    "vfs_billing",
    "vfs_region",
    "vfs_storage_size",
    "vpc-attachment",
    "vpc-attachment-ip",
    "vpc-attachment-linked-subscription",
    "vpc-internet",
];
