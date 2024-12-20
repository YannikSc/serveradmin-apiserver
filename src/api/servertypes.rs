///
/// This file is generated using the codegen_servertypes command!
///


#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, utoipa::ToSchema)]
#[schema(as = serveradmin::innogames::de::v1::MemoryModuleSpec)]
#[serde_inline_default::serde_inline_default]
#[serde(rename_all = "snake_case")]
pub struct MemoryModuleSpec {
    capacity: i32,
    hardware_model: String,
    serial_number: String,
    state: String,
    installed_in: Option<String>,
    hostname: String,
    servertype: String,
    object_id: String,

}
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, utoipa::ToSchema)] #[schema(as = serveradmin::innogames::de::v1::MemoryModuleList)] #[serde(rename_all = "snake_case")] pub struct MemoryModuleList {
    kind: String,
    api_version: String,
    items: MemoryModule,

}
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, utoipa::ToSchema)] #[schema(as = serveradmin::innogames::de::v1::MemoryModule)] #[serde(rename_all = "snake_case")] pub struct MemoryModule {
    kind: String,
    api_version: String,
    metadata: ServerObjectMetadata,
    spec: MemoryModuleSpec,

}
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, utoipa::ToSchema)]
#[schema(as = serveradmin::innogames::de::v1::HdmiTransmitterSpec)]
#[serde_inline_default::serde_inline_default]
#[serde(rename_all = "snake_case")]
pub struct HdmiTransmitterSpec {
    #[serde_inline_default(String::from("online"))]state: String,
    #[serde_inline_default(String::from("officeit"))]project: String,
    mac: Option<Vec<String>>,
    route_network: Option<String>,
    network_zones: Option<Vec<String>>,
    officetool_id: Option<String>,
    ipv4: String,
    ipv6: String,
    hostname: String,
    servertype: String,
    object_id: String,

}
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, utoipa::ToSchema)] #[schema(as = serveradmin::innogames::de::v1::HdmiTransmitterList)] #[serde(rename_all = "snake_case")] pub struct HdmiTransmitterList {
    kind: String,
    api_version: String,
    items: HdmiTransmitter,

}
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, utoipa::ToSchema)] #[schema(as = serveradmin::innogames::de::v1::HdmiTransmitter)] #[serde(rename_all = "snake_case")] pub struct HdmiTransmitter {
    kind: String,
    api_version: String,
    metadata: ServerObjectMetadata,
    spec: HdmiTransmitterSpec,

}
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, utoipa::ToSchema)]
#[schema(as = serveradmin::innogames::de::v1::ServiceGroupSpec)]
#[serde_inline_default::serde_inline_default]
#[serde(rename_all = "snake_case")]
pub struct ServiceGroupSpec {
    function: Option<String>,
    #[serde_inline_default(Some(bool::from(false)))]no_logging: Option<bool>,
    project: String,
    #[serde_inline_default(String::from("online"))]state: String,
    subproject: Option<String>,
    responsible_admin: Vec<String>,
    sg_allow_from: Option<Vec<String>>,
    sg_allow_from_reverse: Option<Vec<String>>,
    sg_allow_to_reverse: Option<Vec<String>>,
    sg_allow_to: Option<Vec<String>>,
    service_group_members: Option<Vec<String>>,
    protocol_ports_inbound: Option<Vec<String>>,
    protocol_ports_outbound: Option<Vec<String>>,
    environment: Option<String>,
    provider: Option<String>,
    hostname: String,
    servertype: String,
    object_id: String,

}
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, utoipa::ToSchema)] #[schema(as = serveradmin::innogames::de::v1::ServiceGroupList)] #[serde(rename_all = "snake_case")] pub struct ServiceGroupList {
    kind: String,
    api_version: String,
    items: ServiceGroup,

}
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, utoipa::ToSchema)] #[schema(as = serveradmin::innogames::de::v1::ServiceGroup)] #[serde(rename_all = "snake_case")] pub struct ServiceGroup {
    kind: String,
    api_version: String,
    metadata: ServerObjectMetadata,
    spec: ServiceGroupSpec,

}
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, utoipa::ToSchema)]
#[schema(as = serveradmin::innogames::de::v1::MonitoringZoneSpec)]
#[serde_inline_default::serde_inline_default]
#[serde(rename_all = "snake_case")]
pub struct MonitoringZoneSpec {
    hostname: String,
    servertype: String,
    object_id: String,

}
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, utoipa::ToSchema)] #[schema(as = serveradmin::innogames::de::v1::MonitoringZoneList)] #[serde(rename_all = "snake_case")] pub struct MonitoringZoneList {
    kind: String,
    api_version: String,
    items: MonitoringZone,

}
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, utoipa::ToSchema)] #[schema(as = serveradmin::innogames::de::v1::MonitoringZone)] #[serde(rename_all = "snake_case")] pub struct MonitoringZone {
    kind: String,
    api_version: String,
    metadata: ServerObjectMetadata,
    spec: MonitoringZoneSpec,

}
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, utoipa::ToSchema)]
#[schema(as = serveradmin::innogames::de::v1::HwLoadbalancerSpec)]
#[serde_inline_default::serde_inline_default]
#[serde(rename_all = "snake_case")]
pub struct HwLoadbalancerSpec {
    order_date: Option<String>,
    order_id: Option<Vec<String>>,
    graphite_graphs: Option<Vec<String>>,
    primary_ip6: Option<String>,
    switchports: Option<Vec<String>>,
    puppet_master: String,
    service_groups: Option<Vec<String>>,
    no_monitoring: Option<bool>,
    security_availability: Option<String>,
    hw_loadbalancer_group: Option<String>,
    monitoring_groups: Option<Vec<String>>,
    repositories: Option<Vec<String>>,
    dell_servicetag: Option<Vec<String>>,
    storage_hardware: Vec<String>,
    security_confidentiality: Option<String>,
    hardware_components: Option<Vec<String>>,
    num_cpu: Option<i32>,
    disk_size_gib: Option<i32>,
    memory: Option<i32>,
    security_integrity: Option<String>,
    provisioning_step: Option<String>,
    cpu_clock_mhz: Option<i32>,
    ipmi: Option<String>,
    function: Option<String>,
    #[serde_inline_default(Some(vec![String::from("ping")]))]monitoring_checks: Option<Vec<String>>,
    bladecenter: Option<String>,
    puppet_environment: Option<String>,
    hardware_model: Option<String>,
    harddisk_type: Option<String>,
    cpu_model: Option<String>,
    #[serde_inline_default(Some(bool::from(false)))]puppet_disabled: Option<bool>,
    legal_personal_data: Option<String>,
    vlan_interfaces: Option<Vec<String>>,
    memory_modules: Option<Vec<String>>,
    bladecenter_slot: Option<Vec<String>>,
    rack: Option<String>,
    #[serde_inline_default(vec![String::from("kajetan.staszkiewicz")])]responsible_admin: Vec<String>,
    network_zones: Option<Vec<String>>,
    rack_height: Option<i32>,
    rack_position: Option<i32>,
    sshfp: Option<Vec<String>>,
    firmware_versions: Option<Vec<String>>,
    network_zone: Option<String>,
    puppet_classes: Option<Vec<String>>,
    route_network: Option<String>,
    project_network: Option<String>,
    #[serde_inline_default(String::from("online"))]state: String,
    #[serde_inline_default(String::from("production"))]environment: String,
    subproject: Option<String>,
    public_networks: Option<Vec<String>>,
    monitoring_queue: Option<String>,
    datacenter: Option<String>,
    vlan_networks: Option<Vec<String>>,
    #[serde_inline_default(String::from("ndco"))]project: String,
    datacenter_type: Option<String>,
    #[serde_inline_default(String::from("puppetca.innogames.de"))]puppet_ca: String,
    #[serde_inline_default(String::from("freebsd_11.2"))]os: String,
    #[serde_inline_default(String::from("amd64"))]arch: String,
    mac: Option<Vec<String>>,
    #[serde_inline_default(Some(String::from("aptly.innogames.de")))]repositories_host: Option<String>,
    #[serde_inline_default(String::from("kernel_bond_ab"))]network_link_type: String,
    nessus_scan_id: Option<i32>,
    ipv4: String,
    ipv6: String,
    hostname: String,
    servertype: String,
    object_id: String,

}
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, utoipa::ToSchema)] #[schema(as = serveradmin::innogames::de::v1::HwLoadbalancerList)] #[serde(rename_all = "snake_case")] pub struct HwLoadbalancerList {
    kind: String,
    api_version: String,
    items: HwLoadbalancer,

}
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, utoipa::ToSchema)] #[schema(as = serveradmin::innogames::de::v1::HwLoadbalancer)] #[serde(rename_all = "snake_case")] pub struct HwLoadbalancer {
    kind: String,
    api_version: String,
    metadata: ServerObjectMetadata,
    spec: HwLoadbalancerSpec,

}
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, utoipa::ToSchema)]
#[schema(as = serveradmin::innogames::de::v1::HardwareSpec)]
#[serde_inline_default::serde_inline_default]
#[serde(rename_all = "snake_case")]
pub struct HardwareSpec {
    backup_state: Option<String>,
    security_availability: Option<String>,
    vlan_networks: Option<Vec<String>>,
    repositories_host: Option<String>,
    provisioning_step: Option<String>,
    rack_height: Option<i32>,
    monitoring_queues: Option<Vec<String>>,
    rack_position: Option<i32>,
    security_confidentiality: Option<String>,
    #[serde_inline_default(Some(i32::from(300)))]backup_prio: Option<i32>,
    order_date: Option<String>,
    order_id: Option<Vec<String>>,
    #[serde_inline_default(Some(bool::from(false)))]no_monitoring: Option<bool>,
    #[serde_inline_default(Some(bool::from(false)))]puppet_disabled: Option<bool>,
    #[serde_inline_default(String::from("ssd"))]harddisk_type: String,
    primary_ip6: Option<String>,
    num_cpu: i32,
    loadbalancer: Option<Vec<String>>,
    security_integrity: Option<String>,
    disk_size_gib: Option<i32>,
    memory: Option<i32>,
    legal_personal_data: Option<String>,
    #[serde_inline_default(String::from("online"))]state: String,
    cpu_clock_mhz: Option<i32>,
    ipmi: Option<String>,
    puppet_environment: Option<String>,
    function: Option<String>,
    graphite_graphs: Option<Vec<String>>,
    bladecenter: Option<String>,
    route_network: Option<String>,
    dell_servicetag: Option<Vec<String>>,
    repositories: Option<Vec<String>>,
    allow_to: Option<Vec<String>>,
    #[serde_inline_default(String::from("production"))]environment: String,
    monitoring_queue: Option<String>,
    hardware_model: Option<String>,
    provider_network: Option<String>,
    cpu_model: Option<String>,
    memory_modules: Option<Vec<String>>,
    bladecenter_slot: Option<Vec<String>>,
    assigned_to: Option<String>,
    os_product_license: Option<String>,
    project_network: Option<String>,
    responsible_admin: Vec<String>,
    subproject: Option<String>,
    network_zones: Option<Vec<String>>,
    fixup_enabled: Option<bool>,
    description: Option<String>,
    sshfp: Option<Vec<String>>,
    puppet_classes: Option<Vec<String>>,
    #[serde_inline_default(Some(vec![String::from("ping")]))]monitoring_checks: Option<Vec<String>>,
    #[serde_inline_default(Some(String::from("aptly.innogames.de")))]apt_repository_url: Option<String>,
    allow_from: Option<Vec<String>>,
    datacenter: Option<String>,
    acl: Option<Vec<String>>,
    network_zone: Option<String>,
    monitoring_groups: Option<Vec<String>>,
    service_groups: Option<Vec<String>>,
    backup_storage: Option<String>,
    project: String,
    restic_keep_monthly: Option<i32>,
    restic_keep_yearly: Option<i32>,
    firmware_versions: Option<Vec<String>>,
    restic_group: Option<String>,
    #[serde_inline_default(Some(i32::from(3)))]restic_keep_daily: Option<i32>,
    #[serde_inline_default(Some(i32::from(3)))]restic_keep_last: Option<i32>,
    #[serde_inline_default(Some(i32::from(2)))]restic_keep_weekly: Option<i32>,
    datacenter_type: Option<String>,
    arch: Option<String>,
    #[serde_inline_default(bool::from(true))]auto_update: bool,
    #[serde_inline_default(vec![String::from("raid")])]storage_hardware: Vec<String>,
    #[serde_inline_default(bool::from(true))]backup_disabled: bool,
    #[serde_inline_default(Some(bool::from(false)))]backup_canshrink: Option<bool>,
    backup_maxage: Option<i32>,
    backup_window: Option<Vec<String>>,
    powerports: Option<Vec<String>>,
    switchports: Option<Vec<String>>,
    mac: Option<Vec<String>>,
    #[serde_inline_default(String::from("stretch"))]os: String,
    backup_timestamp: Option<String>,
    #[serde_inline_default(String::from("kernel_bond_ab"))]network_link_type: String,
    nessus_scan_id: Option<i32>,
    #[serde_inline_default(Some(String::from("puppetca.innogames.de")))]puppet_ca: Option<String>,
    puppet_master: Option<String>,
    rack: Option<String>,
    officetool_id: Option<String>,
    serial_number: Option<String>,
    ipv4: String,
    ipv6: String,
    hostname: String,
    servertype: String,
    object_id: String,

}
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, utoipa::ToSchema)] #[schema(as = serveradmin::innogames::de::v1::HardwareList)] #[serde(rename_all = "snake_case")] pub struct HardwareList {
    kind: String,
    api_version: String,
    items: Hardware,

}
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, utoipa::ToSchema)] #[schema(as = serveradmin::innogames::de::v1::Hardware)] #[serde(rename_all = "snake_case")] pub struct Hardware {
    kind: String,
    api_version: String,
    metadata: ServerObjectMetadata,
    spec: HardwareSpec,

}
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, utoipa::ToSchema)]
#[schema(as = serveradmin::innogames::de::v1::HypervisorSpec)]
#[serde_inline_default::serde_inline_default]
#[serde(rename_all = "snake_case")]
pub struct HypervisorSpec {
    provisioning_step: Option<String>,
    graphite_graphs: Option<Vec<String>>,
    route_network: Option<String>,
    igvm_locked: Option<String>,
    responsible_admin: Vec<String>,
    firmware_versions: Option<Vec<String>>,
    #[serde_inline_default(i32::from(0))]cpu_util_pct_tmp: i32,
    puppet_master: String,
    backup_state: Option<String>,
    vms: Option<Vec<String>>,
    repositories: Option<Vec<String>>,
    #[serde_inline_default(Some(i32::from(300)))]backup_prio: Option<i32>,
    order_date: Option<String>,
    order_id: Option<Vec<String>>,
    #[serde_inline_default(Some(bool::from(false)))]no_monitoring: Option<bool>,
    #[serde_inline_default(Some(bool::from(false)))]puppet_disabled: Option<bool>,
    #[serde_inline_default(String::from("ssd"))]harddisk_type: String,
    security_availability: Option<String>,
    primary_ip6: Option<String>,
    monitoring_groups: Option<Vec<String>>,
    security_confidentiality: Option<String>,
    num_cpu: i32,
    memory: i32,
    disk_size_gib: i32,
    security_integrity: Option<String>,
    cpu_clock_mhz: Option<i32>,
    bladecenter: Option<String>,
    ipmi: Option<String>,
    function: Option<String>,
    dell_servicetag: Option<Vec<String>>,
    puppet_environment: Option<String>,
    #[serde_inline_default(String::from("production"))]environment: String,
    #[serde_inline_default(i32::from(0))]cpu_util_vm_pct: i32,
    #[serde_inline_default(i32::from(0))]cpu_util_pct: i32,
    hardware_model: String,
    cpu_model: Option<String>,
    provider_network: Option<String>,
    legal_personal_data: Option<String>,
    memory_usage_gib: Option<i32>,
    disk_free_gib: Option<i32>,
    load_avg: Option<i32>,
    iops_avg: Option<i32>,
    memory_modules: Option<Vec<String>>,
    bladecenter_slot: Option<Vec<String>>,
    assigned_to: Option<String>,
    os_product_license: Option<String>,
    sshfp: Option<Vec<String>>,
    hardware_components: Option<Vec<String>>,
    #[serde_inline_default(Some(vec![String::from("ping")]))]monitoring_checks: Option<Vec<String>>,
    puppet_classes: Option<Vec<String>>,
    rack_height: Option<i32>,
    rack_position: Option<i32>,
    vlan_networks: Option<Vec<String>>,
    rack: Option<String>,
    #[serde_inline_default(String::from("online"))]state: String,
    project_network: Option<String>,
    subproject: Option<String>,
    network_zone: Option<String>,
    monitoring_queue: Option<String>,
    network_zones: Option<Vec<String>>,
    datacenter: Option<String>,
    service_groups: Option<Vec<String>>,
    backup_storage: Option<String>,
    #[serde_inline_default(String::from("ndco"))]project: String,
    libvirt_pool_free_gib: Option<i32>,
    libvirt_pool_total_gib: Option<i32>,
    libvirt_memory_total_gib: Option<i32>,
    libvirt_memory_free_gib: Option<i32>,
    libvirt_memory_used_gib: Option<i32>,
    libvirt_pool_used_gib: Option<i32>,
    restic_keep_monthly: Option<i32>,
    restic_keep_yearly: Option<i32>,
    datacenter_type: Option<String>,
    restic_group: Option<String>,
    #[serde_inline_default(String::from("puppetca.innogames.de"))]puppet_ca: String,
    #[serde_inline_default(Some(i32::from(3)))]restic_keep_daily: Option<i32>,
    #[serde_inline_default(Some(i32::from(3)))]restic_keep_last: Option<i32>,
    #[serde_inline_default(Some(i32::from(2)))]restic_keep_weekly: Option<i32>,
    loadbalancer: Option<Vec<String>>,
    arch: Option<String>,
    #[serde_inline_default(bool::from(true))]auto_update: bool,
    #[serde_inline_default(vec![String::from("perc310")])]storage_hardware: Vec<String>,
    #[serde_inline_default(bool::from(true))]backup_disabled: bool,
    #[serde_inline_default(Some(bool::from(false)))]backup_canshrink: Option<bool>,
    backup_maxage: Option<i32>,
    backup_window: Option<Vec<String>>,
    powerports: Option<Vec<String>>,
    switchports: Option<Vec<String>>,
    mac: Option<Vec<String>>,
    #[serde_inline_default(String::from("wheezy"))]os: String,
    igvm_migration_log: Option<Vec<String>>,
    #[serde_inline_default(Some(i32::from(1000)))]cpu_perffactor: Option<i32>,
    backup_timestamp: Option<String>,
    #[serde_inline_default(Some(String::from("aptly.innogames.de")))]repositories_host: Option<String>,
    #[serde_inline_default(String::from("ovs_bond_ab"))]network_link_type: String,
    officetool_id: Option<String>,
    serial_number: String,
    ipv4: String,
    ipv6: String,
    hostname: String,
    servertype: String,
    object_id: String,

}
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, utoipa::ToSchema)] #[schema(as = serveradmin::innogames::de::v1::HypervisorList)] #[serde(rename_all = "snake_case")] pub struct HypervisorList {
    kind: String,
    api_version: String,
    items: Hypervisor,

}
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, utoipa::ToSchema)] #[schema(as = serveradmin::innogames::de::v1::Hypervisor)] #[serde(rename_all = "snake_case")] pub struct Hypervisor {
    kind: String,
    api_version: String,
    metadata: ServerObjectMetadata,
    spec: HypervisorSpec,

}
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, utoipa::ToSchema)]
#[schema(as = serveradmin::innogames::de::v1::ExternalNetworkSpec)]
#[serde_inline_default::serde_inline_default]
#[serde(rename_all = "snake_case")]
pub struct ExternalNetworkSpec {
    acl: Option<Vec<String>>,
    #[serde_inline_default(String::from("online"))]state: String,
    service_groups: Option<Vec<String>>,
    primary_ip6: Option<String>,
    ipv4: String,
    ipv6: String,
    hostname: String,
    servertype: String,
    object_id: String,

}
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, utoipa::ToSchema)] #[schema(as = serveradmin::innogames::de::v1::ExternalNetworkList)] #[serde(rename_all = "snake_case")] pub struct ExternalNetworkList {
    kind: String,
    api_version: String,
    items: ExternalNetwork,

}
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, utoipa::ToSchema)] #[schema(as = serveradmin::innogames::de::v1::ExternalNetwork)] #[serde(rename_all = "snake_case")] pub struct ExternalNetwork {
    kind: String,
    api_version: String,
    metadata: ServerObjectMetadata,
    spec: ExternalNetworkSpec,

}
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, utoipa::ToSchema)]
#[schema(as = serveradmin::innogames::de::v1::LoadbalancerSpec)]
#[serde_inline_default::serde_inline_default]
#[serde(rename_all = "snake_case")]
pub struct LoadbalancerSpec {
    network_type: Option<String>,
    project_network: Option<String>,
    backup_pool_of: Option<Vec<String>>,
    #[serde_inline_default(Some(bool::from(false)))]no_monitoring: Option<bool>,
    #[serde_inline_default(i32::from(15000))]state_limit: i32,
    graphite_graphs: Option<Vec<String>>,
    backup_pool: Option<Vec<String>>,
    primary_ip6: Option<String>,
    allow_from: Option<Vec<String>>,
    #[serde_inline_default(Some(bool::from(false)))]symmetric_nat: Option<bool>,
    #[serde_inline_default(Some(String::from("InnoGames")))]group_company: Option<String>,
    min_nodes: Option<i32>,
    domain: Option<Vec<String>>,
    function: Option<String>,
    protocol_port: Option<Vec<String>>,
    min_nodes_action: Option<String>,
    game_type: Option<String>,
    default_snat: Option<bool>,
    max_nodes: Option<i32>,
    responsible_admin: Option<Vec<String>>,
    acl: Option<Vec<String>>,
    monitoring_queue: Option<String>,
    #[serde_inline_default(String::from("online"))]state: String,
    lb_nodes: Option<Vec<String>>,
    health_checks: Option<Vec<String>>,
    monitoring_checks: Option<Vec<String>>,
    provider_network: Option<String>,
    route_network: Option<String>,
    subproject: Option<String>,
    monitoring_queues: Option<Vec<String>>,
    #[serde_inline_default(String::from("production"))]environment: String,
    datacenter: Option<String>,
    backup_storage: Option<String>,
    service_groups: Option<Vec<String>>,
    project: String,
    project_domain: Option<String>,
    network_zones: Option<Vec<String>>,
    monitoring_groups: Option<Vec<String>>,
    game_market: Option<String>,
    game_world: Option<i32>,
    network_zone: Option<String>,
    nessus_scan_id: Option<i32>,
    ipv4: String,
    ipv6: String,
    hostname: String,
    servertype: String,
    object_id: String,

}
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, utoipa::ToSchema)] #[schema(as = serveradmin::innogames::de::v1::LoadbalancerList)] #[serde(rename_all = "snake_case")] pub struct LoadbalancerList {
    kind: String,
    api_version: String,
    items: Loadbalancer,

}
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, utoipa::ToSchema)] #[schema(as = serveradmin::innogames::de::v1::Loadbalancer)] #[serde(rename_all = "snake_case")] pub struct Loadbalancer {
    kind: String,
    api_version: String,
    metadata: ServerObjectMetadata,
    spec: LoadbalancerSpec,

}
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, utoipa::ToSchema)]
#[schema(as = serveradmin::innogames::de::v1::SwitchSpec)]
#[serde_inline_default::serde_inline_default]
#[serde(rename_all = "snake_case")]
pub struct SwitchSpec {
    datacenter_type: Option<String>,
    provider_network: Option<String>,
    order_date: Option<String>,
    order_id: Option<Vec<String>>,
    #[serde_inline_default(Some(bool::from(false)))]puppet_disabled: Option<bool>,
    route_network: Option<String>,
    primary_ip6: Option<String>,
    vlan_networks: Option<Vec<String>>,
    mac: Option<Vec<String>>,
    powerports: Option<Vec<String>>,
    switchports: Option<Vec<String>>,
    no_monitoring: Option<bool>,
    datacenter: Option<String>,
    project_network: Option<String>,
    dell_servicetag: Option<Vec<String>>,
    sshfp: Option<Vec<String>>,
    rack_height: Option<i32>,
    rack_position: Option<i32>,
    function: Option<String>,
    hardware_model: String,
    bladecenter: Option<String>,
    vlan_interfaces: Option<Vec<String>>,
    bladecenter_slot: Option<Vec<String>>,
    #[serde_inline_default(String::from("production"))]environment: String,
    #[serde_inline_default(vec![String::from("kajetan.staszkiewicz")])]responsible_admin: Vec<String>,
    #[serde_inline_default(String::from("online"))]state: String,
    network_zone: Option<String>,
    rack: String,
    monitoring_queue: Option<String>,
    #[serde_inline_default(String::from("ndco"))]project: String,
    puppet_environment: Option<String>,
    network_zones: Option<Vec<String>>,
    puppet_classes: Option<Vec<String>>,
    puppet_master: Option<String>,
    puppet_ca: Option<String>,
    os: Option<String>,
    graphite_graphs: Option<Vec<String>>,
    repositories: Option<Vec<String>>,
    num_cpu: Option<i32>,
    memory: Option<i32>,
    disk_size_gib: Option<i32>,
    #[serde_inline_default(Some(String::from("aptly.innogames.de")))]repositories_host: Option<String>,
    #[serde_inline_default(Some(vec![String::from("switch_cpu_des,switch_links_des")]))]monitoring_checks: Option<Vec<String>>,
    monitoring_groups: Option<Vec<String>>,
    service_groups: Option<Vec<String>>,
    nessus_scan_id: Option<i32>,
    officetool_id: Option<String>,
    serial_number: String,
    ipv4: String,
    ipv6: String,
    hostname: String,
    servertype: String,
    object_id: String,

}
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, utoipa::ToSchema)] #[schema(as = serveradmin::innogames::de::v1::SwitchList)] #[serde(rename_all = "snake_case")] pub struct SwitchList {
    kind: String,
    api_version: String,
    items: Switch,

}
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, utoipa::ToSchema)] #[schema(as = serveradmin::innogames::de::v1::Switch)] #[serde(rename_all = "snake_case")] pub struct Switch {
    kind: String,
    api_version: String,
    metadata: ServerObjectMetadata,
    spec: SwitchSpec,

}
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, utoipa::ToSchema)]
#[schema(as = serveradmin::innogames::de::v1::DatacenterTypeSpec)]
#[serde_inline_default::serde_inline_default]
#[serde(rename_all = "snake_case")]
pub struct DatacenterTypeSpec {
    hostname: String,
    servertype: String,
    object_id: String,

}
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, utoipa::ToSchema)] #[schema(as = serveradmin::innogames::de::v1::DatacenterTypeList)] #[serde(rename_all = "snake_case")] pub struct DatacenterTypeList {
    kind: String,
    api_version: String,
    items: DatacenterType,

}
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, utoipa::ToSchema)] #[schema(as = serveradmin::innogames::de::v1::DatacenterType)] #[serde(rename_all = "snake_case")] pub struct DatacenterType {
    kind: String,
    api_version: String,
    metadata: ServerObjectMetadata,
    spec: DatacenterTypeSpec,

}
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, utoipa::ToSchema)]
#[schema(as = serveradmin::innogames::de::v1::HwLoadbalancerGroupSpec)]
#[serde_inline_default::serde_inline_default]
#[serde(rename_all = "snake_case")]
pub struct HwLoadbalancerGroupSpec {
    #[serde_inline_default(String::from("online"))]state: String,
    vlan_networks: Option<Vec<String>>,
    public_networks: Option<Vec<String>>,
    subproject: Option<String>,
    project: String,
    #[serde_inline_default(String::from("kernel_bond_ab"))]network_link_type: String,
    hostname: String,
    servertype: String,
    object_id: String,

}
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, utoipa::ToSchema)] #[schema(as = serveradmin::innogames::de::v1::HwLoadbalancerGroupList)] #[serde(rename_all = "snake_case")] pub struct HwLoadbalancerGroupList {
    kind: String,
    api_version: String,
    items: HwLoadbalancerGroup,

}
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, utoipa::ToSchema)] #[schema(as = serveradmin::innogames::de::v1::HwLoadbalancerGroup)] #[serde(rename_all = "snake_case")] pub struct HwLoadbalancerGroup {
    kind: String,
    api_version: String,
    metadata: ServerObjectMetadata,
    spec: HwLoadbalancerGroupSpec,

}
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, utoipa::ToSchema)]
#[schema(as = serveradmin::innogames::de::v1::RouterSpec)]
#[serde_inline_default::serde_inline_default]
#[serde(rename_all = "snake_case")]
pub struct RouterSpec {
    #[serde_inline_default(Some(i32::from(300)))]backup_prio: Option<i32>,
    monitoring_groups: Option<Vec<String>>,
    project_network: Option<String>,
    #[serde_inline_default(bool::from(false))]auto_update: bool,
    #[serde_inline_default(bool::from(true))]backup_disabled: bool,
    #[serde_inline_default(Some(bool::from(false)))]backup_canshrink: Option<bool>,
    backup_maxage: Option<i32>,
    backup_window: Option<Vec<String>>,
    order_date: Option<String>,
    order_id: Option<Vec<String>>,
    #[serde_inline_default(Some(bool::from(false)))]no_monitoring: Option<bool>,
    routed_networks: Option<Vec<String>>,
    datacenter_type: Option<String>,
    primary_ip6: Option<String>,
    os: Option<String>,
    mac: Option<Vec<String>>,
    #[serde_inline_default(Some(bool::from(true)))]puppet_disabled: Option<bool>,
    #[serde_inline_default(vec![String::from("kajetan.staszkiewicz")])]responsible_admin: Vec<String>,
    sshfp: Option<Vec<String>>,
    dell_servicetag: Option<Vec<String>>,
    function: Option<String>,
    route_network: Option<String>,
    vlan_interfaces: Option<Vec<String>>,
    memory: Option<i32>,
    #[serde_inline_default(String::from("production"))]environment: String,
    rack: Option<String>,
    hardware_model: Option<String>,
    #[serde_inline_default(Some(vec![String::from("ping")]))]monitoring_checks: Option<Vec<String>>,
    allow_from: Option<Vec<String>>,
    #[serde_inline_default(String::from("online"))]state: String,
    monitoring_queue: Option<String>,
    datacenter: Option<String>,
    subproject: Option<String>,
    backup_storage: Option<String>,
    #[serde_inline_default(String::from("ndco"))]project: String,
    network_zones: Option<Vec<String>>,
    vlan_networks: Option<Vec<String>>,
    puppet_classes: Option<Vec<String>>,
    puppet_ca: Option<String>,
    num_cpu: Option<i32>,
    disk_size_gib: Option<i32>,
    repositories: Option<Vec<String>>,
    graphite_graphs: Option<Vec<String>>,
    puppet_master: Option<String>,
    puppet_environment: Option<String>,
    network_zone: Option<String>,
    #[serde_inline_default(Some(String::from("aptly.innogames.de")))]repositories_host: Option<String>,
    loadbalancer: Option<Vec<String>>,
    service_groups: Option<Vec<String>>,
    ipv4: String,
    ipv6: String,
    hostname: String,
    servertype: String,
    object_id: String,

}
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, utoipa::ToSchema)] #[schema(as = serveradmin::innogames::de::v1::RouterList)] #[serde(rename_all = "snake_case")] pub struct RouterList {
    kind: String,
    api_version: String,
    items: Router,

}
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, utoipa::ToSchema)] #[schema(as = serveradmin::innogames::de::v1::Router)] #[serde(rename_all = "snake_case")] pub struct Router {
    kind: String,
    api_version: String,
    metadata: ServerObjectMetadata,
    spec: RouterSpec,

}
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, utoipa::ToSchema)]
#[schema(as = serveradmin::innogames::de::v1::EverythingSpec)]
#[serde_inline_default::serde_inline_default]
#[serde(rename_all = "snake_case")]
pub struct EverythingSpec {
    acl: Option<Vec<String>>,
    responsible_admin: Option<Vec<String>>,
    #[serde_inline_default(String::from("online"))]state: String,
    service_groups: Option<Vec<String>>,
    #[serde_inline_default(String::from("acl"))]project: String,
    ipv4: String,
    ipv6: String,
    hostname: String,
    servertype: String,
    object_id: String,

}
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, utoipa::ToSchema)] #[schema(as = serveradmin::innogames::de::v1::EverythingList)] #[serde(rename_all = "snake_case")] pub struct EverythingList {
    kind: String,
    api_version: String,
    items: Everything,

}
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, utoipa::ToSchema)] #[schema(as = serveradmin::innogames::de::v1::Everything)] #[serde(rename_all = "snake_case")] pub struct Everything {
    kind: String,
    api_version: String,
    metadata: ServerObjectMetadata,
    spec: EverythingSpec,

}
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, utoipa::ToSchema)]
#[schema(as = serveradmin::innogames::de::v1::DoorlockSpec)]
#[serde_inline_default::serde_inline_default]
#[serde(rename_all = "snake_case")]
pub struct DoorlockSpec {
    datacenter: Option<String>,
    route_network: Option<String>,
    mac: Option<Vec<String>>,
    #[serde_inline_default(String::from("officeit"))]project: String,
    network_zones: Option<Vec<String>>,
    #[serde_inline_default(String::from("online"))]state: String,
    ipv4: String,
    ipv6: String,
    hostname: String,
    servertype: String,
    object_id: String,

}
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, utoipa::ToSchema)] #[schema(as = serveradmin::innogames::de::v1::DoorlockList)] #[serde(rename_all = "snake_case")] pub struct DoorlockList {
    kind: String,
    api_version: String,
    items: Doorlock,

}
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, utoipa::ToSchema)] #[schema(as = serveradmin::innogames::de::v1::Doorlock)] #[serde(rename_all = "snake_case")] pub struct Doorlock {
    kind: String,
    api_version: String,
    metadata: ServerObjectMetadata,
    spec: DoorlockSpec,

}
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, utoipa::ToSchema)]
#[schema(as = serveradmin::innogames::de::v1::ProviderNetworkSpec)]
#[serde_inline_default::serde_inline_default]
#[serde(rename_all = "snake_case")]
pub struct ProviderNetworkSpec {
    datacenter: Option<String>,
    #[serde_inline_default(Some(String::from("public")))]network_type: Option<String>,
    network_zone: Option<String>,
    network_zones: Option<Vec<String>>,
    #[serde_inline_default(Some(String::from("default.queue")))]monitoring_queue: Option<String>,
    primary_ip6: Option<String>,
    #[serde_inline_default(String::from("online"))]state: String,
    acl: Option<Vec<String>>,
    vlan_tag: Option<i32>,
    default_gateway: Option<String>,
    service_groups: Option<Vec<String>>,
    #[serde_inline_default(String::from("ndco"))]project: String,
    ipv4: String,
    ipv6: String,
    hostname: String,
    servertype: String,
    object_id: String,

}
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, utoipa::ToSchema)] #[schema(as = serveradmin::innogames::de::v1::ProviderNetworkList)] #[serde(rename_all = "snake_case")] pub struct ProviderNetworkList {
    kind: String,
    api_version: String,
    items: ProviderNetwork,

}
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, utoipa::ToSchema)] #[schema(as = serveradmin::innogames::de::v1::ProviderNetwork)] #[serde(rename_all = "snake_case")] pub struct ProviderNetwork {
    kind: String,
    api_version: String,
    metadata: ServerObjectMetadata,
    spec: ProviderNetworkSpec,

}
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, utoipa::ToSchema)]
#[schema(as = serveradmin::innogames::de::v1::MonitoringGroupSpec)]
#[serde_inline_default::serde_inline_default]
#[serde(rename_all = "snake_case")]
pub struct MonitoringGroupSpec {
    #[serde_inline_default(String::from("online"))]state: String,
    monitoring_group_members: Vec<String>,
    responsible_admin: Option<Vec<String>>,
    hostname: String,
    servertype: String,
    object_id: String,

}
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, utoipa::ToSchema)] #[schema(as = serveradmin::innogames::de::v1::MonitoringGroupList)] #[serde(rename_all = "snake_case")] pub struct MonitoringGroupList {
    kind: String,
    api_version: String,
    items: MonitoringGroup,

}
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, utoipa::ToSchema)] #[schema(as = serveradmin::innogames::de::v1::MonitoringGroup)] #[serde(rename_all = "snake_case")] pub struct MonitoringGroup {
    kind: String,
    api_version: String,
    metadata: ServerObjectMetadata,
    spec: MonitoringGroupSpec,

}
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, utoipa::ToSchema)]
#[schema(as = serveradmin::innogames::de::v1::VlanInterfaceSpec)]
#[serde_inline_default::serde_inline_default]
#[serde(rename_all = "snake_case")]
pub struct VlanInterfaceSpec {
    network_type: Option<String>,
    parent_bond: Option<String>,
    primary_ip6: Option<String>,
    no_monitoring: Option<bool>,
    project_network: Option<String>,
    function: Option<String>,
    vlan_interface_of: Option<String>,
    acl: Option<Vec<String>>,
    service_groups: Option<Vec<String>>,
    vlan_tag: Option<i32>,
    route_network: Option<String>,
    #[serde_inline_default(Some(vec![String::from("ping")]))]monitoring_checks: Option<Vec<String>>,
    sshfp: Option<Vec<String>>,
    #[serde_inline_default(String::from("online"))]state: String,
    #[serde_inline_default(String::from("production"))]environment: String,
    monitoring_queue: Option<String>,
    bond_no: Option<String>,
    #[serde_inline_default(String::from("ndco"))]project: String,
    network_zones: Option<Vec<String>>,
    datacenter: Option<String>,
    datacenter_type: Option<String>,
    network_zone: Option<String>,
    ipv4: String,
    ipv6: String,
    hostname: String,
    servertype: String,
    object_id: String,

}
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, utoipa::ToSchema)] #[schema(as = serveradmin::innogames::de::v1::VlanInterfaceList)] #[serde(rename_all = "snake_case")] pub struct VlanInterfaceList {
    kind: String,
    api_version: String,
    items: VlanInterface,

}
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, utoipa::ToSchema)] #[schema(as = serveradmin::innogames::de::v1::VlanInterface)] #[serde(rename_all = "snake_case")] pub struct VlanInterface {
    kind: String,
    api_version: String,
    metadata: ServerObjectMetadata,
    spec: VlanInterfaceSpec,

}
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, utoipa::ToSchema)]
#[schema(as = serveradmin::innogames::de::v1::ExternalDomainSpec)]
#[serde_inline_default::serde_inline_default]
#[serde(rename_all = "snake_case")]
pub struct ExternalDomainSpec {
    acl: Option<Vec<String>>,
    monitoring_checks: Option<Vec<String>>,
    responsible_admin: Option<Vec<String>>,
    subproject: Option<String>,
    service_groups: Option<Vec<String>>,
    #[serde_inline_default(bool::from(false))]no_monitoring: bool,
    project: String,
    #[serde_inline_default(String::from("online"))]state: String,
    hostname: String,
    servertype: String,
    object_id: String,

}
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, utoipa::ToSchema)] #[schema(as = serveradmin::innogames::de::v1::ExternalDomainList)] #[serde(rename_all = "snake_case")] pub struct ExternalDomainList {
    kind: String,
    api_version: String,
    items: ExternalDomain,

}
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, utoipa::ToSchema)] #[schema(as = serveradmin::innogames::de::v1::ExternalDomain)] #[serde(rename_all = "snake_case")] pub struct ExternalDomain {
    kind: String,
    api_version: String,
    metadata: ServerObjectMetadata,
    spec: ExternalDomainSpec,

}
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, utoipa::ToSchema)]
#[schema(as = serveradmin::innogames::de::v1::ProviderSpec)]
#[serde_inline_default::serde_inline_default]
#[serde(rename_all = "snake_case")]
pub struct ProviderSpec {
    #[serde_inline_default(String::from("online"))]state: String,
    provider_asn: Option<i32>,
    hostname: String,
    servertype: String,
    object_id: String,

}
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, utoipa::ToSchema)] #[schema(as = serveradmin::innogames::de::v1::ProviderList)] #[serde(rename_all = "snake_case")] pub struct ProviderList {
    kind: String,
    api_version: String,
    items: Provider,

}
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, utoipa::ToSchema)] #[schema(as = serveradmin::innogames::de::v1::Provider)] #[serde(rename_all = "snake_case")] pub struct Provider {
    kind: String,
    api_version: String,
    metadata: ServerObjectMetadata,
    spec: ProviderSpec,

}
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, utoipa::ToSchema)]
#[schema(as = serveradmin::innogames::de::v1::HealthCheckSpec)]
#[serde_inline_default::serde_inline_default]
#[serde(rename_all = "snake_case")]
pub struct HealthCheckSpec {
    #[serde_inline_default(String::from("ping"))]hc_type: String,
    #[serde_inline_default(i32::from(2))]hc_interval: i32,
    #[serde_inline_default(i32::from(3))]hc_max_failed: i32,
    hc_host: Option<String>,
    #[serde_inline_default(i32::from(1500))]hc_timeout: i32,
    hc_ok_codes: Option<Vec<i32>>,
    subproject: Option<String>,
    hc_port: Option<i32>,
    hc_dbname: Option<String>,
    hc_user: Option<String>,
    hc_query: Option<String>,
    project: String,
    hc_drain_codes: Option<Vec<i32>>,
    hostname: String,
    servertype: String,
    object_id: String,

}
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, utoipa::ToSchema)] #[schema(as = serveradmin::innogames::de::v1::HealthCheckList)] #[serde(rename_all = "snake_case")] pub struct HealthCheckList {
    kind: String,
    api_version: String,
    items: HealthCheck,

}
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, utoipa::ToSchema)] #[schema(as = serveradmin::innogames::de::v1::HealthCheck)] #[serde(rename_all = "snake_case")] pub struct HealthCheck {
    kind: String,
    api_version: String,
    metadata: ServerObjectMetadata,
    spec: HealthCheckSpec,

}
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, utoipa::ToSchema)]
#[schema(as = serveradmin::innogames::de::v1::StorageModuleSpec)]
#[serde_inline_default::serde_inline_default]
#[serde(rename_all = "snake_case")]
pub struct StorageModuleSpec {
    state: String,
    capacity: i32,
    serial_number: String,
    installed_in: Option<String>,
    harddisk_type: String,
    hardware_model: String,
    hostname: String,
    servertype: String,
    object_id: String,

}
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, utoipa::ToSchema)] #[schema(as = serveradmin::innogames::de::v1::StorageModuleList)] #[serde(rename_all = "snake_case")] pub struct StorageModuleList {
    kind: String,
    api_version: String,
    items: StorageModule,

}
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, utoipa::ToSchema)] #[schema(as = serveradmin::innogames::de::v1::StorageModule)] #[serde(rename_all = "snake_case")] pub struct StorageModule {
    kind: String,
    api_version: String,
    metadata: ServerObjectMetadata,
    spec: StorageModuleSpec,

}
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, utoipa::ToSchema)]
#[schema(as = serveradmin::innogames::de::v1::BladeEnclosureSpec)]
#[serde_inline_default::serde_inline_default]
#[serde(rename_all = "snake_case")]
pub struct BladeEnclosureSpec {
    #[serde_inline_default(Some(bool::from(false)))]backup_canshrink: Option<bool>,
    backup_maxage: Option<i32>,
    backup_window: Option<Vec<String>>,
    #[serde_inline_default(Some(i32::from(300)))]backup_prio: Option<i32>,
    rack_height: Option<i32>,
    rack_position: Option<i32>,
    order_id: Option<Vec<String>>,
    #[serde_inline_default(bool::from(false))]auto_update: bool,
    #[serde_inline_default(bool::from(true))]backup_disabled: bool,
    order_date: Option<String>,
    no_monitoring: Option<bool>,
    primary_ip6: Option<String>,
    dell_servicetag: Option<Vec<String>>,
    route_network: Option<String>,
    monitoring_groups: Option<Vec<String>>,
    #[serde_inline_default(Some(vec![String::from("ping")]))]monitoring_checks: Option<Vec<String>>,
    function: Option<String>,
    hardware_model: Option<String>,
    sshfp: Option<Vec<String>>,
    #[serde_inline_default(vec![String::from("kajetan.staszkiewicz")])]responsible_admin: Vec<String>,
    #[serde_inline_default(String::from("online"))]state: String,
    datacenter: Option<String>,
    rack: Option<String>,
    project_network: Option<String>,
    #[serde_inline_default(String::from("production"))]environment: String,
    monitoring_queue: Option<String>,
    backup_storage: Option<String>,
    #[serde_inline_default(String::from("ndco"))]project: String,
    network_zones: Option<Vec<String>>,
    firmware_versions: Option<Vec<String>>,
    mac: Option<Vec<String>>,
    #[serde_inline_default(String::from("iDRAC6"))]os: String,
    powerports: Option<Vec<String>>,
    ipv4: String,
    ipv6: String,
    hostname: String,
    servertype: String,
    object_id: String,

}
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, utoipa::ToSchema)] #[schema(as = serveradmin::innogames::de::v1::BladeEnclosureList)] #[serde(rename_all = "snake_case")] pub struct BladeEnclosureList {
    kind: String,
    api_version: String,
    items: BladeEnclosure,

}
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, utoipa::ToSchema)] #[schema(as = serveradmin::innogames::de::v1::BladeEnclosure)] #[serde(rename_all = "snake_case")] pub struct BladeEnclosure {
    kind: String,
    api_version: String,
    metadata: ServerObjectMetadata,
    spec: BladeEnclosureSpec,

}
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, utoipa::ToSchema)]
#[schema(as = serveradmin::innogames::de::v1::TunnelInterfaceSpec)]
#[serde_inline_default::serde_inline_default]
#[serde(rename_all = "snake_case")]
pub struct TunnelInterfaceSpec {
    #[serde_inline_default(String::from("online"))]state: String,
    primary_ip6: Option<String>,
    #[serde_inline_default(String::from("ndco"))]project: String,
    monitoring_queue: Option<String>,
    network_type: Option<String>,
    network_zone: Option<String>,
    network_zones: Option<Vec<String>>,
    service_groups: Option<Vec<String>>,
    tunnel_interface_of: Option<String>,
    route_network: Option<String>,
    ipv4: String,
    ipv6: String,
    hostname: String,
    servertype: String,
    object_id: String,

}
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, utoipa::ToSchema)] #[schema(as = serveradmin::innogames::de::v1::TunnelInterfaceList)] #[serde(rename_all = "snake_case")] pub struct TunnelInterfaceList {
    kind: String,
    api_version: String,
    items: TunnelInterface,

}
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, utoipa::ToSchema)] #[schema(as = serveradmin::innogames::de::v1::TunnelInterface)] #[serde(rename_all = "snake_case")] pub struct TunnelInterface {
    kind: String,
    api_version: String,
    metadata: ServerObjectMetadata,
    spec: TunnelInterfaceSpec,

}
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, utoipa::ToSchema)]
#[schema(as = serveradmin::innogames::de::v1::RouteNetworkSpec)]
#[serde_inline_default::serde_inline_default]
#[serde(rename_all = "snake_case")]
pub struct RouteNetworkSpec {
    parent_bond: Option<String>,
    datacenter: String,
    public_networks: Option<Vec<String>>,
    network_zone: Option<String>,
    provider: Option<String>,
    table_priority: Option<i32>,
    internal_gateway_vhid: Option<i32>,
    network_zones: Option<Vec<String>>,
    default_gateway: Option<String>,
    vlan_tag: Option<i32>,
    primary_ip6: Option<String>,
    #[serde_inline_default(String::from("online"))]state: String,
    acl: Option<Vec<String>>,
    routing_table: Option<i32>,
    #[serde_inline_default(String::from("internal"))]network_type: String,
    provider_network: Option<String>,
    assigned_to: String,
    internal_gateway: Option<String>,
    bond_no: Option<String>,
    service_groups: Option<Vec<String>>,
    #[serde_inline_default(String::from("ndco"))]project: String,
    datacenter_type: Option<String>,
    aws_vpc_id: Option<String>,
    #[serde_inline_default(String::from("default.queue"))]monitoring_queue: String,
    ospf_active: Option<bool>,
    snat_lb_pool: Option<String>,
    network_autoconfiguration: Option<String>,
    ipv4: String,
    ipv6: String,
    hostname: String,
    servertype: String,
    object_id: String,

}
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, utoipa::ToSchema)] #[schema(as = serveradmin::innogames::de::v1::RouteNetworkList)] #[serde(rename_all = "snake_case")] pub struct RouteNetworkList {
    kind: String,
    api_version: String,
    items: RouteNetwork,

}
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, utoipa::ToSchema)] #[schema(as = serveradmin::innogames::de::v1::RouteNetwork)] #[serde(rename_all = "snake_case")] pub struct RouteNetwork {
    kind: String,
    api_version: String,
    metadata: ServerObjectMetadata,
    spec: RouteNetworkSpec,

}
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, utoipa::ToSchema)]
#[schema(as = serveradmin::innogames::de::v1::DatacenterSpec)]
#[serde_inline_default::serde_inline_default]
#[serde(rename_all = "snake_case")]
pub struct DatacenterSpec {
    no_monitoring: Option<bool>,
    datacenter_colo: Option<String>,
    responsible_admin: Option<Vec<String>>,
    #[serde_inline_default(String::from("online"))]state: String,
    #[serde_inline_default(String::from("ndco"))]project: String,
    hostname: String,
    servertype: String,
    object_id: String,

}
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, utoipa::ToSchema)] #[schema(as = serveradmin::innogames::de::v1::DatacenterList)] #[serde(rename_all = "snake_case")] pub struct DatacenterList {
    kind: String,
    api_version: String,
    items: Datacenter,

}
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, utoipa::ToSchema)] #[schema(as = serveradmin::innogames::de::v1::Datacenter)] #[serde(rename_all = "snake_case")] pub struct Datacenter {
    kind: String,
    api_version: String,
    metadata: ServerObjectMetadata,
    spec: DatacenterSpec,

}
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, utoipa::ToSchema)]
#[schema(as = serveradmin::innogames::de::v1::MonitoringQueueSpec)]
#[serde_inline_default::serde_inline_default]
#[serde(rename_all = "snake_case")]
pub struct MonitoringQueueSpec {
    #[serde_inline_default(String::from("admin"))]project: String,
    #[serde_inline_default(String::from("monitoring"))]subproject: String,
    monitoring_workers: Option<Vec<String>>,
    monitoring_networks: Option<Vec<String>>,
    hostname: String,
    servertype: String,
    object_id: String,

}
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, utoipa::ToSchema)] #[schema(as = serveradmin::innogames::de::v1::MonitoringQueueList)] #[serde(rename_all = "snake_case")] pub struct MonitoringQueueList {
    kind: String,
    api_version: String,
    items: MonitoringQueue,

}
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, utoipa::ToSchema)] #[schema(as = serveradmin::innogames::de::v1::MonitoringQueue)] #[serde(rename_all = "snake_case")] pub struct MonitoringQueue {
    kind: String,
    api_version: String,
    metadata: ServerObjectMetadata,
    spec: MonitoringQueueSpec,

}
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, utoipa::ToSchema)]
#[schema(as = serveradmin::innogames::de::v1::NatGatewaySpec)]
#[serde_inline_default::serde_inline_default]
#[serde(rename_all = "snake_case")]
pub struct NatGatewaySpec {
    network_type: Option<String>,
    datacenter_type: Option<String>,
    primary_ip6: Option<String>,
    #[serde_inline_default(Some(bool::from(false)))]default_snat: Option<bool>,
    #[serde_inline_default(Some(vec![String::from("ping")]))]monitoring_checks: Option<Vec<String>>,
    function: Option<String>,
    project_network: Option<String>,
    vrrp_id: Option<i32>,
    #[serde_inline_default(String::from("production"))]environment: String,
    route_network: Option<String>,
    responsible_admin: Vec<String>,
    acl: Option<Vec<String>>,
    #[serde_inline_default(Some(bool::from(false)))]no_monitoring: Option<bool>,
    monitoring_queue: Option<String>,
    gateway_type: Option<String>,
    #[serde_inline_default(String::from("online"))]state: String,
    datacenter: Option<String>,
    service_groups: Option<Vec<String>>,
    #[serde_inline_default(String::from("ndco"))]project: String,
    network_zones: Option<Vec<String>>,
    network_zone: Option<String>,
    monitoring_groups: Option<Vec<String>>,
    ipv4: String,
    ipv6: String,
    hostname: String,
    servertype: String,
    object_id: String,

}
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, utoipa::ToSchema)] #[schema(as = serveradmin::innogames::de::v1::NatGatewayList)] #[serde(rename_all = "snake_case")] pub struct NatGatewayList {
    kind: String,
    api_version: String,
    items: NatGateway,

}
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, utoipa::ToSchema)] #[schema(as = serveradmin::innogames::de::v1::NatGateway)] #[serde(rename_all = "snake_case")] pub struct NatGateway {
    kind: String,
    api_version: String,
    metadata: ServerObjectMetadata,
    spec: NatGatewaySpec,

}
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, utoipa::ToSchema)]
#[schema(as = serveradmin::innogames::de::v1::MediaDeviceSpec)]
#[serde_inline_default::serde_inline_default]
#[serde(rename_all = "snake_case")]
pub struct MediaDeviceSpec {
    primary_ip6: Option<String>,
    #[serde_inline_default(String::from("online"))]state: String,
    hardware_model: String,
    hardware_type: String,
    responsible_admin: Vec<String>,
    responsible_external: Option<Vec<String>>,
    access_uri: Option<Vec<String>>,
    serial_number: Option<String>,
    responsible_internal: Vec<String>,
    #[serde_inline_default(String::from("officeit"))]project: String,
    allow_from: Option<Vec<String>>,
    route_network: Option<String>,
    network_zones: Option<Vec<String>>,
    #[serde_inline_default(vec![String::from("direct")])]powerports: Vec<String>,
    officetool_id: Option<String>,
    ipv4: String,
    ipv6: String,
    hostname: String,
    servertype: String,
    object_id: String,

}
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, utoipa::ToSchema)] #[schema(as = serveradmin::innogames::de::v1::MediaDeviceList)] #[serde(rename_all = "snake_case")] pub struct MediaDeviceList {
    kind: String,
    api_version: String,
    items: MediaDevice,

}
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, utoipa::ToSchema)] #[schema(as = serveradmin::innogames::de::v1::MediaDevice)] #[serde(rename_all = "snake_case")] pub struct MediaDevice {
    kind: String,
    api_version: String,
    metadata: ServerObjectMetadata,
    spec: MediaDeviceSpec,

}
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, utoipa::ToSchema)]
#[schema(as = serveradmin::innogames::de::v1::ProjectSpec)]
#[serde_inline_default::serde_inline_default]
#[serde(rename_all = "snake_case")]
pub struct ProjectSpec {
    responsible_admin: Vec<String>,
    event_tracking_code: Option<String>,
    #[serde_inline_default(bool::from(false))]is_game: bool,
    jira_project: Option<String>,
    project_english_name: Option<String>,
    short_code: Option<String>,
    slack_channel: Option<String>,
    support_code: Option<String>,
    sentry_team: Option<String>,
    hostname: String,
    servertype: String,
    object_id: String,

}
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, utoipa::ToSchema)] #[schema(as = serveradmin::innogames::de::v1::ProjectList)] #[serde(rename_all = "snake_case")] pub struct ProjectList {
    kind: String,
    api_version: String,
    items: Project,

}
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, utoipa::ToSchema)] #[schema(as = serveradmin::innogames::de::v1::Project)] #[serde(rename_all = "snake_case")] pub struct Project {
    kind: String,
    api_version: String,
    metadata: ServerObjectMetadata,
    spec: ProjectSpec,

}
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, utoipa::ToSchema)]
#[schema(as = serveradmin::innogames::de::v1::NatboxSpec)]
#[serde_inline_default::serde_inline_default]
#[serde(rename_all = "snake_case")]
pub struct NatboxSpec {
    service_groups: Option<Vec<String>>,
    vlan_networks: Option<Vec<String>>,
    #[serde_inline_default(String::from("puppetca.innogames.de"))]puppet_ca: String,
    puppet_master: String,
    #[serde_inline_default(String::from("amd64"))]arch: String,
    order_date: Option<String>,
    order_id: Option<Vec<String>>,
    #[serde_inline_default(Some(bool::from(false)))]puppet_disabled: Option<bool>,
    primary_ip6: Option<String>,
    no_monitoring: Option<bool>,
    route_network: Option<String>,
    graphite_graphs: Option<Vec<String>>,
    backup_window: Option<Vec<String>>,
    backup_prio: Option<i32>,
    powerports: Option<Vec<String>>,
    switchports: Option<Vec<String>>,
    mac: Option<Vec<String>>,
    routed_networks: Option<Vec<String>>,
    num_cpu: Option<i32>,
    dell_servicetag: Option<Vec<String>>,
    repositories: Option<Vec<String>>,
    ipmi: Option<String>,
    function: Option<String>,
    cpu_model: Option<String>,
    cpu_clock_mhz: Option<i32>,
    puppet_environment: Option<String>,
    memory: Option<i32>,
    network_zones: Option<Vec<String>>,
    hardware_model: Option<String>,
    #[serde_inline_default(String::from("production"))]environment: String,
    memory_modules: Option<Vec<String>>,
    disk_size_gib: Option<i32>,
    rack: Option<String>,
    #[serde_inline_default(vec![String::from("kajetan.staszkiewicz")])]responsible_admin: Vec<String>,
    harddisk_type: Option<String>,
    #[serde_inline_default(Some(vec![String::from("ping")]))]monitoring_checks: Option<Vec<String>>,
    public_networks: Option<Vec<String>>,
    sshfp: Option<Vec<String>>,
    vlan_interfaces: Option<Vec<String>>,
    puppet_classes: Option<Vec<String>>,
    #[serde_inline_default(String::from("online"))]state: String,
    #[serde_inline_default(bool::from(true))]backup_disabled: bool,
    #[serde_inline_default(Some(bool::from(false)))]backup_canshrink: Option<bool>,
    network_zone: Option<String>,
    datacenter: Option<String>,
    monitoring_queue: Option<String>,
    firmware_versions: Option<Vec<String>>,
    subproject: Option<String>,
    backup_storage: Option<String>,
    dhcp_networks: Option<Vec<String>>,
    hw_loadbalancer_group: Option<String>,
    #[serde_inline_default(String::from("ndco"))]project: String,
    storage_hardware: Option<Vec<String>>,
    monitoring_groups: Option<Vec<String>>,
    #[serde_inline_default(bool::from(false))]auto_update: bool,
    backup_maxage: Option<i32>,
    #[serde_inline_default(String::from("jessie"))]os: String,
    #[serde_inline_default(Some(String::from("aptly.innogames.de")))]repositories_host: Option<String>,
    officetool_id: Option<String>,
    ipv4: String,
    ipv6: String,
    hostname: String,
    servertype: String,
    object_id: String,

}
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, utoipa::ToSchema)] #[schema(as = serveradmin::innogames::de::v1::NatboxList)] #[serde(rename_all = "snake_case")] pub struct NatboxList {
    kind: String,
    api_version: String,
    items: Natbox,

}
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, utoipa::ToSchema)] #[schema(as = serveradmin::innogames::de::v1::Natbox)] #[serde(rename_all = "snake_case")] pub struct Natbox {
    kind: String,
    api_version: String,
    metadata: ServerObjectMetadata,
    spec: NatboxSpec,

}
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, utoipa::ToSchema)]
#[schema(as = serveradmin::innogames::de::v1::AclSpec)]
#[serde_inline_default::serde_inline_default]
#[serde(rename_all = "snake_case")]
pub struct AclSpec {
    acl_members: Option<Vec<String>>,
    #[serde_inline_default(String::from("production"))]environment: String,
    function: Option<String>,
    protocol_port: Option<Vec<String>>,
    #[serde_inline_default(Some(bool::from(false)))]no_logging: Option<bool>,
    subproject: Option<String>,
    #[serde_inline_default(Some(bool::from(false)))]no_monitoring: Option<bool>,
    #[serde_inline_default(String::from("online"))]state: String,
    responsible_admin: Vec<String>,
    project: String,
    allow_from_reverse: Option<Vec<String>>,
    allow_to_reverse: Option<Vec<String>>,
    hostname: String,
    servertype: String,
    object_id: String,

}
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, utoipa::ToSchema)] #[schema(as = serveradmin::innogames::de::v1::AclList)] #[serde(rename_all = "snake_case")] pub struct AclList {
    kind: String,
    api_version: String,
    items: Acl,

}
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, utoipa::ToSchema)] #[schema(as = serveradmin::innogames::de::v1::Acl)] #[serde(rename_all = "snake_case")] pub struct Acl {
    kind: String,
    api_version: String,
    metadata: ServerObjectMetadata,
    spec: AclSpec,

}
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, utoipa::ToSchema)]
#[schema(as = serveradmin::innogames::de::v1::RoomTabletSpec)]
#[serde_inline_default::serde_inline_default]
#[serde(rename_all = "snake_case")]
pub struct RoomTabletSpec {
    provider_network: Option<String>,
    order_date: Option<String>,
    order_id: Option<Vec<String>>,
    hardware_model: Option<String>,
    route_network: Option<String>,
    mac: Vec<String>,
    state: String,
    #[serde_inline_default(String::from("officeit"))]project: String,
    primary_ip6: Option<String>,
    network_zones: Option<Vec<String>>,
    officetool_id: Option<String>,
    ipv4: String,
    ipv6: String,
    hostname: String,
    servertype: String,
    object_id: String,

}
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, utoipa::ToSchema)] #[schema(as = serveradmin::innogames::de::v1::RoomTabletList)] #[serde(rename_all = "snake_case")] pub struct RoomTabletList {
    kind: String,
    api_version: String,
    items: RoomTablet,

}
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, utoipa::ToSchema)] #[schema(as = serveradmin::innogames::de::v1::RoomTablet)] #[serde(rename_all = "snake_case")] pub struct RoomTablet {
    kind: String,
    api_version: String,
    metadata: ServerObjectMetadata,
    spec: RoomTabletSpec,

}
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, utoipa::ToSchema)]
#[schema(as = serveradmin::innogames::de::v1::AzureNetworkSpec)]
#[serde_inline_default::serde_inline_default]
#[serde(rename_all = "snake_case")]
pub struct AzureNetworkSpec {
    ipv4: String,
    ipv6: String,
    hostname: String,
    servertype: String,
    object_id: String,

}
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, utoipa::ToSchema)] #[schema(as = serveradmin::innogames::de::v1::AzureNetworkList)] #[serde(rename_all = "snake_case")] pub struct AzureNetworkList {
    kind: String,
    api_version: String,
    items: AzureNetwork,

}
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, utoipa::ToSchema)] #[schema(as = serveradmin::innogames::de::v1::AzureNetwork)] #[serde(rename_all = "snake_case")] pub struct AzureNetwork {
    kind: String,
    api_version: String,
    metadata: ServerObjectMetadata,
    spec: AzureNetworkSpec,

}
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, utoipa::ToSchema)]
#[schema(as = serveradmin::innogames::de::v1::PublicDomainSpec)]
#[serde_inline_default::serde_inline_default]
#[serde(rename_all = "snake_case")]
pub struct PublicDomainSpec {
    project: String,
    monitoring_checks: Option<Vec<String>>,
    no_monitoring: Option<bool>,
    subproject: Option<String>,
    responsible_admin: Option<Vec<String>>,
    service_groups: Option<Vec<String>>,
    letsencrypt_enabled: Option<bool>,
    dns_soa: Option<String>,
    dns_ns: Option<Vec<String>>,
    dns_txt: Option<Vec<String>>,
    dns_cname: Option<String>,
    loadbalancer_nodes: Option<Vec<String>>,
    dns_mx: Option<Vec<String>>,
    dns_spf: Option<String>,
    dns_srv: Option<Vec<String>>,
    hostname: String,
    servertype: String,
    object_id: String,

}
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, utoipa::ToSchema)] #[schema(as = serveradmin::innogames::de::v1::PublicDomainList)] #[serde(rename_all = "snake_case")] pub struct PublicDomainList {
    kind: String,
    api_version: String,
    items: PublicDomain,

}
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, utoipa::ToSchema)] #[schema(as = serveradmin::innogames::de::v1::PublicDomain)] #[serde(rename_all = "snake_case")] pub struct PublicDomain {
    kind: String,
    api_version: String,
    metadata: ServerObjectMetadata,
    spec: PublicDomainSpec,

}
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, utoipa::ToSchema)]
#[schema(as = serveradmin::innogames::de::v1::HwInternalrouterSpec)]
#[serde_inline_default::serde_inline_default]
#[serde(rename_all = "snake_case")]
pub struct HwInternalrouterSpec {
    service_groups: Option<Vec<String>>,
    order_date: Option<String>,
    order_id: Option<Vec<String>>,
    #[serde_inline_default(Some(bool::from(false)))]puppet_disabled: Option<bool>,
    graphite_graphs: Option<Vec<String>>,
    project_network: Option<String>,
    primary_ip6: Option<String>,
    dell_servicetag: Option<Vec<String>>,
    repositories: Option<Vec<String>>,
    #[serde_inline_default(String::from("amd64"))]arch: String,
    provisioning_step: Option<String>,
    #[serde_inline_default(String::from("puppetca.innogames.de"))]puppet_ca: String,
    puppet_master: String,
    storage_hardware: Option<Vec<String>>,
    disk_size_gib: Option<i32>,
    memory: Option<i32>,
    num_cpu: Option<i32>,
    cpu_clock_mhz: Option<i32>,
    ipmi: Option<String>,
    function: Option<String>,
    bladecenter: Option<String>,
    puppet_environment: Option<String>,
    no_monitoring: Option<bool>,
    hardware_model: Option<String>,
    harddisk_type: Option<String>,
    cpu_model: Option<String>,
    rack_height: Option<i32>,
    rack_position: Option<i32>,
    hardware_components: Option<Vec<String>>,
    memory_modules: Option<Vec<String>>,
    bladecenter_slot: Option<Vec<String>>,
    datacenter: Option<String>,
    rack: Option<String>,
    #[serde_inline_default(Some(vec![String::from("kajetan.staszkiewicz")]))]responsible_admin: Option<Vec<String>>,
    route_network: Option<String>,
    #[serde_inline_default(Some(vec![String::from("ping")]))]monitoring_checks: Option<Vec<String>>,
    sshfp: Option<Vec<String>>,
    puppet_classes: Option<Vec<String>>,
    vlan_interfaces: Option<Vec<String>>,
    vlan_networks: Option<Vec<String>>,
    #[serde_inline_default(String::from("online"))]state: String,
    #[serde_inline_default(String::from("production"))]environment: String,
    monitoring_queue: Option<String>,
    network_zones: Option<Vec<String>>,
    network_zone: Option<String>,
    subproject: Option<String>,
    backup_storage: Option<String>,
    firmware_versions: Option<Vec<String>>,
    #[serde_inline_default(String::from("ndco"))]project: String,
    #[serde_inline_default(bool::from(false))]auto_update: bool,
    #[serde_inline_default(bool::from(true))]backup_disabled: bool,
    #[serde_inline_default(Some(bool::from(false)))]backup_canshrink: Option<bool>,
    backup_maxage: Option<i32>,
    backup_window: Option<Vec<String>>,
    backup_prio: Option<i32>,
    powerports: Option<Vec<String>>,
    switchports: Option<Vec<String>>,
    mac: Option<Vec<String>>,
    #[serde_inline_default(String::from("freebsd10"))]os: String,
    #[serde_inline_default(Some(String::from("aptly.innogames.de")))]repositories_host: Option<String>,
    #[serde_inline_default(String::from("kernel_bond_ab"))]network_link_type: String,
    monitoring_groups: Option<Vec<String>>,
    tunnel_interfaces: Option<Vec<String>>,
    ipv4: String,
    ipv6: String,
    hostname: String,
    servertype: String,
    object_id: String,

}
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, utoipa::ToSchema)] #[schema(as = serveradmin::innogames::de::v1::HwInternalrouterList)] #[serde(rename_all = "snake_case")] pub struct HwInternalrouterList {
    kind: String,
    api_version: String,
    items: HwInternalrouter,

}
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, utoipa::ToSchema)] #[schema(as = serveradmin::innogames::de::v1::HwInternalrouter)] #[serde(rename_all = "snake_case")] pub struct HwInternalrouter {
    kind: String,
    api_version: String,
    metadata: ServerObjectMetadata,
    spec: HwInternalrouterSpec,

}
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, utoipa::ToSchema)]
#[schema(as = serveradmin::innogames::de::v1::ProjectNetworkSpec)]
#[serde_inline_default::serde_inline_default]
#[serde(rename_all = "snake_case")]
pub struct ProjectNetworkSpec {
    function: Option<String>,
    datacenter: Option<String>,
    subproject: Option<String>,
    provider_network: Option<String>,
    default_gateway: Option<String>,
    primary_ip6: Option<String>,
    internal_gateway: Option<String>,
    acl: Option<Vec<String>>,
    service_groups: Option<Vec<String>>,
    public_networks: Option<Vec<String>>,
    route_network: Option<String>,
    game_market: Option<String>,
    allow_from: Option<Vec<String>>,
    network_type: Option<String>,
    monitoring_queue: Option<String>,
    #[serde_inline_default(String::from("online"))]state: String,
    #[serde_inline_default(String::from("production"))]environment: String,
    project: String,
    network_zones: Option<Vec<String>>,
    aws_placement: Option<Vec<String>>,
    aws_subnet_id: Option<String>,
    aws_security_group_ids: Option<Vec<String>>,
    datacenter_type: Option<String>,
    aws_vpc_id: Option<String>,
    nessus_folder_name: Option<String>,
    nessus_scan_id: Option<i32>,
    ipv4: String,
    ipv6: String,
    hostname: String,
    servertype: String,
    object_id: String,

}
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, utoipa::ToSchema)] #[schema(as = serveradmin::innogames::de::v1::ProjectNetworkList)] #[serde(rename_all = "snake_case")] pub struct ProjectNetworkList {
    kind: String,
    api_version: String,
    items: ProjectNetwork,

}
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, utoipa::ToSchema)] #[schema(as = serveradmin::innogames::de::v1::ProjectNetwork)] #[serde(rename_all = "snake_case")] pub struct ProjectNetwork {
    kind: String,
    api_version: String,
    metadata: ServerObjectMetadata,
    spec: ProjectNetworkSpec,

}
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, utoipa::ToSchema)]
#[schema(as = serveradmin::innogames::de::v1::VmSpec)]
#[serde_inline_default::serde_inline_default]
#[serde(rename_all = "snake_case")]
pub struct VmSpec {
    #[serde_inline_default(Some(String::from("InnoGames")))]group_company: Option<String>,
    monitoring_satellite_of: Option<String>,
    mac: Option<Vec<String>>,
    game_market: Option<String>,
    routed_networks: Option<Vec<String>>,
    monitoring_queues: Option<Vec<String>>,
    ipv4_only: Option<bool>,
    #[serde_inline_default(Some(bool::from(false)))]auto_update: Option<bool>,
    game_world: Option<i32>,
    #[serde_inline_default(Some(bool::from(false)))]backup_disabled: Option<bool>,
    #[serde_inline_default(Some(bool::from(false)))]backup_canshrink: Option<bool>,
    #[serde_inline_default(Some(String::from("buster")))]os: Option<String>,
    #[serde_inline_default(Some(i32::from(300)))]backup_prio: Option<i32>,
    #[serde_inline_default(i32::from(4))]num_cpu: i32,
    #[serde_inline_default(Some(bool::from(false)))]no_monitoring: Option<bool>,
    #[serde_inline_default(Some(bool::from(false)))]puppet_disabled: Option<bool>,
    primary_ip6: Option<String>,
    #[serde_inline_default(i32::from(2048))]memory: i32,
    #[serde_inline_default(i32::from(6))]disk_size_gib: i32,
    security_availability: Option<String>,
    #[serde_inline_default(String::from("puppetca.innogames.de"))]puppet_ca: String,
    puppet_master: String,
    backup_state: Option<String>,
    provider_network: Option<String>,
    route_network: Option<String>,
    graphite_graphs: Option<Vec<String>>,
    served_markets: Option<Vec<String>>,
    loadbalancer: Option<Vec<String>>,
    datacenter: Option<String>,
    igvm_locked: Option<String>,
    #[serde_inline_default(Some(vec![String::from("int:innogames:stable")]))]repositories: Option<Vec<String>>,
    security_confidentiality: Option<String>,
    fixup_enabled: Option<bool>,
    security_integrity: Option<String>,
    #[serde_inline_default(String::from("production"))]environment: String,
    puppet_environment: Option<String>,
    function: Option<String>,
    os_product_license: Option<String>,
    #[serde_inline_default(Some(vec![String::from("2-5")]))]backup_window: Option<Vec<String>>,
    vlan_interfaces: Option<Vec<String>>,
    allow_to: Option<Vec<String>>,
    bladecenter: Option<String>,
    bladecenter_slot: Option<Vec<String>>,
    game_type: Option<String>,
    legal_personal_data: Option<String>,
    served_game: Option<String>,
    project_network: Option<String>,
    responsible_admin: Vec<String>,
    hypervisor: Option<String>,
    hypervisor_standby: Option<String>,
    subproject: Option<String>,
    sshfp: Option<Vec<String>>,
    barman_host: Option<String>,
    #[serde_inline_default(Some(vec![String::from("ping")]))]monitoring_checks: Option<Vec<String>>,
    puppet_classes: Option<Vec<String>>,
    allow_from: Option<Vec<String>>,
    acl: Option<Vec<String>>,
    description: Option<String>,
    project_domain: Option<String>,
    #[serde_inline_default(Some(bool::from(false)))]iggop_locked: Option<bool>,
    instance: Option<i32>,
    #[serde_inline_default(String::from("online"))]state: String,
    aws_subnet_id: Option<String>,
    network_zone: Option<String>,
    monitoring_queue: Option<String>,
    #[serde_inline_default(Some(String::from("secondary")))]lb_multihoming: Option<String>,
    network_zones: Option<Vec<String>>,
    backup_storage: Option<String>,
    service_groups: Option<Vec<String>>,
    #[serde_inline_default(String::from("default"))]io_weight: String,
    project: String,
    aws_placement: Option<Vec<String>>,
    aws_image_id: Option<String>,
    aws_instance_id: Option<String>,
    aws_instance_type: Option<String>,
    aws_key_name: Option<String>,
    datacenter_type: Option<String>,
    aws_security_group_ids: Option<Vec<String>>,
    restic_keep_monthly: Option<i32>,
    restic_keep_yearly: Option<i32>,
    aws_vpc_id: Option<String>,
    restic_group: Option<String>,
    vlan_networks: Option<Vec<String>>,
    #[serde_inline_default(Some(i32::from(3)))]restic_keep_daily: Option<i32>,
    #[serde_inline_default(Some(i32::from(3)))]restic_keep_last: Option<i32>,
    #[serde_inline_default(Some(i32::from(2)))]restic_keep_weekly: Option<i32>,
    #[serde_inline_default(Some(i32::from(0)))]load_99: Option<i32>,
    igvm_action: Option<String>,
    #[serde_inline_default(Some(bool::from(false)))]powa_enabled: Option<bool>,
    #[serde_inline_default(Some(i32::from(24)))]backup_maxage: Option<i32>,
    lb_pool: Option<Vec<String>>,
    rack: Option<String>,
    backup_timestamp: Option<String>,
    #[serde_inline_default(Some(String::from("aptly.innogames.de")))]repositories_host: Option<String>,
    monitoring_groups: Option<Vec<String>>,
    internal_domain: Option<String>,
    nessus_scan_id: Option<i32>,
    ipv4: String,
    ipv6: String,
    hostname: String,
    servertype: String,
    object_id: String,

}
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, utoipa::ToSchema)] #[schema(as = serveradmin::innogames::de::v1::VmList)] #[serde(rename_all = "snake_case")] pub struct VmList {
    kind: String,
    api_version: String,
    items: Vm,

}
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, utoipa::ToSchema)] #[schema(as = serveradmin::innogames::de::v1::Vm)] #[serde(rename_all = "snake_case")] pub struct Vm {
    kind: String,
    api_version: String,
    metadata: ServerObjectMetadata,
    spec: VmSpec,

}
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, utoipa::ToSchema)]
#[schema(as = serveradmin::innogames::de::v1::AccesspointSpec)]
#[serde_inline_default::serde_inline_default]
#[serde(rename_all = "snake_case")]
pub struct AccesspointSpec {
    mac: Option<Vec<String>>,
    #[serde_inline_default(String::from("online"))]state: String,
    switchports: Option<Vec<String>>,
    dell_servicetag: Option<Vec<String>>,
    primary_ip6: Option<String>,
    hardware_model: Option<String>,
    vlan_networks: Option<Vec<String>>,
    route_network: Option<String>,
    sshfp: Option<Vec<String>>,
    monitoring_groups: Option<Vec<String>>,
    provider_network: Option<String>,
    responsible_admin: Option<Vec<String>>,
    monitoring_queue: Option<String>,
    datacenter: Option<String>,
    description: Option<String>,
    #[serde_inline_default(String::from("officeit"))]project: String,
    network_zones: Option<Vec<String>>,
    officetool_id: Option<String>,
    ipv4: String,
    ipv6: String,
    hostname: String,
    servertype: String,
    object_id: String,

}
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, utoipa::ToSchema)] #[schema(as = serveradmin::innogames::de::v1::AccesspointList)] #[serde(rename_all = "snake_case")] pub struct AccesspointList {
    kind: String,
    api_version: String,
    items: Accesspoint,

}
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, utoipa::ToSchema)] #[schema(as = serveradmin::innogames::de::v1::Accesspoint)] #[serde(rename_all = "snake_case")] pub struct Accesspoint {
    kind: String,
    api_version: String,
    metadata: ServerObjectMetadata,
    spec: AccesspointSpec,

}
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, utoipa::ToSchema)]
#[schema(as = serveradmin::innogames::de::v1::PrinterSpec)]
#[serde_inline_default::serde_inline_default]
#[serde(rename_all = "snake_case")]
pub struct PrinterSpec {
    mac: Vec<String>,
    provider_network: Option<String>,
    hardware_model: Option<String>,
    primary_ip6: Option<String>,
    #[serde_inline_default(String::from("online"))]state: String,
    route_network: Option<String>,
    #[serde_inline_default(String::from("officeit"))]project: String,
    monitoring_groups: Option<Vec<String>>,
    network_zones: Option<Vec<String>>,
    officetool_id: Option<String>,
    ipv4: String,
    ipv6: String,
    hostname: String,
    servertype: String,
    object_id: String,

}
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, utoipa::ToSchema)] #[schema(as = serveradmin::innogames::de::v1::PrinterList)] #[serde(rename_all = "snake_case")] pub struct PrinterList {
    kind: String,
    api_version: String,
    items: Printer,

}
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, utoipa::ToSchema)] #[schema(as = serveradmin::innogames::de::v1::Printer)] #[serde(rename_all = "snake_case")] pub struct Printer {
    kind: String,
    api_version: String,
    metadata: ServerObjectMetadata,
    spec: PrinterSpec,

}
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, utoipa::ToSchema)]
#[schema(as = serveradmin::innogames::de::v1::CloudNetworkSpec)]
#[serde_inline_default::serde_inline_default]
#[serde(rename_all = "snake_case")]
pub struct CloudNetworkSpec {
    state: String,
    service_groups: Option<Vec<String>>,
    provider: String,
    project: String,
    responsible_admin: Vec<String>,
    ipv4: String,
    ipv6: String,
    hostname: String,
    servertype: String,
    object_id: String,

}
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, utoipa::ToSchema)] #[schema(as = serveradmin::innogames::de::v1::CloudNetworkList)] #[serde(rename_all = "snake_case")] pub struct CloudNetworkList {
    kind: String,
    api_version: String,
    items: CloudNetwork,

}
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, utoipa::ToSchema)] #[schema(as = serveradmin::innogames::de::v1::CloudNetwork)] #[serde(rename_all = "snake_case")] pub struct CloudNetwork {
    kind: String,
    api_version: String,
    metadata: ServerObjectMetadata,
    spec: CloudNetworkSpec,

}
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, utoipa::ToSchema)]
#[schema(as = serveradmin::innogames::de::v1::IpmiSpec)]
#[serde_inline_default::serde_inline_default]
#[serde(rename_all = "snake_case")]
pub struct IpmiSpec {
    service_groups: Option<Vec<String>>,
    project_network: Option<String>,
    route_network: Option<String>,
    dell_servicetag: Option<Vec<String>>,
    primary_ip6: Option<String>,
    #[serde_inline_default(Some(bool::from(false)))]no_monitoring: Option<bool>,
    function: Option<String>,
    responsible_admin: Option<Vec<String>>,
    monitoring_queue: Option<String>,
    ipmi_of: Option<String>,
    bladecenter: Option<String>,
    bladecenter_slot: Option<Vec<String>>,
    rack: Option<String>,
    storage_hardware: Option<Vec<String>>,
    network_type: Option<String>,
    sshfp: Option<Vec<String>>,
    datacenter: Option<String>,
    #[serde_inline_default(String::from("online"))]state: String,
    #[serde_inline_default(String::from("production"))]environment: String,
    hardware_model: Option<String>,
    network_zone: Option<String>,
    mac: Option<Vec<String>>,
    provider_network: Option<String>,
    #[serde_inline_default(String::from("ndco"))]project: String,
    network_zones: Option<Vec<String>>,
    rack_colo: Option<String>,
    rack_number: Option<String>,
    rack_row: Option<String>,
    #[serde_inline_default(Some(vec![String::from("ping, ping_6, ipmi_sel, ipmi_ssl_cert, ipmi_sensors")]))]monitoring_checks: Option<Vec<String>>,
    nessus_scan_id: Option<i32>,
    serial_number: Option<String>,
    ipv4: String,
    ipv6: String,
    hostname: String,
    servertype: String,
    object_id: String,

}
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, utoipa::ToSchema)] #[schema(as = serveradmin::innogames::de::v1::IpmiList)] #[serde(rename_all = "snake_case")] pub struct IpmiList {
    kind: String,
    api_version: String,
    items: Ipmi,

}
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, utoipa::ToSchema)] #[schema(as = serveradmin::innogames::de::v1::Ipmi)] #[serde(rename_all = "snake_case")] pub struct Ipmi {
    kind: String,
    api_version: String,
    metadata: ServerObjectMetadata,
    spec: IpmiSpec,

}
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, utoipa::ToSchema)]
#[schema(as = serveradmin::innogames::de::v1::VoipPhoneSpec)]
#[serde_inline_default::serde_inline_default]
#[serde(rename_all = "snake_case")]
pub struct VoipPhoneSpec {
    project: Option<String>,
    route_network: Option<String>,
    primary_ip6: Option<String>,
    mac: Option<Vec<String>>,
    network_zones: Option<Vec<String>>,
    datacenter: Option<String>,
    #[serde_inline_default(String::from("online"))]state: String,
    officetool_id: Option<String>,
    ipv4: String,
    ipv6: String,
    hostname: String,
    servertype: String,
    object_id: String,

}
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, utoipa::ToSchema)] #[schema(as = serveradmin::innogames::de::v1::VoipPhoneList)] #[serde(rename_all = "snake_case")] pub struct VoipPhoneList {
    kind: String,
    api_version: String,
    items: VoipPhone,

}
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, utoipa::ToSchema)] #[schema(as = serveradmin::innogames::de::v1::VoipPhone)] #[serde(rename_all = "snake_case")] pub struct VoipPhone {
    kind: String,
    api_version: String,
    metadata: ServerObjectMetadata,
    spec: VoipPhoneSpec,

}
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, utoipa::ToSchema)]
#[schema(as = serveradmin::innogames::de::v1::ProjectDomainSpec)]
#[serde_inline_default::serde_inline_default]
#[serde(rename_all = "snake_case")]
pub struct ProjectDomainSpec {
    mx: Option<String>,
    project: String,
    provider_domain: Option<String>,
    internal: Option<bool>,
    hostname: String,
    servertype: String,
    object_id: String,

}
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, utoipa::ToSchema)] #[schema(as = serveradmin::innogames::de::v1::ProjectDomainList)] #[serde(rename_all = "snake_case")] pub struct ProjectDomainList {
    kind: String,
    api_version: String,
    items: ProjectDomain,

}
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, utoipa::ToSchema)] #[schema(as = serveradmin::innogames::de::v1::ProjectDomain)] #[serde(rename_all = "snake_case")] pub struct ProjectDomain {
    kind: String,
    api_version: String,
    metadata: ServerObjectMetadata,
    spec: ProjectDomainSpec,

}
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, utoipa::ToSchema)]
#[schema(as = serveradmin::innogames::de::v1::RackSpec)]
#[serde_inline_default::serde_inline_default]
#[serde(rename_all = "snake_case")]
pub struct RackSpec {
    rack_row: Option<String>,
    rack_number: Option<String>,
    rack_colo: Option<String>,
    datacenter: Option<String>,
    responsible_admin: Option<Vec<String>>,
    order_date: Option<String>,
    order_id: Option<Vec<String>>,
    #[serde_inline_default(String::from("online"))]state: String,
    datacenter_colo: Option<String>,
    #[serde_inline_default(String::from("ndco"))]project: String,
    hostname: String,
    servertype: String,
    object_id: String,

}
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, utoipa::ToSchema)] #[schema(as = serveradmin::innogames::de::v1::RackList)] #[serde(rename_all = "snake_case")] pub struct RackList {
    kind: String,
    api_version: String,
    items: Rack,

}
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, utoipa::ToSchema)] #[schema(as = serveradmin::innogames::de::v1::Rack)] #[serde(rename_all = "snake_case")] pub struct Rack {
    kind: String,
    api_version: String,
    metadata: ServerObjectMetadata,
    spec: RackSpec,

}
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, utoipa::ToSchema)]
#[schema(as = serveradmin::innogames::de::v1::HardwareExternalSpec)]
#[serde_inline_default::serde_inline_default]
#[serde(rename_all = "snake_case")]
pub struct HardwareExternalSpec {
    datacenter: Option<String>,
    security_availability: Option<String>,
    dell_servicetag: Option<Vec<String>>,
    rack_height: Option<i32>,
    monitoring_queues: Option<Vec<String>>,
    security_confidentiality: Option<String>,
    #[serde_inline_default(Some(i32::from(300)))]backup_prio: Option<i32>,
    order_date: Option<String>,
    order_id: Option<Vec<String>>,
    #[serde_inline_default(Some(bool::from(false)))]no_monitoring: Option<bool>,
    #[serde_inline_default(Some(bool::from(false)))]puppet_disabled: Option<bool>,
    #[serde_inline_default(String::from("ssd"))]harddisk_type: String,
    primary_ip6: Option<String>,
    rack_position: Option<i32>,
    description: Option<String>,
    arch: Option<String>,
    #[serde_inline_default(bool::from(true))]auto_update: bool,
    num_cpu: i32,
    security_integrity: Option<String>,
    puppet_environment: Option<String>,
    function: Option<String>,
    graphite_graphs: Option<Vec<String>>,
    #[serde_inline_default(String::from("puppetca.innogames.de"))]puppet_ca: String,
    puppet_master: String,
    repositories: Option<Vec<String>>,
    allow_to: Option<Vec<String>>,
    legal_personal_data: Option<String>,
    memory_modules: Option<Vec<String>>,
    assigned_to: Option<String>,
    route_network: Option<String>,
    acl: Option<Vec<String>>,
    responsible_admin: Vec<String>,
    #[serde_inline_default(Some(String::from("aptly.innogames.de")))]apt_repository_url: Option<String>,
    memory: Option<i32>,
    sshfp: Option<Vec<String>>,
    service_groups: Option<Vec<String>>,
    disk_size_gib: Option<i32>,
    loadbalancer: Option<Vec<String>>,
    hardware_model: Option<String>,
    ipmi: Option<String>,
    cpu_clock_mhz: Option<i32>,
    puppet_classes: Option<Vec<String>>,
    #[serde_inline_default(Some(vec![String::from("ping")]))]monitoring_checks: Option<Vec<String>>,
    subproject: Option<String>,
    #[serde_inline_default(String::from("online"))]state: String,
    #[serde_inline_default(String::from("production"))]environment: String,
    network_zone: Option<String>,
    monitoring_queue: Option<String>,
    network_zones: Option<Vec<String>>,
    backup_storage: Option<String>,
    firmware_versions: Option<Vec<String>>,
    cpu_model: Option<String>,
    project: String,
    vlan_networks: Option<Vec<String>>,
    monitoring_groups: Option<Vec<String>>,
    datacenter_type: Option<String>,
    #[serde_inline_default(vec![String::from("raid")])]storage_hardware: Vec<String>,
    #[serde_inline_default(bool::from(true))]backup_disabled: bool,
    #[serde_inline_default(Some(bool::from(false)))]backup_canshrink: Option<bool>,
    backup_maxage: Option<i32>,
    backup_window: Option<Vec<String>>,
    #[serde_inline_default(String::from("wheezy"))]os: String,
    powerports: Option<Vec<String>>,
    switchports: Option<Vec<String>>,
    mac: Option<Vec<String>>,
    ipv4: String,
    ipv6: String,
    hostname: String,
    servertype: String,
    object_id: String,

}
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, utoipa::ToSchema)] #[schema(as = serveradmin::innogames::de::v1::HardwareExternalList)] #[serde(rename_all = "snake_case")] pub struct HardwareExternalList {
    kind: String,
    api_version: String,
    items: HardwareExternal,

}
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, utoipa::ToSchema)] #[schema(as = serveradmin::innogames::de::v1::HardwareExternal)] #[serde(rename_all = "snake_case")] pub struct HardwareExternal {
    kind: String,
    api_version: String,
    metadata: ServerObjectMetadata,
    spec: HardwareExternalSpec,

}
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, utoipa::ToSchema)]
#[schema(as = serveradmin::innogames::de::v1::VpnSpec)]
#[serde_inline_default::serde_inline_default]
#[serde(rename_all = "snake_case")]
pub struct VpnSpec {
    #[serde_inline_default(bool::from(true))]backup_disabled: bool,
    #[serde_inline_default(Some(bool::from(false)))]backup_canshrink: Option<bool>,
    backup_maxage: Option<i32>,
    backup_window: Option<Vec<String>>,
    #[serde_inline_default(Some(i32::from(300)))]backup_prio: Option<i32>,
    vlan_networks: Option<Vec<String>>,
    route_network: Option<String>,
    #[serde_inline_default(bool::from(false))]auto_update: bool,
    #[serde_inline_default(Some(bool::from(false)))]puppet_disabled: Option<bool>,
    primary_ip6: Option<String>,
    #[serde_inline_default(Some(String::from("puppetca.innogames.de")))]puppet_ca: Option<String>,
    puppet_master: Option<String>,
    routed_networks: Option<Vec<String>>,
    domain: Option<Vec<String>>,
    powerports: Option<Vec<String>>,
    datacenter: Option<String>,
    allow_to: Option<Vec<String>>,
    #[serde_inline_default(Some(vec![String::from("ping")]))]monitoring_checks: Option<Vec<String>>,
    function: Option<String>,
    vlan_interfaces: Option<Vec<String>>,
    no_monitoring: Option<bool>,
    #[serde_inline_default(String::from("production"))]environment: String,
    rack: Option<String>,
    acl: Option<Vec<String>>,
    responsible_admin: Option<Vec<String>>,
    monitoring_queue: Option<String>,
    #[serde_inline_default(String::from("online"))]state: String,
    service_groups: Option<Vec<String>>,
    puppet_classes: Option<Vec<String>>,
    sshfp: Option<Vec<String>>,
    backup_storage: Option<String>,
    #[serde_inline_default(String::from("ndco"))]project: String,
    protocol_port: Option<Vec<String>>,
    os: Option<String>,
    repositories: Option<Vec<String>>,
    puppet_environment: Option<String>,
    network_zones: Option<Vec<String>>,
    graphite_graphs: Option<Vec<String>>,
    num_cpu: Option<i32>,
    memory: Option<i32>,
    disk_size_gib: Option<i32>,
    harddisk_type: Option<String>,
    subproject: Option<String>,
    mac: Option<Vec<String>>,
    network_zone: Option<String>,
    ipmi: Option<String>,
    hardware_model: Option<String>,
    #[serde_inline_default(Some(String::from("aptly.innogames.de")))]repositories_host: Option<String>,
    monitoring_groups: Option<Vec<String>>,
    ipv4: String,
    ipv6: String,
    hostname: String,
    servertype: String,
    object_id: String,

}
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, utoipa::ToSchema)] #[schema(as = serveradmin::innogames::de::v1::VpnList)] #[serde(rename_all = "snake_case")] pub struct VpnList {
    kind: String,
    api_version: String,
    items: Vpn,

}
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, utoipa::ToSchema)] #[schema(as = serveradmin::innogames::de::v1::Vpn)] #[serde(rename_all = "snake_case")] pub struct Vpn {
    kind: String,
    api_version: String,
    metadata: ServerObjectMetadata,
    spec: VpnSpec,

}
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, utoipa::ToSchema)]
#[schema(as = serveradmin::innogames::de::v1::ProviderDomainSpec)]
#[serde_inline_default::serde_inline_default]
#[serde(rename_all = "snake_case")]
pub struct ProviderDomainSpec {
    mx: Option<String>,
    #[serde_inline_default(String::from("online"))]state: String,
    #[serde_inline_default(Some(bool::from(false)))]hidden_primary: Option<bool>,
    dns_soa: Option<String>,
    hostname: String,
    servertype: String,
    object_id: String,

}
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, utoipa::ToSchema)] #[schema(as = serveradmin::innogames::de::v1::ProviderDomainList)] #[serde(rename_all = "snake_case")] pub struct ProviderDomainList {
    kind: String,
    api_version: String,
    items: ProviderDomain,

}
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, utoipa::ToSchema)] #[schema(as = serveradmin::innogames::de::v1::ProviderDomain)] #[serde(rename_all = "snake_case")] pub struct ProviderDomain {
    kind: String,
    api_version: String,
    metadata: ServerObjectMetadata,
    spec: ProviderDomainSpec,

}
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, utoipa::ToSchema)]
#[schema(as = serveradmin::innogames::de::v1::LbPoolSpec)]
#[serde_inline_default::serde_inline_default]
#[serde(rename_all = "snake_case")]
pub struct LbPoolSpec {
    acl: Option<Vec<String>>,
    allow_from: Option<Vec<String>>,
    domain: Option<Vec<String>>,
    #[serde_inline_default(String::from("production"))]environment: String,
    function: Option<String>,
    game_market: Option<String>,
    game_type: Option<String>,
    game_world: Option<i32>,
    monitoring_checks: Option<Vec<String>>,
    monitoring_queue: Option<String>,
    monitoring_queues: Option<Vec<String>>,
    no_monitoring: Option<bool>,
    project: Option<String>,
    protocol_port: Option<Vec<String>>,
    responsible_admin: Option<Vec<String>>,
    service_groups: Option<Vec<String>>,
    #[serde_inline_default(String::from("online"))]state: String,
    subproject: Option<String>,
    vlan_interfaces: Option<Vec<String>>,
    lb_pool_nodes: Option<Vec<String>>,
    aws_placement: Option<Vec<String>>,
    datacenter: Option<String>,
    health_checks: Option<Vec<String>>,
    hostname: String,
    servertype: String,
    object_id: String,

}
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, utoipa::ToSchema)] #[schema(as = serveradmin::innogames::de::v1::LbPoolList)] #[serde(rename_all = "snake_case")] pub struct LbPoolList {
    kind: String,
    api_version: String,
    items: LbPool,

}
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, utoipa::ToSchema)] #[schema(as = serveradmin::innogames::de::v1::LbPool)] #[serde(rename_all = "snake_case")] pub struct LbPool {
    kind: String,
    api_version: String,
    metadata: ServerObjectMetadata,
    spec: LbPoolSpec,

}
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, utoipa::ToSchema)]
#[schema(as = serveradmin::innogames::de::v1::BondInterfaceSpec)]
#[serde_inline_default::serde_inline_default]
#[serde(rename_all = "snake_case")]
pub struct BondInterfaceSpec {
    bond_no: Option<String>,
    network_type: Option<String>,
    #[serde_inline_default(String::from("ndco"))]project: String,
    hostname: String,
    servertype: String,
    object_id: String,

}
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, utoipa::ToSchema)] #[schema(as = serveradmin::innogames::de::v1::BondInterfaceList)] #[serde(rename_all = "snake_case")] pub struct BondInterfaceList {
    kind: String,
    api_version: String,
    items: BondInterface,

}
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, utoipa::ToSchema)] #[schema(as = serveradmin::innogames::de::v1::BondInterface)] #[serde(rename_all = "snake_case")] pub struct BondInterface {
    kind: String,
    api_version: String,
    metadata: ServerObjectMetadata,
    spec: BondInterfaceSpec,

}
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, utoipa::ToSchema)]
#[schema(as = serveradmin::innogames::de::v1::FloatingAddressSpec)]
#[serde_inline_default::serde_inline_default]
#[serde(rename_all = "snake_case")]
pub struct FloatingAddressSpec {
    #[serde_inline_default(String::from("production"))]environment: String,
    project_network: Option<String>,
    no_monitoring: Option<bool>,
    route_network: Option<String>,
    function: Option<String>,
    sshfp: Option<Vec<String>>,
    monitoring_groups: Option<Vec<String>>,
    subproject: Option<String>,
    #[serde_inline_default(String::from("online"))]state: String,
    acl: Option<Vec<String>>,
    allow_from: Option<Vec<String>>,
    primary_ip6: Option<String>,
    #[serde_inline_default(Some(vec![String::from("ping")]))]monitoring_checks: Option<Vec<String>>,
    responsible_admin: Vec<String>,
    datacenter: Option<String>,
    monitoring_queue: Option<String>,
    project: String,
    service_groups: Option<Vec<String>>,
    ipv4: String,
    ipv6: String,
    hostname: String,
    servertype: String,
    object_id: String,

}
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, utoipa::ToSchema)] #[schema(as = serveradmin::innogames::de::v1::FloatingAddressList)] #[serde(rename_all = "snake_case")] pub struct FloatingAddressList {
    kind: String,
    api_version: String,
    items: FloatingAddress,

}
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, utoipa::ToSchema)] #[schema(as = serveradmin::innogames::de::v1::FloatingAddress)] #[serde(rename_all = "snake_case")] pub struct FloatingAddress {
    kind: String,
    api_version: String,
    metadata: ServerObjectMetadata,
    spec: FloatingAddressSpec,

}
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, utoipa::ToSchema)]
#[schema(as = serveradmin::innogames::de::v1::VmExternalSpec)]
#[serde_inline_default::serde_inline_default]
#[serde(rename_all = "snake_case")]
pub struct VmExternalSpec {
    loadbalancer: Option<Vec<String>>,
    security_availability: Option<String>,
    #[serde_inline_default(Some(String::from("buster")))]os: Option<String>,
    #[serde_inline_default(String::from("puppetca.innogames.de"))]puppet_ca: String,
    puppet_master: String,
    #[serde_inline_default(Some(String::from("InnoGames")))]group_company: Option<String>,
    game_market: Option<String>,
    sshfp: Option<Vec<String>>,
    #[serde_inline_default(Some(bool::from(false)))]auto_update: Option<bool>,
    game_world: Option<i32>,
    #[serde_inline_default(Some(bool::from(false)))]backup_disabled: Option<bool>,
    #[serde_inline_default(Some(bool::from(false)))]backup_canshrink: Option<bool>,
    #[serde_inline_default(Some(i32::from(36)))]backup_maxage: Option<i32>,
    #[serde_inline_default(Some(i32::from(300)))]backup_prio: Option<i32>,
    #[serde_inline_default(i32::from(8))]num_cpu: i32,
    #[serde_inline_default(Some(bool::from(false)))]no_monitoring: Option<bool>,
    memory: Option<i32>,
    #[serde_inline_default(Some(bool::from(false)))]puppet_disabled: Option<bool>,
    security_confidentiality: Option<String>,
    primary_ip6: Option<String>,
    disk_size_gib: i32,
    graphite_graphs: Option<Vec<String>>,
    security_integrity: Option<String>,
    legal_personal_data: Option<String>,
    description: Option<String>,
    function: Option<String>,
    monitoring_queues: Option<Vec<String>>,
    puppet_environment: Option<String>,
    #[serde_inline_default(Some(vec![String::from("2-5")]))]backup_window: Option<Vec<String>>,
    #[serde_inline_default(Some(vec![String::from("int:innogames:stable")]))]repositories: Option<Vec<String>>,
    domain: Option<Vec<String>>,
    responsible_admin: Vec<String>,
    route_network: Option<String>,
    acl: Option<Vec<String>>,
    subproject: Option<String>,
    network_zones: Option<Vec<String>>,
    #[serde_inline_default(String::from("online"))]state: String,
    network_type: Option<String>,
    #[serde_inline_default(Some(vec![String::from("ping")]))]monitoring_checks: Option<Vec<String>>,
    puppet_classes: Option<Vec<String>>,
    provider_network: Option<String>,
    monitoring_satellite_of: Option<String>,
    #[serde_inline_default(String::from("production"))]environment: String,
    datacenter: Option<String>,
    network_zone: Option<String>,
    instance: Option<i32>,
    service_groups: Option<Vec<String>>,
    monitoring_queue: Option<String>,
    backup_storage: Option<String>,
    project: String,
    vlan_networks: Option<Vec<String>>,
    restic_keep_monthly: Option<i32>,
    restic_keep_yearly: Option<i32>,
    datacenter_type: Option<String>,
    restic_group: Option<String>,
    #[serde_inline_default(Some(i32::from(3)))]restic_keep_daily: Option<i32>,
    #[serde_inline_default(Some(i32::from(3)))]restic_keep_last: Option<i32>,
    #[serde_inline_default(Some(i32::from(2)))]restic_keep_weekly: Option<i32>,
    #[serde_inline_default(Some(i32::from(-1)))]load_99: Option<i32>,
    #[serde_inline_default(Some(String::from("aptly.innogames.de")))]repositories_host: Option<String>,
    monitoring_groups: Option<Vec<String>>,
    ipv4: String,
    ipv6: String,
    hostname: String,
    servertype: String,
    object_id: String,

}
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, utoipa::ToSchema)] #[schema(as = serveradmin::innogames::de::v1::VmExternalList)] #[serde(rename_all = "snake_case")] pub struct VmExternalList {
    kind: String,
    api_version: String,
    items: VmExternal,

}
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, utoipa::ToSchema)] #[schema(as = serveradmin::innogames::de::v1::VmExternal)] #[serde(rename_all = "snake_case")] pub struct VmExternal {
    kind: String,
    api_version: String,
    metadata: ServerObjectMetadata,
    spec: VmExternalSpec,

}
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize,
utoipa::ToSchema)] #[serde(rename_all = "snake_case")]
#[schema(as = serveradmin::innogames::de::v1::ServerObjectMetadata)] pub
struct ServerObjectMetadata { name: String, }
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)] pub struct
Attribute
{
    pub name: String, pub typ: AttributeType, pub reversed_attribute_id:
    String, pub multi: bool, pub regexp: String, pub readonly: bool, pub
    hovertext: String, pub required: bool, pub default: String,
} #[derive(Clone, Debug, serde::Deserialize, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")] pub enum AttributeType
{
    Domain, Date, Number, Inet, Supernet, Macaddr, Boolean, Relation, String,
    Datetime, Reverse,
}

use utoipa::openapi::{path::{Operation, Parameter, ParameterIn}, request_body::RequestBody, Components, Content, PathItem, Paths, Ref, Response};

pub fn openapi() -> utoipa::openapi::OpenApi {
utoipa::openapi::OpenApi::builder()
    .paths(
        Paths::builder()
        
.path(
    "/apis/serveradmin.innogames.de/v1/namespace//memorymodules",
    PathItem::builder()
        .parameters(Some(vec![
            
        ]))
        .operation(utoipa::openapi::HttpMethod::Get, Operation::builder()
            .response("200", Response::builder()
                .content("application/json", Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.MemoryModuleList"))).build())
                .build(),
            )
        .build())
        .operation(utoipa::openapi::HttpMethod::Post, Operation::builder()
            .request_body(Some(RequestBody::builder().content("application/json", Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.MemoryModule"))).build()).build()))
            .response(
                "200",
                Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.MemoryModuleList"))).build()).build(),
            )
            .build(),
        )
        .build(),
)
.path(
    "/apis/serveradmin.innogames.de/v1/namespace//memorymodules/{name}",
    PathItem::builder()
        .parameters(Some(vec![
            
            Parameter::builder().name("name").parameter_in(ParameterIn::Path).schema(Some("string")).build(),
        ]))
        .operation(
            utoipa::openapi::HttpMethod::Get,
            Operation::builder()
                .response(
                    "200",
                    Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.MemoryModule"))).build()).build(),
                )
                .build(),
        )
        .operation(
            utoipa::openapi::HttpMethod::Delete,
            Operation::builder()
                .response(
                    "200",
                    Response::builder().content( "application/json", Content::builder().schema(Some(Ref::new( "#/components/schemas/serveradmin.innogames.de.v1.MemoryModule"))).build()).build(),
                )
                .build(),
        )
        .operation(
            utoipa::openapi::HttpMethod::Patch,
            Operation::builder()
                .request_body(Some(RequestBody::builder().content("application/json", Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.MemoryModule"))).build()).build(),))
                .response(
                    "200",
                    Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.MemoryModule"))).build()).build(),
                )
                .build(),
        )
        .operation(
            utoipa::openapi::HttpMethod::Put,
            Operation::builder()
                .request_body(Some(RequestBody::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.MemoryModule"))).build()).build()))
                .response(
                    "200",
                    Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.MemoryModule"))).build()).build(),
                )
                .build(),
        )
        .build(),
)
    

.path(
    "/apis/serveradmin.innogames.de/v1/namespace/{namespace}/hdmitransmitters",
    PathItem::builder()
        .parameters(Some(vec![
            Parameter::builder().name("namespace").schema(Some("string")).parameter_in(ParameterIn::Path).build(),
        ]))
        .operation(utoipa::openapi::HttpMethod::Get, Operation::builder()
            .response("200", Response::builder()
                .content("application/json", Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.HdmiTransmitterList"))).build())
                .build(),
            )
        .build())
        .operation(utoipa::openapi::HttpMethod::Post, Operation::builder()
            .request_body(Some(RequestBody::builder().content("application/json", Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.HdmiTransmitter"))).build()).build()))
            .response(
                "200",
                Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.HdmiTransmitterList"))).build()).build(),
            )
            .build(),
        )
        .build(),
)
.path(
    "/apis/serveradmin.innogames.de/v1/namespace/{namespace}/hdmitransmitters/{name}",
    PathItem::builder()
        .parameters(Some(vec![
            Parameter::builder().name("namespace").schema(Some("string")).parameter_in(ParameterIn::Path).build(),
            Parameter::builder().name("name").parameter_in(ParameterIn::Path).schema(Some("string")).build(),
        ]))
        .operation(
            utoipa::openapi::HttpMethod::Get,
            Operation::builder()
                .response(
                    "200",
                    Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.HdmiTransmitter"))).build()).build(),
                )
                .build(),
        )
        .operation(
            utoipa::openapi::HttpMethod::Delete,
            Operation::builder()
                .response(
                    "200",
                    Response::builder().content( "application/json", Content::builder().schema(Some(Ref::new( "#/components/schemas/serveradmin.innogames.de.v1.HdmiTransmitter"))).build()).build(),
                )
                .build(),
        )
        .operation(
            utoipa::openapi::HttpMethod::Patch,
            Operation::builder()
                .request_body(Some(RequestBody::builder().content("application/json", Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.HdmiTransmitter"))).build()).build(),))
                .response(
                    "200",
                    Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.HdmiTransmitter"))).build()).build(),
                )
                .build(),
        )
        .operation(
            utoipa::openapi::HttpMethod::Put,
            Operation::builder()
                .request_body(Some(RequestBody::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.HdmiTransmitter"))).build()).build()))
                .response(
                    "200",
                    Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.HdmiTransmitter"))).build()).build(),
                )
                .build(),
        )
        .build(),
)
    

.path(
    "/apis/serveradmin.innogames.de/v1/namespace/{namespace}/servicegroups",
    PathItem::builder()
        .parameters(Some(vec![
            Parameter::builder().name("namespace").schema(Some("string")).parameter_in(ParameterIn::Path).build(),
        ]))
        .operation(utoipa::openapi::HttpMethod::Get, Operation::builder()
            .response("200", Response::builder()
                .content("application/json", Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.ServiceGroupList"))).build())
                .build(),
            )
        .build())
        .operation(utoipa::openapi::HttpMethod::Post, Operation::builder()
            .request_body(Some(RequestBody::builder().content("application/json", Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.ServiceGroup"))).build()).build()))
            .response(
                "200",
                Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.ServiceGroupList"))).build()).build(),
            )
            .build(),
        )
        .build(),
)
.path(
    "/apis/serveradmin.innogames.de/v1/namespace/{namespace}/servicegroups/{name}",
    PathItem::builder()
        .parameters(Some(vec![
            Parameter::builder().name("namespace").schema(Some("string")).parameter_in(ParameterIn::Path).build(),
            Parameter::builder().name("name").parameter_in(ParameterIn::Path).schema(Some("string")).build(),
        ]))
        .operation(
            utoipa::openapi::HttpMethod::Get,
            Operation::builder()
                .response(
                    "200",
                    Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.ServiceGroup"))).build()).build(),
                )
                .build(),
        )
        .operation(
            utoipa::openapi::HttpMethod::Delete,
            Operation::builder()
                .response(
                    "200",
                    Response::builder().content( "application/json", Content::builder().schema(Some(Ref::new( "#/components/schemas/serveradmin.innogames.de.v1.ServiceGroup"))).build()).build(),
                )
                .build(),
        )
        .operation(
            utoipa::openapi::HttpMethod::Patch,
            Operation::builder()
                .request_body(Some(RequestBody::builder().content("application/json", Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.ServiceGroup"))).build()).build(),))
                .response(
                    "200",
                    Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.ServiceGroup"))).build()).build(),
                )
                .build(),
        )
        .operation(
            utoipa::openapi::HttpMethod::Put,
            Operation::builder()
                .request_body(Some(RequestBody::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.ServiceGroup"))).build()).build()))
                .response(
                    "200",
                    Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.ServiceGroup"))).build()).build(),
                )
                .build(),
        )
        .build(),
)
    

.path(
    "/apis/serveradmin.innogames.de/v1/namespace//monitoringzones",
    PathItem::builder()
        .parameters(Some(vec![
            
        ]))
        .operation(utoipa::openapi::HttpMethod::Get, Operation::builder()
            .response("200", Response::builder()
                .content("application/json", Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.MonitoringZoneList"))).build())
                .build(),
            )
        .build())
        .operation(utoipa::openapi::HttpMethod::Post, Operation::builder()
            .request_body(Some(RequestBody::builder().content("application/json", Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.MonitoringZone"))).build()).build()))
            .response(
                "200",
                Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.MonitoringZoneList"))).build()).build(),
            )
            .build(),
        )
        .build(),
)
.path(
    "/apis/serveradmin.innogames.de/v1/namespace//monitoringzones/{name}",
    PathItem::builder()
        .parameters(Some(vec![
            
            Parameter::builder().name("name").parameter_in(ParameterIn::Path).schema(Some("string")).build(),
        ]))
        .operation(
            utoipa::openapi::HttpMethod::Get,
            Operation::builder()
                .response(
                    "200",
                    Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.MonitoringZone"))).build()).build(),
                )
                .build(),
        )
        .operation(
            utoipa::openapi::HttpMethod::Delete,
            Operation::builder()
                .response(
                    "200",
                    Response::builder().content( "application/json", Content::builder().schema(Some(Ref::new( "#/components/schemas/serveradmin.innogames.de.v1.MonitoringZone"))).build()).build(),
                )
                .build(),
        )
        .operation(
            utoipa::openapi::HttpMethod::Patch,
            Operation::builder()
                .request_body(Some(RequestBody::builder().content("application/json", Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.MonitoringZone"))).build()).build(),))
                .response(
                    "200",
                    Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.MonitoringZone"))).build()).build(),
                )
                .build(),
        )
        .operation(
            utoipa::openapi::HttpMethod::Put,
            Operation::builder()
                .request_body(Some(RequestBody::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.MonitoringZone"))).build()).build()))
                .response(
                    "200",
                    Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.MonitoringZone"))).build()).build(),
                )
                .build(),
        )
        .build(),
)
    

.path(
    "/apis/serveradmin.innogames.de/v1/namespace/{namespace}/hwloadbalancers",
    PathItem::builder()
        .parameters(Some(vec![
            Parameter::builder().name("namespace").schema(Some("string")).parameter_in(ParameterIn::Path).build(),
        ]))
        .operation(utoipa::openapi::HttpMethod::Get, Operation::builder()
            .response("200", Response::builder()
                .content("application/json", Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.HwLoadbalancerList"))).build())
                .build(),
            )
        .build())
        .operation(utoipa::openapi::HttpMethod::Post, Operation::builder()
            .request_body(Some(RequestBody::builder().content("application/json", Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.HwLoadbalancer"))).build()).build()))
            .response(
                "200",
                Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.HwLoadbalancerList"))).build()).build(),
            )
            .build(),
        )
        .build(),
)
.path(
    "/apis/serveradmin.innogames.de/v1/namespace/{namespace}/hwloadbalancers/{name}",
    PathItem::builder()
        .parameters(Some(vec![
            Parameter::builder().name("namespace").schema(Some("string")).parameter_in(ParameterIn::Path).build(),
            Parameter::builder().name("name").parameter_in(ParameterIn::Path).schema(Some("string")).build(),
        ]))
        .operation(
            utoipa::openapi::HttpMethod::Get,
            Operation::builder()
                .response(
                    "200",
                    Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.HwLoadbalancer"))).build()).build(),
                )
                .build(),
        )
        .operation(
            utoipa::openapi::HttpMethod::Delete,
            Operation::builder()
                .response(
                    "200",
                    Response::builder().content( "application/json", Content::builder().schema(Some(Ref::new( "#/components/schemas/serveradmin.innogames.de.v1.HwLoadbalancer"))).build()).build(),
                )
                .build(),
        )
        .operation(
            utoipa::openapi::HttpMethod::Patch,
            Operation::builder()
                .request_body(Some(RequestBody::builder().content("application/json", Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.HwLoadbalancer"))).build()).build(),))
                .response(
                    "200",
                    Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.HwLoadbalancer"))).build()).build(),
                )
                .build(),
        )
        .operation(
            utoipa::openapi::HttpMethod::Put,
            Operation::builder()
                .request_body(Some(RequestBody::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.HwLoadbalancer"))).build()).build()))
                .response(
                    "200",
                    Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.HwLoadbalancer"))).build()).build(),
                )
                .build(),
        )
        .build(),
)
    

.path(
    "/apis/serveradmin.innogames.de/v1/namespace/{namespace}/hardwares",
    PathItem::builder()
        .parameters(Some(vec![
            Parameter::builder().name("namespace").schema(Some("string")).parameter_in(ParameterIn::Path).build(),
        ]))
        .operation(utoipa::openapi::HttpMethod::Get, Operation::builder()
            .response("200", Response::builder()
                .content("application/json", Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.HardwareList"))).build())
                .build(),
            )
        .build())
        .operation(utoipa::openapi::HttpMethod::Post, Operation::builder()
            .request_body(Some(RequestBody::builder().content("application/json", Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.Hardware"))).build()).build()))
            .response(
                "200",
                Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.HardwareList"))).build()).build(),
            )
            .build(),
        )
        .build(),
)
.path(
    "/apis/serveradmin.innogames.de/v1/namespace/{namespace}/hardwares/{name}",
    PathItem::builder()
        .parameters(Some(vec![
            Parameter::builder().name("namespace").schema(Some("string")).parameter_in(ParameterIn::Path).build(),
            Parameter::builder().name("name").parameter_in(ParameterIn::Path).schema(Some("string")).build(),
        ]))
        .operation(
            utoipa::openapi::HttpMethod::Get,
            Operation::builder()
                .response(
                    "200",
                    Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.Hardware"))).build()).build(),
                )
                .build(),
        )
        .operation(
            utoipa::openapi::HttpMethod::Delete,
            Operation::builder()
                .response(
                    "200",
                    Response::builder().content( "application/json", Content::builder().schema(Some(Ref::new( "#/components/schemas/serveradmin.innogames.de.v1.Hardware"))).build()).build(),
                )
                .build(),
        )
        .operation(
            utoipa::openapi::HttpMethod::Patch,
            Operation::builder()
                .request_body(Some(RequestBody::builder().content("application/json", Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.Hardware"))).build()).build(),))
                .response(
                    "200",
                    Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.Hardware"))).build()).build(),
                )
                .build(),
        )
        .operation(
            utoipa::openapi::HttpMethod::Put,
            Operation::builder()
                .request_body(Some(RequestBody::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.Hardware"))).build()).build()))
                .response(
                    "200",
                    Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.Hardware"))).build()).build(),
                )
                .build(),
        )
        .build(),
)
    

.path(
    "/apis/serveradmin.innogames.de/v1/namespace/{namespace}/hypervisors",
    PathItem::builder()
        .parameters(Some(vec![
            Parameter::builder().name("namespace").schema(Some("string")).parameter_in(ParameterIn::Path).build(),
        ]))
        .operation(utoipa::openapi::HttpMethod::Get, Operation::builder()
            .response("200", Response::builder()
                .content("application/json", Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.HypervisorList"))).build())
                .build(),
            )
        .build())
        .operation(utoipa::openapi::HttpMethod::Post, Operation::builder()
            .request_body(Some(RequestBody::builder().content("application/json", Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.Hypervisor"))).build()).build()))
            .response(
                "200",
                Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.HypervisorList"))).build()).build(),
            )
            .build(),
        )
        .build(),
)
.path(
    "/apis/serveradmin.innogames.de/v1/namespace/{namespace}/hypervisors/{name}",
    PathItem::builder()
        .parameters(Some(vec![
            Parameter::builder().name("namespace").schema(Some("string")).parameter_in(ParameterIn::Path).build(),
            Parameter::builder().name("name").parameter_in(ParameterIn::Path).schema(Some("string")).build(),
        ]))
        .operation(
            utoipa::openapi::HttpMethod::Get,
            Operation::builder()
                .response(
                    "200",
                    Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.Hypervisor"))).build()).build(),
                )
                .build(),
        )
        .operation(
            utoipa::openapi::HttpMethod::Delete,
            Operation::builder()
                .response(
                    "200",
                    Response::builder().content( "application/json", Content::builder().schema(Some(Ref::new( "#/components/schemas/serveradmin.innogames.de.v1.Hypervisor"))).build()).build(),
                )
                .build(),
        )
        .operation(
            utoipa::openapi::HttpMethod::Patch,
            Operation::builder()
                .request_body(Some(RequestBody::builder().content("application/json", Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.Hypervisor"))).build()).build(),))
                .response(
                    "200",
                    Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.Hypervisor"))).build()).build(),
                )
                .build(),
        )
        .operation(
            utoipa::openapi::HttpMethod::Put,
            Operation::builder()
                .request_body(Some(RequestBody::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.Hypervisor"))).build()).build()))
                .response(
                    "200",
                    Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.Hypervisor"))).build()).build(),
                )
                .build(),
        )
        .build(),
)
    

.path(
    "/apis/serveradmin.innogames.de/v1/namespace//externalnetworks",
    PathItem::builder()
        .parameters(Some(vec![
            
        ]))
        .operation(utoipa::openapi::HttpMethod::Get, Operation::builder()
            .response("200", Response::builder()
                .content("application/json", Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.ExternalNetworkList"))).build())
                .build(),
            )
        .build())
        .operation(utoipa::openapi::HttpMethod::Post, Operation::builder()
            .request_body(Some(RequestBody::builder().content("application/json", Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.ExternalNetwork"))).build()).build()))
            .response(
                "200",
                Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.ExternalNetworkList"))).build()).build(),
            )
            .build(),
        )
        .build(),
)
.path(
    "/apis/serveradmin.innogames.de/v1/namespace//externalnetworks/{name}",
    PathItem::builder()
        .parameters(Some(vec![
            
            Parameter::builder().name("name").parameter_in(ParameterIn::Path).schema(Some("string")).build(),
        ]))
        .operation(
            utoipa::openapi::HttpMethod::Get,
            Operation::builder()
                .response(
                    "200",
                    Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.ExternalNetwork"))).build()).build(),
                )
                .build(),
        )
        .operation(
            utoipa::openapi::HttpMethod::Delete,
            Operation::builder()
                .response(
                    "200",
                    Response::builder().content( "application/json", Content::builder().schema(Some(Ref::new( "#/components/schemas/serveradmin.innogames.de.v1.ExternalNetwork"))).build()).build(),
                )
                .build(),
        )
        .operation(
            utoipa::openapi::HttpMethod::Patch,
            Operation::builder()
                .request_body(Some(RequestBody::builder().content("application/json", Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.ExternalNetwork"))).build()).build(),))
                .response(
                    "200",
                    Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.ExternalNetwork"))).build()).build(),
                )
                .build(),
        )
        .operation(
            utoipa::openapi::HttpMethod::Put,
            Operation::builder()
                .request_body(Some(RequestBody::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.ExternalNetwork"))).build()).build()))
                .response(
                    "200",
                    Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.ExternalNetwork"))).build()).build(),
                )
                .build(),
        )
        .build(),
)
    

.path(
    "/apis/serveradmin.innogames.de/v1/namespace/{namespace}/loadbalancers",
    PathItem::builder()
        .parameters(Some(vec![
            Parameter::builder().name("namespace").schema(Some("string")).parameter_in(ParameterIn::Path).build(),
        ]))
        .operation(utoipa::openapi::HttpMethod::Get, Operation::builder()
            .response("200", Response::builder()
                .content("application/json", Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.LoadbalancerList"))).build())
                .build(),
            )
        .build())
        .operation(utoipa::openapi::HttpMethod::Post, Operation::builder()
            .request_body(Some(RequestBody::builder().content("application/json", Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.Loadbalancer"))).build()).build()))
            .response(
                "200",
                Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.LoadbalancerList"))).build()).build(),
            )
            .build(),
        )
        .build(),
)
.path(
    "/apis/serveradmin.innogames.de/v1/namespace/{namespace}/loadbalancers/{name}",
    PathItem::builder()
        .parameters(Some(vec![
            Parameter::builder().name("namespace").schema(Some("string")).parameter_in(ParameterIn::Path).build(),
            Parameter::builder().name("name").parameter_in(ParameterIn::Path).schema(Some("string")).build(),
        ]))
        .operation(
            utoipa::openapi::HttpMethod::Get,
            Operation::builder()
                .response(
                    "200",
                    Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.Loadbalancer"))).build()).build(),
                )
                .build(),
        )
        .operation(
            utoipa::openapi::HttpMethod::Delete,
            Operation::builder()
                .response(
                    "200",
                    Response::builder().content( "application/json", Content::builder().schema(Some(Ref::new( "#/components/schemas/serveradmin.innogames.de.v1.Loadbalancer"))).build()).build(),
                )
                .build(),
        )
        .operation(
            utoipa::openapi::HttpMethod::Patch,
            Operation::builder()
                .request_body(Some(RequestBody::builder().content("application/json", Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.Loadbalancer"))).build()).build(),))
                .response(
                    "200",
                    Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.Loadbalancer"))).build()).build(),
                )
                .build(),
        )
        .operation(
            utoipa::openapi::HttpMethod::Put,
            Operation::builder()
                .request_body(Some(RequestBody::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.Loadbalancer"))).build()).build()))
                .response(
                    "200",
                    Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.Loadbalancer"))).build()).build(),
                )
                .build(),
        )
        .build(),
)
    

.path(
    "/apis/serveradmin.innogames.de/v1/namespace/{namespace}/switchs",
    PathItem::builder()
        .parameters(Some(vec![
            Parameter::builder().name("namespace").schema(Some("string")).parameter_in(ParameterIn::Path).build(),
        ]))
        .operation(utoipa::openapi::HttpMethod::Get, Operation::builder()
            .response("200", Response::builder()
                .content("application/json", Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.SwitchList"))).build())
                .build(),
            )
        .build())
        .operation(utoipa::openapi::HttpMethod::Post, Operation::builder()
            .request_body(Some(RequestBody::builder().content("application/json", Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.Switch"))).build()).build()))
            .response(
                "200",
                Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.SwitchList"))).build()).build(),
            )
            .build(),
        )
        .build(),
)
.path(
    "/apis/serveradmin.innogames.de/v1/namespace/{namespace}/switchs/{name}",
    PathItem::builder()
        .parameters(Some(vec![
            Parameter::builder().name("namespace").schema(Some("string")).parameter_in(ParameterIn::Path).build(),
            Parameter::builder().name("name").parameter_in(ParameterIn::Path).schema(Some("string")).build(),
        ]))
        .operation(
            utoipa::openapi::HttpMethod::Get,
            Operation::builder()
                .response(
                    "200",
                    Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.Switch"))).build()).build(),
                )
                .build(),
        )
        .operation(
            utoipa::openapi::HttpMethod::Delete,
            Operation::builder()
                .response(
                    "200",
                    Response::builder().content( "application/json", Content::builder().schema(Some(Ref::new( "#/components/schemas/serveradmin.innogames.de.v1.Switch"))).build()).build(),
                )
                .build(),
        )
        .operation(
            utoipa::openapi::HttpMethod::Patch,
            Operation::builder()
                .request_body(Some(RequestBody::builder().content("application/json", Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.Switch"))).build()).build(),))
                .response(
                    "200",
                    Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.Switch"))).build()).build(),
                )
                .build(),
        )
        .operation(
            utoipa::openapi::HttpMethod::Put,
            Operation::builder()
                .request_body(Some(RequestBody::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.Switch"))).build()).build()))
                .response(
                    "200",
                    Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.Switch"))).build()).build(),
                )
                .build(),
        )
        .build(),
)
    

.path(
    "/apis/serveradmin.innogames.de/v1/namespace//datacentertypes",
    PathItem::builder()
        .parameters(Some(vec![
            
        ]))
        .operation(utoipa::openapi::HttpMethod::Get, Operation::builder()
            .response("200", Response::builder()
                .content("application/json", Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.DatacenterTypeList"))).build())
                .build(),
            )
        .build())
        .operation(utoipa::openapi::HttpMethod::Post, Operation::builder()
            .request_body(Some(RequestBody::builder().content("application/json", Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.DatacenterType"))).build()).build()))
            .response(
                "200",
                Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.DatacenterTypeList"))).build()).build(),
            )
            .build(),
        )
        .build(),
)
.path(
    "/apis/serveradmin.innogames.de/v1/namespace//datacentertypes/{name}",
    PathItem::builder()
        .parameters(Some(vec![
            
            Parameter::builder().name("name").parameter_in(ParameterIn::Path).schema(Some("string")).build(),
        ]))
        .operation(
            utoipa::openapi::HttpMethod::Get,
            Operation::builder()
                .response(
                    "200",
                    Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.DatacenterType"))).build()).build(),
                )
                .build(),
        )
        .operation(
            utoipa::openapi::HttpMethod::Delete,
            Operation::builder()
                .response(
                    "200",
                    Response::builder().content( "application/json", Content::builder().schema(Some(Ref::new( "#/components/schemas/serveradmin.innogames.de.v1.DatacenterType"))).build()).build(),
                )
                .build(),
        )
        .operation(
            utoipa::openapi::HttpMethod::Patch,
            Operation::builder()
                .request_body(Some(RequestBody::builder().content("application/json", Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.DatacenterType"))).build()).build(),))
                .response(
                    "200",
                    Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.DatacenterType"))).build()).build(),
                )
                .build(),
        )
        .operation(
            utoipa::openapi::HttpMethod::Put,
            Operation::builder()
                .request_body(Some(RequestBody::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.DatacenterType"))).build()).build()))
                .response(
                    "200",
                    Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.DatacenterType"))).build()).build(),
                )
                .build(),
        )
        .build(),
)
    

.path(
    "/apis/serveradmin.innogames.de/v1/namespace/{namespace}/hwloadbalancergroups",
    PathItem::builder()
        .parameters(Some(vec![
            Parameter::builder().name("namespace").schema(Some("string")).parameter_in(ParameterIn::Path).build(),
        ]))
        .operation(utoipa::openapi::HttpMethod::Get, Operation::builder()
            .response("200", Response::builder()
                .content("application/json", Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.HwLoadbalancerGroupList"))).build())
                .build(),
            )
        .build())
        .operation(utoipa::openapi::HttpMethod::Post, Operation::builder()
            .request_body(Some(RequestBody::builder().content("application/json", Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.HwLoadbalancerGroup"))).build()).build()))
            .response(
                "200",
                Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.HwLoadbalancerGroupList"))).build()).build(),
            )
            .build(),
        )
        .build(),
)
.path(
    "/apis/serveradmin.innogames.de/v1/namespace/{namespace}/hwloadbalancergroups/{name}",
    PathItem::builder()
        .parameters(Some(vec![
            Parameter::builder().name("namespace").schema(Some("string")).parameter_in(ParameterIn::Path).build(),
            Parameter::builder().name("name").parameter_in(ParameterIn::Path).schema(Some("string")).build(),
        ]))
        .operation(
            utoipa::openapi::HttpMethod::Get,
            Operation::builder()
                .response(
                    "200",
                    Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.HwLoadbalancerGroup"))).build()).build(),
                )
                .build(),
        )
        .operation(
            utoipa::openapi::HttpMethod::Delete,
            Operation::builder()
                .response(
                    "200",
                    Response::builder().content( "application/json", Content::builder().schema(Some(Ref::new( "#/components/schemas/serveradmin.innogames.de.v1.HwLoadbalancerGroup"))).build()).build(),
                )
                .build(),
        )
        .operation(
            utoipa::openapi::HttpMethod::Patch,
            Operation::builder()
                .request_body(Some(RequestBody::builder().content("application/json", Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.HwLoadbalancerGroup"))).build()).build(),))
                .response(
                    "200",
                    Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.HwLoadbalancerGroup"))).build()).build(),
                )
                .build(),
        )
        .operation(
            utoipa::openapi::HttpMethod::Put,
            Operation::builder()
                .request_body(Some(RequestBody::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.HwLoadbalancerGroup"))).build()).build()))
                .response(
                    "200",
                    Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.HwLoadbalancerGroup"))).build()).build(),
                )
                .build(),
        )
        .build(),
)
    

.path(
    "/apis/serveradmin.innogames.de/v1/namespace/{namespace}/routers",
    PathItem::builder()
        .parameters(Some(vec![
            Parameter::builder().name("namespace").schema(Some("string")).parameter_in(ParameterIn::Path).build(),
        ]))
        .operation(utoipa::openapi::HttpMethod::Get, Operation::builder()
            .response("200", Response::builder()
                .content("application/json", Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.RouterList"))).build())
                .build(),
            )
        .build())
        .operation(utoipa::openapi::HttpMethod::Post, Operation::builder()
            .request_body(Some(RequestBody::builder().content("application/json", Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.Router"))).build()).build()))
            .response(
                "200",
                Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.RouterList"))).build()).build(),
            )
            .build(),
        )
        .build(),
)
.path(
    "/apis/serveradmin.innogames.de/v1/namespace/{namespace}/routers/{name}",
    PathItem::builder()
        .parameters(Some(vec![
            Parameter::builder().name("namespace").schema(Some("string")).parameter_in(ParameterIn::Path).build(),
            Parameter::builder().name("name").parameter_in(ParameterIn::Path).schema(Some("string")).build(),
        ]))
        .operation(
            utoipa::openapi::HttpMethod::Get,
            Operation::builder()
                .response(
                    "200",
                    Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.Router"))).build()).build(),
                )
                .build(),
        )
        .operation(
            utoipa::openapi::HttpMethod::Delete,
            Operation::builder()
                .response(
                    "200",
                    Response::builder().content( "application/json", Content::builder().schema(Some(Ref::new( "#/components/schemas/serveradmin.innogames.de.v1.Router"))).build()).build(),
                )
                .build(),
        )
        .operation(
            utoipa::openapi::HttpMethod::Patch,
            Operation::builder()
                .request_body(Some(RequestBody::builder().content("application/json", Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.Router"))).build()).build(),))
                .response(
                    "200",
                    Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.Router"))).build()).build(),
                )
                .build(),
        )
        .operation(
            utoipa::openapi::HttpMethod::Put,
            Operation::builder()
                .request_body(Some(RequestBody::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.Router"))).build()).build()))
                .response(
                    "200",
                    Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.Router"))).build()).build(),
                )
                .build(),
        )
        .build(),
)
    

.path(
    "/apis/serveradmin.innogames.de/v1/namespace/{namespace}/everythings",
    PathItem::builder()
        .parameters(Some(vec![
            Parameter::builder().name("namespace").schema(Some("string")).parameter_in(ParameterIn::Path).build(),
        ]))
        .operation(utoipa::openapi::HttpMethod::Get, Operation::builder()
            .response("200", Response::builder()
                .content("application/json", Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.EverythingList"))).build())
                .build(),
            )
        .build())
        .operation(utoipa::openapi::HttpMethod::Post, Operation::builder()
            .request_body(Some(RequestBody::builder().content("application/json", Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.Everything"))).build()).build()))
            .response(
                "200",
                Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.EverythingList"))).build()).build(),
            )
            .build(),
        )
        .build(),
)
.path(
    "/apis/serveradmin.innogames.de/v1/namespace/{namespace}/everythings/{name}",
    PathItem::builder()
        .parameters(Some(vec![
            Parameter::builder().name("namespace").schema(Some("string")).parameter_in(ParameterIn::Path).build(),
            Parameter::builder().name("name").parameter_in(ParameterIn::Path).schema(Some("string")).build(),
        ]))
        .operation(
            utoipa::openapi::HttpMethod::Get,
            Operation::builder()
                .response(
                    "200",
                    Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.Everything"))).build()).build(),
                )
                .build(),
        )
        .operation(
            utoipa::openapi::HttpMethod::Delete,
            Operation::builder()
                .response(
                    "200",
                    Response::builder().content( "application/json", Content::builder().schema(Some(Ref::new( "#/components/schemas/serveradmin.innogames.de.v1.Everything"))).build()).build(),
                )
                .build(),
        )
        .operation(
            utoipa::openapi::HttpMethod::Patch,
            Operation::builder()
                .request_body(Some(RequestBody::builder().content("application/json", Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.Everything"))).build()).build(),))
                .response(
                    "200",
                    Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.Everything"))).build()).build(),
                )
                .build(),
        )
        .operation(
            utoipa::openapi::HttpMethod::Put,
            Operation::builder()
                .request_body(Some(RequestBody::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.Everything"))).build()).build()))
                .response(
                    "200",
                    Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.Everything"))).build()).build(),
                )
                .build(),
        )
        .build(),
)
    

.path(
    "/apis/serveradmin.innogames.de/v1/namespace/{namespace}/doorlocks",
    PathItem::builder()
        .parameters(Some(vec![
            Parameter::builder().name("namespace").schema(Some("string")).parameter_in(ParameterIn::Path).build(),
        ]))
        .operation(utoipa::openapi::HttpMethod::Get, Operation::builder()
            .response("200", Response::builder()
                .content("application/json", Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.DoorlockList"))).build())
                .build(),
            )
        .build())
        .operation(utoipa::openapi::HttpMethod::Post, Operation::builder()
            .request_body(Some(RequestBody::builder().content("application/json", Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.Doorlock"))).build()).build()))
            .response(
                "200",
                Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.DoorlockList"))).build()).build(),
            )
            .build(),
        )
        .build(),
)
.path(
    "/apis/serveradmin.innogames.de/v1/namespace/{namespace}/doorlocks/{name}",
    PathItem::builder()
        .parameters(Some(vec![
            Parameter::builder().name("namespace").schema(Some("string")).parameter_in(ParameterIn::Path).build(),
            Parameter::builder().name("name").parameter_in(ParameterIn::Path).schema(Some("string")).build(),
        ]))
        .operation(
            utoipa::openapi::HttpMethod::Get,
            Operation::builder()
                .response(
                    "200",
                    Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.Doorlock"))).build()).build(),
                )
                .build(),
        )
        .operation(
            utoipa::openapi::HttpMethod::Delete,
            Operation::builder()
                .response(
                    "200",
                    Response::builder().content( "application/json", Content::builder().schema(Some(Ref::new( "#/components/schemas/serveradmin.innogames.de.v1.Doorlock"))).build()).build(),
                )
                .build(),
        )
        .operation(
            utoipa::openapi::HttpMethod::Patch,
            Operation::builder()
                .request_body(Some(RequestBody::builder().content("application/json", Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.Doorlock"))).build()).build(),))
                .response(
                    "200",
                    Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.Doorlock"))).build()).build(),
                )
                .build(),
        )
        .operation(
            utoipa::openapi::HttpMethod::Put,
            Operation::builder()
                .request_body(Some(RequestBody::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.Doorlock"))).build()).build()))
                .response(
                    "200",
                    Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.Doorlock"))).build()).build(),
                )
                .build(),
        )
        .build(),
)
    

.path(
    "/apis/serveradmin.innogames.de/v1/namespace/{namespace}/providernetworks",
    PathItem::builder()
        .parameters(Some(vec![
            Parameter::builder().name("namespace").schema(Some("string")).parameter_in(ParameterIn::Path).build(),
        ]))
        .operation(utoipa::openapi::HttpMethod::Get, Operation::builder()
            .response("200", Response::builder()
                .content("application/json", Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.ProviderNetworkList"))).build())
                .build(),
            )
        .build())
        .operation(utoipa::openapi::HttpMethod::Post, Operation::builder()
            .request_body(Some(RequestBody::builder().content("application/json", Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.ProviderNetwork"))).build()).build()))
            .response(
                "200",
                Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.ProviderNetworkList"))).build()).build(),
            )
            .build(),
        )
        .build(),
)
.path(
    "/apis/serveradmin.innogames.de/v1/namespace/{namespace}/providernetworks/{name}",
    PathItem::builder()
        .parameters(Some(vec![
            Parameter::builder().name("namespace").schema(Some("string")).parameter_in(ParameterIn::Path).build(),
            Parameter::builder().name("name").parameter_in(ParameterIn::Path).schema(Some("string")).build(),
        ]))
        .operation(
            utoipa::openapi::HttpMethod::Get,
            Operation::builder()
                .response(
                    "200",
                    Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.ProviderNetwork"))).build()).build(),
                )
                .build(),
        )
        .operation(
            utoipa::openapi::HttpMethod::Delete,
            Operation::builder()
                .response(
                    "200",
                    Response::builder().content( "application/json", Content::builder().schema(Some(Ref::new( "#/components/schemas/serveradmin.innogames.de.v1.ProviderNetwork"))).build()).build(),
                )
                .build(),
        )
        .operation(
            utoipa::openapi::HttpMethod::Patch,
            Operation::builder()
                .request_body(Some(RequestBody::builder().content("application/json", Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.ProviderNetwork"))).build()).build(),))
                .response(
                    "200",
                    Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.ProviderNetwork"))).build()).build(),
                )
                .build(),
        )
        .operation(
            utoipa::openapi::HttpMethod::Put,
            Operation::builder()
                .request_body(Some(RequestBody::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.ProviderNetwork"))).build()).build()))
                .response(
                    "200",
                    Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.ProviderNetwork"))).build()).build(),
                )
                .build(),
        )
        .build(),
)
    

.path(
    "/apis/serveradmin.innogames.de/v1/namespace//monitoringgroups",
    PathItem::builder()
        .parameters(Some(vec![
            
        ]))
        .operation(utoipa::openapi::HttpMethod::Get, Operation::builder()
            .response("200", Response::builder()
                .content("application/json", Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.MonitoringGroupList"))).build())
                .build(),
            )
        .build())
        .operation(utoipa::openapi::HttpMethod::Post, Operation::builder()
            .request_body(Some(RequestBody::builder().content("application/json", Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.MonitoringGroup"))).build()).build()))
            .response(
                "200",
                Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.MonitoringGroupList"))).build()).build(),
            )
            .build(),
        )
        .build(),
)
.path(
    "/apis/serveradmin.innogames.de/v1/namespace//monitoringgroups/{name}",
    PathItem::builder()
        .parameters(Some(vec![
            
            Parameter::builder().name("name").parameter_in(ParameterIn::Path).schema(Some("string")).build(),
        ]))
        .operation(
            utoipa::openapi::HttpMethod::Get,
            Operation::builder()
                .response(
                    "200",
                    Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.MonitoringGroup"))).build()).build(),
                )
                .build(),
        )
        .operation(
            utoipa::openapi::HttpMethod::Delete,
            Operation::builder()
                .response(
                    "200",
                    Response::builder().content( "application/json", Content::builder().schema(Some(Ref::new( "#/components/schemas/serveradmin.innogames.de.v1.MonitoringGroup"))).build()).build(),
                )
                .build(),
        )
        .operation(
            utoipa::openapi::HttpMethod::Patch,
            Operation::builder()
                .request_body(Some(RequestBody::builder().content("application/json", Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.MonitoringGroup"))).build()).build(),))
                .response(
                    "200",
                    Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.MonitoringGroup"))).build()).build(),
                )
                .build(),
        )
        .operation(
            utoipa::openapi::HttpMethod::Put,
            Operation::builder()
                .request_body(Some(RequestBody::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.MonitoringGroup"))).build()).build()))
                .response(
                    "200",
                    Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.MonitoringGroup"))).build()).build(),
                )
                .build(),
        )
        .build(),
)
    

.path(
    "/apis/serveradmin.innogames.de/v1/namespace/{namespace}/vlaninterfaces",
    PathItem::builder()
        .parameters(Some(vec![
            Parameter::builder().name("namespace").schema(Some("string")).parameter_in(ParameterIn::Path).build(),
        ]))
        .operation(utoipa::openapi::HttpMethod::Get, Operation::builder()
            .response("200", Response::builder()
                .content("application/json", Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.VlanInterfaceList"))).build())
                .build(),
            )
        .build())
        .operation(utoipa::openapi::HttpMethod::Post, Operation::builder()
            .request_body(Some(RequestBody::builder().content("application/json", Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.VlanInterface"))).build()).build()))
            .response(
                "200",
                Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.VlanInterfaceList"))).build()).build(),
            )
            .build(),
        )
        .build(),
)
.path(
    "/apis/serveradmin.innogames.de/v1/namespace/{namespace}/vlaninterfaces/{name}",
    PathItem::builder()
        .parameters(Some(vec![
            Parameter::builder().name("namespace").schema(Some("string")).parameter_in(ParameterIn::Path).build(),
            Parameter::builder().name("name").parameter_in(ParameterIn::Path).schema(Some("string")).build(),
        ]))
        .operation(
            utoipa::openapi::HttpMethod::Get,
            Operation::builder()
                .response(
                    "200",
                    Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.VlanInterface"))).build()).build(),
                )
                .build(),
        )
        .operation(
            utoipa::openapi::HttpMethod::Delete,
            Operation::builder()
                .response(
                    "200",
                    Response::builder().content( "application/json", Content::builder().schema(Some(Ref::new( "#/components/schemas/serveradmin.innogames.de.v1.VlanInterface"))).build()).build(),
                )
                .build(),
        )
        .operation(
            utoipa::openapi::HttpMethod::Patch,
            Operation::builder()
                .request_body(Some(RequestBody::builder().content("application/json", Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.VlanInterface"))).build()).build(),))
                .response(
                    "200",
                    Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.VlanInterface"))).build()).build(),
                )
                .build(),
        )
        .operation(
            utoipa::openapi::HttpMethod::Put,
            Operation::builder()
                .request_body(Some(RequestBody::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.VlanInterface"))).build()).build()))
                .response(
                    "200",
                    Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.VlanInterface"))).build()).build(),
                )
                .build(),
        )
        .build(),
)
    

.path(
    "/apis/serveradmin.innogames.de/v1/namespace/{namespace}/externaldomains",
    PathItem::builder()
        .parameters(Some(vec![
            Parameter::builder().name("namespace").schema(Some("string")).parameter_in(ParameterIn::Path).build(),
        ]))
        .operation(utoipa::openapi::HttpMethod::Get, Operation::builder()
            .response("200", Response::builder()
                .content("application/json", Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.ExternalDomainList"))).build())
                .build(),
            )
        .build())
        .operation(utoipa::openapi::HttpMethod::Post, Operation::builder()
            .request_body(Some(RequestBody::builder().content("application/json", Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.ExternalDomain"))).build()).build()))
            .response(
                "200",
                Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.ExternalDomainList"))).build()).build(),
            )
            .build(),
        )
        .build(),
)
.path(
    "/apis/serveradmin.innogames.de/v1/namespace/{namespace}/externaldomains/{name}",
    PathItem::builder()
        .parameters(Some(vec![
            Parameter::builder().name("namespace").schema(Some("string")).parameter_in(ParameterIn::Path).build(),
            Parameter::builder().name("name").parameter_in(ParameterIn::Path).schema(Some("string")).build(),
        ]))
        .operation(
            utoipa::openapi::HttpMethod::Get,
            Operation::builder()
                .response(
                    "200",
                    Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.ExternalDomain"))).build()).build(),
                )
                .build(),
        )
        .operation(
            utoipa::openapi::HttpMethod::Delete,
            Operation::builder()
                .response(
                    "200",
                    Response::builder().content( "application/json", Content::builder().schema(Some(Ref::new( "#/components/schemas/serveradmin.innogames.de.v1.ExternalDomain"))).build()).build(),
                )
                .build(),
        )
        .operation(
            utoipa::openapi::HttpMethod::Patch,
            Operation::builder()
                .request_body(Some(RequestBody::builder().content("application/json", Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.ExternalDomain"))).build()).build(),))
                .response(
                    "200",
                    Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.ExternalDomain"))).build()).build(),
                )
                .build(),
        )
        .operation(
            utoipa::openapi::HttpMethod::Put,
            Operation::builder()
                .request_body(Some(RequestBody::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.ExternalDomain"))).build()).build()))
                .response(
                    "200",
                    Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.ExternalDomain"))).build()).build(),
                )
                .build(),
        )
        .build(),
)
    

.path(
    "/apis/serveradmin.innogames.de/v1/namespace//providers",
    PathItem::builder()
        .parameters(Some(vec![
            
        ]))
        .operation(utoipa::openapi::HttpMethod::Get, Operation::builder()
            .response("200", Response::builder()
                .content("application/json", Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.ProviderList"))).build())
                .build(),
            )
        .build())
        .operation(utoipa::openapi::HttpMethod::Post, Operation::builder()
            .request_body(Some(RequestBody::builder().content("application/json", Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.Provider"))).build()).build()))
            .response(
                "200",
                Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.ProviderList"))).build()).build(),
            )
            .build(),
        )
        .build(),
)
.path(
    "/apis/serveradmin.innogames.de/v1/namespace//providers/{name}",
    PathItem::builder()
        .parameters(Some(vec![
            
            Parameter::builder().name("name").parameter_in(ParameterIn::Path).schema(Some("string")).build(),
        ]))
        .operation(
            utoipa::openapi::HttpMethod::Get,
            Operation::builder()
                .response(
                    "200",
                    Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.Provider"))).build()).build(),
                )
                .build(),
        )
        .operation(
            utoipa::openapi::HttpMethod::Delete,
            Operation::builder()
                .response(
                    "200",
                    Response::builder().content( "application/json", Content::builder().schema(Some(Ref::new( "#/components/schemas/serveradmin.innogames.de.v1.Provider"))).build()).build(),
                )
                .build(),
        )
        .operation(
            utoipa::openapi::HttpMethod::Patch,
            Operation::builder()
                .request_body(Some(RequestBody::builder().content("application/json", Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.Provider"))).build()).build(),))
                .response(
                    "200",
                    Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.Provider"))).build()).build(),
                )
                .build(),
        )
        .operation(
            utoipa::openapi::HttpMethod::Put,
            Operation::builder()
                .request_body(Some(RequestBody::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.Provider"))).build()).build()))
                .response(
                    "200",
                    Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.Provider"))).build()).build(),
                )
                .build(),
        )
        .build(),
)
    

.path(
    "/apis/serveradmin.innogames.de/v1/namespace/{namespace}/healthchecks",
    PathItem::builder()
        .parameters(Some(vec![
            Parameter::builder().name("namespace").schema(Some("string")).parameter_in(ParameterIn::Path).build(),
        ]))
        .operation(utoipa::openapi::HttpMethod::Get, Operation::builder()
            .response("200", Response::builder()
                .content("application/json", Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.HealthCheckList"))).build())
                .build(),
            )
        .build())
        .operation(utoipa::openapi::HttpMethod::Post, Operation::builder()
            .request_body(Some(RequestBody::builder().content("application/json", Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.HealthCheck"))).build()).build()))
            .response(
                "200",
                Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.HealthCheckList"))).build()).build(),
            )
            .build(),
        )
        .build(),
)
.path(
    "/apis/serveradmin.innogames.de/v1/namespace/{namespace}/healthchecks/{name}",
    PathItem::builder()
        .parameters(Some(vec![
            Parameter::builder().name("namespace").schema(Some("string")).parameter_in(ParameterIn::Path).build(),
            Parameter::builder().name("name").parameter_in(ParameterIn::Path).schema(Some("string")).build(),
        ]))
        .operation(
            utoipa::openapi::HttpMethod::Get,
            Operation::builder()
                .response(
                    "200",
                    Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.HealthCheck"))).build()).build(),
                )
                .build(),
        )
        .operation(
            utoipa::openapi::HttpMethod::Delete,
            Operation::builder()
                .response(
                    "200",
                    Response::builder().content( "application/json", Content::builder().schema(Some(Ref::new( "#/components/schemas/serveradmin.innogames.de.v1.HealthCheck"))).build()).build(),
                )
                .build(),
        )
        .operation(
            utoipa::openapi::HttpMethod::Patch,
            Operation::builder()
                .request_body(Some(RequestBody::builder().content("application/json", Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.HealthCheck"))).build()).build(),))
                .response(
                    "200",
                    Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.HealthCheck"))).build()).build(),
                )
                .build(),
        )
        .operation(
            utoipa::openapi::HttpMethod::Put,
            Operation::builder()
                .request_body(Some(RequestBody::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.HealthCheck"))).build()).build()))
                .response(
                    "200",
                    Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.HealthCheck"))).build()).build(),
                )
                .build(),
        )
        .build(),
)
    

.path(
    "/apis/serveradmin.innogames.de/v1/namespace//storagemodules",
    PathItem::builder()
        .parameters(Some(vec![
            
        ]))
        .operation(utoipa::openapi::HttpMethod::Get, Operation::builder()
            .response("200", Response::builder()
                .content("application/json", Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.StorageModuleList"))).build())
                .build(),
            )
        .build())
        .operation(utoipa::openapi::HttpMethod::Post, Operation::builder()
            .request_body(Some(RequestBody::builder().content("application/json", Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.StorageModule"))).build()).build()))
            .response(
                "200",
                Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.StorageModuleList"))).build()).build(),
            )
            .build(),
        )
        .build(),
)
.path(
    "/apis/serveradmin.innogames.de/v1/namespace//storagemodules/{name}",
    PathItem::builder()
        .parameters(Some(vec![
            
            Parameter::builder().name("name").parameter_in(ParameterIn::Path).schema(Some("string")).build(),
        ]))
        .operation(
            utoipa::openapi::HttpMethod::Get,
            Operation::builder()
                .response(
                    "200",
                    Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.StorageModule"))).build()).build(),
                )
                .build(),
        )
        .operation(
            utoipa::openapi::HttpMethod::Delete,
            Operation::builder()
                .response(
                    "200",
                    Response::builder().content( "application/json", Content::builder().schema(Some(Ref::new( "#/components/schemas/serveradmin.innogames.de.v1.StorageModule"))).build()).build(),
                )
                .build(),
        )
        .operation(
            utoipa::openapi::HttpMethod::Patch,
            Operation::builder()
                .request_body(Some(RequestBody::builder().content("application/json", Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.StorageModule"))).build()).build(),))
                .response(
                    "200",
                    Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.StorageModule"))).build()).build(),
                )
                .build(),
        )
        .operation(
            utoipa::openapi::HttpMethod::Put,
            Operation::builder()
                .request_body(Some(RequestBody::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.StorageModule"))).build()).build()))
                .response(
                    "200",
                    Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.StorageModule"))).build()).build(),
                )
                .build(),
        )
        .build(),
)
    

.path(
    "/apis/serveradmin.innogames.de/v1/namespace/{namespace}/bladeenclosures",
    PathItem::builder()
        .parameters(Some(vec![
            Parameter::builder().name("namespace").schema(Some("string")).parameter_in(ParameterIn::Path).build(),
        ]))
        .operation(utoipa::openapi::HttpMethod::Get, Operation::builder()
            .response("200", Response::builder()
                .content("application/json", Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.BladeEnclosureList"))).build())
                .build(),
            )
        .build())
        .operation(utoipa::openapi::HttpMethod::Post, Operation::builder()
            .request_body(Some(RequestBody::builder().content("application/json", Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.BladeEnclosure"))).build()).build()))
            .response(
                "200",
                Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.BladeEnclosureList"))).build()).build(),
            )
            .build(),
        )
        .build(),
)
.path(
    "/apis/serveradmin.innogames.de/v1/namespace/{namespace}/bladeenclosures/{name}",
    PathItem::builder()
        .parameters(Some(vec![
            Parameter::builder().name("namespace").schema(Some("string")).parameter_in(ParameterIn::Path).build(),
            Parameter::builder().name("name").parameter_in(ParameterIn::Path).schema(Some("string")).build(),
        ]))
        .operation(
            utoipa::openapi::HttpMethod::Get,
            Operation::builder()
                .response(
                    "200",
                    Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.BladeEnclosure"))).build()).build(),
                )
                .build(),
        )
        .operation(
            utoipa::openapi::HttpMethod::Delete,
            Operation::builder()
                .response(
                    "200",
                    Response::builder().content( "application/json", Content::builder().schema(Some(Ref::new( "#/components/schemas/serveradmin.innogames.de.v1.BladeEnclosure"))).build()).build(),
                )
                .build(),
        )
        .operation(
            utoipa::openapi::HttpMethod::Patch,
            Operation::builder()
                .request_body(Some(RequestBody::builder().content("application/json", Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.BladeEnclosure"))).build()).build(),))
                .response(
                    "200",
                    Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.BladeEnclosure"))).build()).build(),
                )
                .build(),
        )
        .operation(
            utoipa::openapi::HttpMethod::Put,
            Operation::builder()
                .request_body(Some(RequestBody::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.BladeEnclosure"))).build()).build()))
                .response(
                    "200",
                    Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.BladeEnclosure"))).build()).build(),
                )
                .build(),
        )
        .build(),
)
    

.path(
    "/apis/serveradmin.innogames.de/v1/namespace/{namespace}/tunnelinterfaces",
    PathItem::builder()
        .parameters(Some(vec![
            Parameter::builder().name("namespace").schema(Some("string")).parameter_in(ParameterIn::Path).build(),
        ]))
        .operation(utoipa::openapi::HttpMethod::Get, Operation::builder()
            .response("200", Response::builder()
                .content("application/json", Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.TunnelInterfaceList"))).build())
                .build(),
            )
        .build())
        .operation(utoipa::openapi::HttpMethod::Post, Operation::builder()
            .request_body(Some(RequestBody::builder().content("application/json", Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.TunnelInterface"))).build()).build()))
            .response(
                "200",
                Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.TunnelInterfaceList"))).build()).build(),
            )
            .build(),
        )
        .build(),
)
.path(
    "/apis/serveradmin.innogames.de/v1/namespace/{namespace}/tunnelinterfaces/{name}",
    PathItem::builder()
        .parameters(Some(vec![
            Parameter::builder().name("namespace").schema(Some("string")).parameter_in(ParameterIn::Path).build(),
            Parameter::builder().name("name").parameter_in(ParameterIn::Path).schema(Some("string")).build(),
        ]))
        .operation(
            utoipa::openapi::HttpMethod::Get,
            Operation::builder()
                .response(
                    "200",
                    Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.TunnelInterface"))).build()).build(),
                )
                .build(),
        )
        .operation(
            utoipa::openapi::HttpMethod::Delete,
            Operation::builder()
                .response(
                    "200",
                    Response::builder().content( "application/json", Content::builder().schema(Some(Ref::new( "#/components/schemas/serveradmin.innogames.de.v1.TunnelInterface"))).build()).build(),
                )
                .build(),
        )
        .operation(
            utoipa::openapi::HttpMethod::Patch,
            Operation::builder()
                .request_body(Some(RequestBody::builder().content("application/json", Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.TunnelInterface"))).build()).build(),))
                .response(
                    "200",
                    Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.TunnelInterface"))).build()).build(),
                )
                .build(),
        )
        .operation(
            utoipa::openapi::HttpMethod::Put,
            Operation::builder()
                .request_body(Some(RequestBody::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.TunnelInterface"))).build()).build()))
                .response(
                    "200",
                    Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.TunnelInterface"))).build()).build(),
                )
                .build(),
        )
        .build(),
)
    

.path(
    "/apis/serveradmin.innogames.de/v1/namespace/{namespace}/routenetworks",
    PathItem::builder()
        .parameters(Some(vec![
            Parameter::builder().name("namespace").schema(Some("string")).parameter_in(ParameterIn::Path).build(),
        ]))
        .operation(utoipa::openapi::HttpMethod::Get, Operation::builder()
            .response("200", Response::builder()
                .content("application/json", Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.RouteNetworkList"))).build())
                .build(),
            )
        .build())
        .operation(utoipa::openapi::HttpMethod::Post, Operation::builder()
            .request_body(Some(RequestBody::builder().content("application/json", Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.RouteNetwork"))).build()).build()))
            .response(
                "200",
                Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.RouteNetworkList"))).build()).build(),
            )
            .build(),
        )
        .build(),
)
.path(
    "/apis/serveradmin.innogames.de/v1/namespace/{namespace}/routenetworks/{name}",
    PathItem::builder()
        .parameters(Some(vec![
            Parameter::builder().name("namespace").schema(Some("string")).parameter_in(ParameterIn::Path).build(),
            Parameter::builder().name("name").parameter_in(ParameterIn::Path).schema(Some("string")).build(),
        ]))
        .operation(
            utoipa::openapi::HttpMethod::Get,
            Operation::builder()
                .response(
                    "200",
                    Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.RouteNetwork"))).build()).build(),
                )
                .build(),
        )
        .operation(
            utoipa::openapi::HttpMethod::Delete,
            Operation::builder()
                .response(
                    "200",
                    Response::builder().content( "application/json", Content::builder().schema(Some(Ref::new( "#/components/schemas/serveradmin.innogames.de.v1.RouteNetwork"))).build()).build(),
                )
                .build(),
        )
        .operation(
            utoipa::openapi::HttpMethod::Patch,
            Operation::builder()
                .request_body(Some(RequestBody::builder().content("application/json", Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.RouteNetwork"))).build()).build(),))
                .response(
                    "200",
                    Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.RouteNetwork"))).build()).build(),
                )
                .build(),
        )
        .operation(
            utoipa::openapi::HttpMethod::Put,
            Operation::builder()
                .request_body(Some(RequestBody::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.RouteNetwork"))).build()).build()))
                .response(
                    "200",
                    Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.RouteNetwork"))).build()).build(),
                )
                .build(),
        )
        .build(),
)
    

.path(
    "/apis/serveradmin.innogames.de/v1/namespace/{namespace}/datacenters",
    PathItem::builder()
        .parameters(Some(vec![
            Parameter::builder().name("namespace").schema(Some("string")).parameter_in(ParameterIn::Path).build(),
        ]))
        .operation(utoipa::openapi::HttpMethod::Get, Operation::builder()
            .response("200", Response::builder()
                .content("application/json", Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.DatacenterList"))).build())
                .build(),
            )
        .build())
        .operation(utoipa::openapi::HttpMethod::Post, Operation::builder()
            .request_body(Some(RequestBody::builder().content("application/json", Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.Datacenter"))).build()).build()))
            .response(
                "200",
                Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.DatacenterList"))).build()).build(),
            )
            .build(),
        )
        .build(),
)
.path(
    "/apis/serveradmin.innogames.de/v1/namespace/{namespace}/datacenters/{name}",
    PathItem::builder()
        .parameters(Some(vec![
            Parameter::builder().name("namespace").schema(Some("string")).parameter_in(ParameterIn::Path).build(),
            Parameter::builder().name("name").parameter_in(ParameterIn::Path).schema(Some("string")).build(),
        ]))
        .operation(
            utoipa::openapi::HttpMethod::Get,
            Operation::builder()
                .response(
                    "200",
                    Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.Datacenter"))).build()).build(),
                )
                .build(),
        )
        .operation(
            utoipa::openapi::HttpMethod::Delete,
            Operation::builder()
                .response(
                    "200",
                    Response::builder().content( "application/json", Content::builder().schema(Some(Ref::new( "#/components/schemas/serveradmin.innogames.de.v1.Datacenter"))).build()).build(),
                )
                .build(),
        )
        .operation(
            utoipa::openapi::HttpMethod::Patch,
            Operation::builder()
                .request_body(Some(RequestBody::builder().content("application/json", Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.Datacenter"))).build()).build(),))
                .response(
                    "200",
                    Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.Datacenter"))).build()).build(),
                )
                .build(),
        )
        .operation(
            utoipa::openapi::HttpMethod::Put,
            Operation::builder()
                .request_body(Some(RequestBody::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.Datacenter"))).build()).build()))
                .response(
                    "200",
                    Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.Datacenter"))).build()).build(),
                )
                .build(),
        )
        .build(),
)
    

.path(
    "/apis/serveradmin.innogames.de/v1/namespace/{namespace}/monitoringqueues",
    PathItem::builder()
        .parameters(Some(vec![
            Parameter::builder().name("namespace").schema(Some("string")).parameter_in(ParameterIn::Path).build(),
        ]))
        .operation(utoipa::openapi::HttpMethod::Get, Operation::builder()
            .response("200", Response::builder()
                .content("application/json", Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.MonitoringQueueList"))).build())
                .build(),
            )
        .build())
        .operation(utoipa::openapi::HttpMethod::Post, Operation::builder()
            .request_body(Some(RequestBody::builder().content("application/json", Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.MonitoringQueue"))).build()).build()))
            .response(
                "200",
                Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.MonitoringQueueList"))).build()).build(),
            )
            .build(),
        )
        .build(),
)
.path(
    "/apis/serveradmin.innogames.de/v1/namespace/{namespace}/monitoringqueues/{name}",
    PathItem::builder()
        .parameters(Some(vec![
            Parameter::builder().name("namespace").schema(Some("string")).parameter_in(ParameterIn::Path).build(),
            Parameter::builder().name("name").parameter_in(ParameterIn::Path).schema(Some("string")).build(),
        ]))
        .operation(
            utoipa::openapi::HttpMethod::Get,
            Operation::builder()
                .response(
                    "200",
                    Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.MonitoringQueue"))).build()).build(),
                )
                .build(),
        )
        .operation(
            utoipa::openapi::HttpMethod::Delete,
            Operation::builder()
                .response(
                    "200",
                    Response::builder().content( "application/json", Content::builder().schema(Some(Ref::new( "#/components/schemas/serveradmin.innogames.de.v1.MonitoringQueue"))).build()).build(),
                )
                .build(),
        )
        .operation(
            utoipa::openapi::HttpMethod::Patch,
            Operation::builder()
                .request_body(Some(RequestBody::builder().content("application/json", Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.MonitoringQueue"))).build()).build(),))
                .response(
                    "200",
                    Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.MonitoringQueue"))).build()).build(),
                )
                .build(),
        )
        .operation(
            utoipa::openapi::HttpMethod::Put,
            Operation::builder()
                .request_body(Some(RequestBody::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.MonitoringQueue"))).build()).build()))
                .response(
                    "200",
                    Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.MonitoringQueue"))).build()).build(),
                )
                .build(),
        )
        .build(),
)
    

.path(
    "/apis/serveradmin.innogames.de/v1/namespace/{namespace}/natgateways",
    PathItem::builder()
        .parameters(Some(vec![
            Parameter::builder().name("namespace").schema(Some("string")).parameter_in(ParameterIn::Path).build(),
        ]))
        .operation(utoipa::openapi::HttpMethod::Get, Operation::builder()
            .response("200", Response::builder()
                .content("application/json", Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.NatGatewayList"))).build())
                .build(),
            )
        .build())
        .operation(utoipa::openapi::HttpMethod::Post, Operation::builder()
            .request_body(Some(RequestBody::builder().content("application/json", Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.NatGateway"))).build()).build()))
            .response(
                "200",
                Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.NatGatewayList"))).build()).build(),
            )
            .build(),
        )
        .build(),
)
.path(
    "/apis/serveradmin.innogames.de/v1/namespace/{namespace}/natgateways/{name}",
    PathItem::builder()
        .parameters(Some(vec![
            Parameter::builder().name("namespace").schema(Some("string")).parameter_in(ParameterIn::Path).build(),
            Parameter::builder().name("name").parameter_in(ParameterIn::Path).schema(Some("string")).build(),
        ]))
        .operation(
            utoipa::openapi::HttpMethod::Get,
            Operation::builder()
                .response(
                    "200",
                    Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.NatGateway"))).build()).build(),
                )
                .build(),
        )
        .operation(
            utoipa::openapi::HttpMethod::Delete,
            Operation::builder()
                .response(
                    "200",
                    Response::builder().content( "application/json", Content::builder().schema(Some(Ref::new( "#/components/schemas/serveradmin.innogames.de.v1.NatGateway"))).build()).build(),
                )
                .build(),
        )
        .operation(
            utoipa::openapi::HttpMethod::Patch,
            Operation::builder()
                .request_body(Some(RequestBody::builder().content("application/json", Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.NatGateway"))).build()).build(),))
                .response(
                    "200",
                    Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.NatGateway"))).build()).build(),
                )
                .build(),
        )
        .operation(
            utoipa::openapi::HttpMethod::Put,
            Operation::builder()
                .request_body(Some(RequestBody::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.NatGateway"))).build()).build()))
                .response(
                    "200",
                    Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.NatGateway"))).build()).build(),
                )
                .build(),
        )
        .build(),
)
    

.path(
    "/apis/serveradmin.innogames.de/v1/namespace/{namespace}/mediadevices",
    PathItem::builder()
        .parameters(Some(vec![
            Parameter::builder().name("namespace").schema(Some("string")).parameter_in(ParameterIn::Path).build(),
        ]))
        .operation(utoipa::openapi::HttpMethod::Get, Operation::builder()
            .response("200", Response::builder()
                .content("application/json", Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.MediaDeviceList"))).build())
                .build(),
            )
        .build())
        .operation(utoipa::openapi::HttpMethod::Post, Operation::builder()
            .request_body(Some(RequestBody::builder().content("application/json", Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.MediaDevice"))).build()).build()))
            .response(
                "200",
                Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.MediaDeviceList"))).build()).build(),
            )
            .build(),
        )
        .build(),
)
.path(
    "/apis/serveradmin.innogames.de/v1/namespace/{namespace}/mediadevices/{name}",
    PathItem::builder()
        .parameters(Some(vec![
            Parameter::builder().name("namespace").schema(Some("string")).parameter_in(ParameterIn::Path).build(),
            Parameter::builder().name("name").parameter_in(ParameterIn::Path).schema(Some("string")).build(),
        ]))
        .operation(
            utoipa::openapi::HttpMethod::Get,
            Operation::builder()
                .response(
                    "200",
                    Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.MediaDevice"))).build()).build(),
                )
                .build(),
        )
        .operation(
            utoipa::openapi::HttpMethod::Delete,
            Operation::builder()
                .response(
                    "200",
                    Response::builder().content( "application/json", Content::builder().schema(Some(Ref::new( "#/components/schemas/serveradmin.innogames.de.v1.MediaDevice"))).build()).build(),
                )
                .build(),
        )
        .operation(
            utoipa::openapi::HttpMethod::Patch,
            Operation::builder()
                .request_body(Some(RequestBody::builder().content("application/json", Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.MediaDevice"))).build()).build(),))
                .response(
                    "200",
                    Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.MediaDevice"))).build()).build(),
                )
                .build(),
        )
        .operation(
            utoipa::openapi::HttpMethod::Put,
            Operation::builder()
                .request_body(Some(RequestBody::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.MediaDevice"))).build()).build()))
                .response(
                    "200",
                    Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.MediaDevice"))).build()).build(),
                )
                .build(),
        )
        .build(),
)
    

.path(
    "/apis/serveradmin.innogames.de/v1/namespace//projects",
    PathItem::builder()
        .parameters(Some(vec![
            
        ]))
        .operation(utoipa::openapi::HttpMethod::Get, Operation::builder()
            .response("200", Response::builder()
                .content("application/json", Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.ProjectList"))).build())
                .build(),
            )
        .build())
        .operation(utoipa::openapi::HttpMethod::Post, Operation::builder()
            .request_body(Some(RequestBody::builder().content("application/json", Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.Project"))).build()).build()))
            .response(
                "200",
                Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.ProjectList"))).build()).build(),
            )
            .build(),
        )
        .build(),
)
.path(
    "/apis/serveradmin.innogames.de/v1/namespace//projects/{name}",
    PathItem::builder()
        .parameters(Some(vec![
            
            Parameter::builder().name("name").parameter_in(ParameterIn::Path).schema(Some("string")).build(),
        ]))
        .operation(
            utoipa::openapi::HttpMethod::Get,
            Operation::builder()
                .response(
                    "200",
                    Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.Project"))).build()).build(),
                )
                .build(),
        )
        .operation(
            utoipa::openapi::HttpMethod::Delete,
            Operation::builder()
                .response(
                    "200",
                    Response::builder().content( "application/json", Content::builder().schema(Some(Ref::new( "#/components/schemas/serveradmin.innogames.de.v1.Project"))).build()).build(),
                )
                .build(),
        )
        .operation(
            utoipa::openapi::HttpMethod::Patch,
            Operation::builder()
                .request_body(Some(RequestBody::builder().content("application/json", Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.Project"))).build()).build(),))
                .response(
                    "200",
                    Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.Project"))).build()).build(),
                )
                .build(),
        )
        .operation(
            utoipa::openapi::HttpMethod::Put,
            Operation::builder()
                .request_body(Some(RequestBody::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.Project"))).build()).build()))
                .response(
                    "200",
                    Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.Project"))).build()).build(),
                )
                .build(),
        )
        .build(),
)
    

.path(
    "/apis/serveradmin.innogames.de/v1/namespace/{namespace}/natboxs",
    PathItem::builder()
        .parameters(Some(vec![
            Parameter::builder().name("namespace").schema(Some("string")).parameter_in(ParameterIn::Path).build(),
        ]))
        .operation(utoipa::openapi::HttpMethod::Get, Operation::builder()
            .response("200", Response::builder()
                .content("application/json", Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.NatboxList"))).build())
                .build(),
            )
        .build())
        .operation(utoipa::openapi::HttpMethod::Post, Operation::builder()
            .request_body(Some(RequestBody::builder().content("application/json", Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.Natbox"))).build()).build()))
            .response(
                "200",
                Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.NatboxList"))).build()).build(),
            )
            .build(),
        )
        .build(),
)
.path(
    "/apis/serveradmin.innogames.de/v1/namespace/{namespace}/natboxs/{name}",
    PathItem::builder()
        .parameters(Some(vec![
            Parameter::builder().name("namespace").schema(Some("string")).parameter_in(ParameterIn::Path).build(),
            Parameter::builder().name("name").parameter_in(ParameterIn::Path).schema(Some("string")).build(),
        ]))
        .operation(
            utoipa::openapi::HttpMethod::Get,
            Operation::builder()
                .response(
                    "200",
                    Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.Natbox"))).build()).build(),
                )
                .build(),
        )
        .operation(
            utoipa::openapi::HttpMethod::Delete,
            Operation::builder()
                .response(
                    "200",
                    Response::builder().content( "application/json", Content::builder().schema(Some(Ref::new( "#/components/schemas/serveradmin.innogames.de.v1.Natbox"))).build()).build(),
                )
                .build(),
        )
        .operation(
            utoipa::openapi::HttpMethod::Patch,
            Operation::builder()
                .request_body(Some(RequestBody::builder().content("application/json", Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.Natbox"))).build()).build(),))
                .response(
                    "200",
                    Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.Natbox"))).build()).build(),
                )
                .build(),
        )
        .operation(
            utoipa::openapi::HttpMethod::Put,
            Operation::builder()
                .request_body(Some(RequestBody::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.Natbox"))).build()).build()))
                .response(
                    "200",
                    Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.Natbox"))).build()).build(),
                )
                .build(),
        )
        .build(),
)
    

.path(
    "/apis/serveradmin.innogames.de/v1/namespace/{namespace}/acls",
    PathItem::builder()
        .parameters(Some(vec![
            Parameter::builder().name("namespace").schema(Some("string")).parameter_in(ParameterIn::Path).build(),
        ]))
        .operation(utoipa::openapi::HttpMethod::Get, Operation::builder()
            .response("200", Response::builder()
                .content("application/json", Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.AclList"))).build())
                .build(),
            )
        .build())
        .operation(utoipa::openapi::HttpMethod::Post, Operation::builder()
            .request_body(Some(RequestBody::builder().content("application/json", Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.Acl"))).build()).build()))
            .response(
                "200",
                Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.AclList"))).build()).build(),
            )
            .build(),
        )
        .build(),
)
.path(
    "/apis/serveradmin.innogames.de/v1/namespace/{namespace}/acls/{name}",
    PathItem::builder()
        .parameters(Some(vec![
            Parameter::builder().name("namespace").schema(Some("string")).parameter_in(ParameterIn::Path).build(),
            Parameter::builder().name("name").parameter_in(ParameterIn::Path).schema(Some("string")).build(),
        ]))
        .operation(
            utoipa::openapi::HttpMethod::Get,
            Operation::builder()
                .response(
                    "200",
                    Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.Acl"))).build()).build(),
                )
                .build(),
        )
        .operation(
            utoipa::openapi::HttpMethod::Delete,
            Operation::builder()
                .response(
                    "200",
                    Response::builder().content( "application/json", Content::builder().schema(Some(Ref::new( "#/components/schemas/serveradmin.innogames.de.v1.Acl"))).build()).build(),
                )
                .build(),
        )
        .operation(
            utoipa::openapi::HttpMethod::Patch,
            Operation::builder()
                .request_body(Some(RequestBody::builder().content("application/json", Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.Acl"))).build()).build(),))
                .response(
                    "200",
                    Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.Acl"))).build()).build(),
                )
                .build(),
        )
        .operation(
            utoipa::openapi::HttpMethod::Put,
            Operation::builder()
                .request_body(Some(RequestBody::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.Acl"))).build()).build()))
                .response(
                    "200",
                    Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.Acl"))).build()).build(),
                )
                .build(),
        )
        .build(),
)
    

.path(
    "/apis/serveradmin.innogames.de/v1/namespace/{namespace}/roomtablets",
    PathItem::builder()
        .parameters(Some(vec![
            Parameter::builder().name("namespace").schema(Some("string")).parameter_in(ParameterIn::Path).build(),
        ]))
        .operation(utoipa::openapi::HttpMethod::Get, Operation::builder()
            .response("200", Response::builder()
                .content("application/json", Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.RoomTabletList"))).build())
                .build(),
            )
        .build())
        .operation(utoipa::openapi::HttpMethod::Post, Operation::builder()
            .request_body(Some(RequestBody::builder().content("application/json", Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.RoomTablet"))).build()).build()))
            .response(
                "200",
                Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.RoomTabletList"))).build()).build(),
            )
            .build(),
        )
        .build(),
)
.path(
    "/apis/serveradmin.innogames.de/v1/namespace/{namespace}/roomtablets/{name}",
    PathItem::builder()
        .parameters(Some(vec![
            Parameter::builder().name("namespace").schema(Some("string")).parameter_in(ParameterIn::Path).build(),
            Parameter::builder().name("name").parameter_in(ParameterIn::Path).schema(Some("string")).build(),
        ]))
        .operation(
            utoipa::openapi::HttpMethod::Get,
            Operation::builder()
                .response(
                    "200",
                    Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.RoomTablet"))).build()).build(),
                )
                .build(),
        )
        .operation(
            utoipa::openapi::HttpMethod::Delete,
            Operation::builder()
                .response(
                    "200",
                    Response::builder().content( "application/json", Content::builder().schema(Some(Ref::new( "#/components/schemas/serveradmin.innogames.de.v1.RoomTablet"))).build()).build(),
                )
                .build(),
        )
        .operation(
            utoipa::openapi::HttpMethod::Patch,
            Operation::builder()
                .request_body(Some(RequestBody::builder().content("application/json", Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.RoomTablet"))).build()).build(),))
                .response(
                    "200",
                    Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.RoomTablet"))).build()).build(),
                )
                .build(),
        )
        .operation(
            utoipa::openapi::HttpMethod::Put,
            Operation::builder()
                .request_body(Some(RequestBody::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.RoomTablet"))).build()).build()))
                .response(
                    "200",
                    Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.RoomTablet"))).build()).build(),
                )
                .build(),
        )
        .build(),
)
    

.path(
    "/apis/serveradmin.innogames.de/v1/namespace//azurenetworks",
    PathItem::builder()
        .parameters(Some(vec![
            
        ]))
        .operation(utoipa::openapi::HttpMethod::Get, Operation::builder()
            .response("200", Response::builder()
                .content("application/json", Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.AzureNetworkList"))).build())
                .build(),
            )
        .build())
        .operation(utoipa::openapi::HttpMethod::Post, Operation::builder()
            .request_body(Some(RequestBody::builder().content("application/json", Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.AzureNetwork"))).build()).build()))
            .response(
                "200",
                Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.AzureNetworkList"))).build()).build(),
            )
            .build(),
        )
        .build(),
)
.path(
    "/apis/serveradmin.innogames.de/v1/namespace//azurenetworks/{name}",
    PathItem::builder()
        .parameters(Some(vec![
            
            Parameter::builder().name("name").parameter_in(ParameterIn::Path).schema(Some("string")).build(),
        ]))
        .operation(
            utoipa::openapi::HttpMethod::Get,
            Operation::builder()
                .response(
                    "200",
                    Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.AzureNetwork"))).build()).build(),
                )
                .build(),
        )
        .operation(
            utoipa::openapi::HttpMethod::Delete,
            Operation::builder()
                .response(
                    "200",
                    Response::builder().content( "application/json", Content::builder().schema(Some(Ref::new( "#/components/schemas/serveradmin.innogames.de.v1.AzureNetwork"))).build()).build(),
                )
                .build(),
        )
        .operation(
            utoipa::openapi::HttpMethod::Patch,
            Operation::builder()
                .request_body(Some(RequestBody::builder().content("application/json", Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.AzureNetwork"))).build()).build(),))
                .response(
                    "200",
                    Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.AzureNetwork"))).build()).build(),
                )
                .build(),
        )
        .operation(
            utoipa::openapi::HttpMethod::Put,
            Operation::builder()
                .request_body(Some(RequestBody::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.AzureNetwork"))).build()).build()))
                .response(
                    "200",
                    Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.AzureNetwork"))).build()).build(),
                )
                .build(),
        )
        .build(),
)
    

.path(
    "/apis/serveradmin.innogames.de/v1/namespace/{namespace}/publicdomains",
    PathItem::builder()
        .parameters(Some(vec![
            Parameter::builder().name("namespace").schema(Some("string")).parameter_in(ParameterIn::Path).build(),
        ]))
        .operation(utoipa::openapi::HttpMethod::Get, Operation::builder()
            .response("200", Response::builder()
                .content("application/json", Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.PublicDomainList"))).build())
                .build(),
            )
        .build())
        .operation(utoipa::openapi::HttpMethod::Post, Operation::builder()
            .request_body(Some(RequestBody::builder().content("application/json", Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.PublicDomain"))).build()).build()))
            .response(
                "200",
                Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.PublicDomainList"))).build()).build(),
            )
            .build(),
        )
        .build(),
)
.path(
    "/apis/serveradmin.innogames.de/v1/namespace/{namespace}/publicdomains/{name}",
    PathItem::builder()
        .parameters(Some(vec![
            Parameter::builder().name("namespace").schema(Some("string")).parameter_in(ParameterIn::Path).build(),
            Parameter::builder().name("name").parameter_in(ParameterIn::Path).schema(Some("string")).build(),
        ]))
        .operation(
            utoipa::openapi::HttpMethod::Get,
            Operation::builder()
                .response(
                    "200",
                    Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.PublicDomain"))).build()).build(),
                )
                .build(),
        )
        .operation(
            utoipa::openapi::HttpMethod::Delete,
            Operation::builder()
                .response(
                    "200",
                    Response::builder().content( "application/json", Content::builder().schema(Some(Ref::new( "#/components/schemas/serveradmin.innogames.de.v1.PublicDomain"))).build()).build(),
                )
                .build(),
        )
        .operation(
            utoipa::openapi::HttpMethod::Patch,
            Operation::builder()
                .request_body(Some(RequestBody::builder().content("application/json", Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.PublicDomain"))).build()).build(),))
                .response(
                    "200",
                    Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.PublicDomain"))).build()).build(),
                )
                .build(),
        )
        .operation(
            utoipa::openapi::HttpMethod::Put,
            Operation::builder()
                .request_body(Some(RequestBody::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.PublicDomain"))).build()).build()))
                .response(
                    "200",
                    Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.PublicDomain"))).build()).build(),
                )
                .build(),
        )
        .build(),
)
    

.path(
    "/apis/serveradmin.innogames.de/v1/namespace/{namespace}/hwinternalrouters",
    PathItem::builder()
        .parameters(Some(vec![
            Parameter::builder().name("namespace").schema(Some("string")).parameter_in(ParameterIn::Path).build(),
        ]))
        .operation(utoipa::openapi::HttpMethod::Get, Operation::builder()
            .response("200", Response::builder()
                .content("application/json", Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.HwInternalrouterList"))).build())
                .build(),
            )
        .build())
        .operation(utoipa::openapi::HttpMethod::Post, Operation::builder()
            .request_body(Some(RequestBody::builder().content("application/json", Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.HwInternalrouter"))).build()).build()))
            .response(
                "200",
                Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.HwInternalrouterList"))).build()).build(),
            )
            .build(),
        )
        .build(),
)
.path(
    "/apis/serveradmin.innogames.de/v1/namespace/{namespace}/hwinternalrouters/{name}",
    PathItem::builder()
        .parameters(Some(vec![
            Parameter::builder().name("namespace").schema(Some("string")).parameter_in(ParameterIn::Path).build(),
            Parameter::builder().name("name").parameter_in(ParameterIn::Path).schema(Some("string")).build(),
        ]))
        .operation(
            utoipa::openapi::HttpMethod::Get,
            Operation::builder()
                .response(
                    "200",
                    Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.HwInternalrouter"))).build()).build(),
                )
                .build(),
        )
        .operation(
            utoipa::openapi::HttpMethod::Delete,
            Operation::builder()
                .response(
                    "200",
                    Response::builder().content( "application/json", Content::builder().schema(Some(Ref::new( "#/components/schemas/serveradmin.innogames.de.v1.HwInternalrouter"))).build()).build(),
                )
                .build(),
        )
        .operation(
            utoipa::openapi::HttpMethod::Patch,
            Operation::builder()
                .request_body(Some(RequestBody::builder().content("application/json", Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.HwInternalrouter"))).build()).build(),))
                .response(
                    "200",
                    Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.HwInternalrouter"))).build()).build(),
                )
                .build(),
        )
        .operation(
            utoipa::openapi::HttpMethod::Put,
            Operation::builder()
                .request_body(Some(RequestBody::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.HwInternalrouter"))).build()).build()))
                .response(
                    "200",
                    Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.HwInternalrouter"))).build()).build(),
                )
                .build(),
        )
        .build(),
)
    

.path(
    "/apis/serveradmin.innogames.de/v1/namespace/{namespace}/projectnetworks",
    PathItem::builder()
        .parameters(Some(vec![
            Parameter::builder().name("namespace").schema(Some("string")).parameter_in(ParameterIn::Path).build(),
        ]))
        .operation(utoipa::openapi::HttpMethod::Get, Operation::builder()
            .response("200", Response::builder()
                .content("application/json", Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.ProjectNetworkList"))).build())
                .build(),
            )
        .build())
        .operation(utoipa::openapi::HttpMethod::Post, Operation::builder()
            .request_body(Some(RequestBody::builder().content("application/json", Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.ProjectNetwork"))).build()).build()))
            .response(
                "200",
                Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.ProjectNetworkList"))).build()).build(),
            )
            .build(),
        )
        .build(),
)
.path(
    "/apis/serveradmin.innogames.de/v1/namespace/{namespace}/projectnetworks/{name}",
    PathItem::builder()
        .parameters(Some(vec![
            Parameter::builder().name("namespace").schema(Some("string")).parameter_in(ParameterIn::Path).build(),
            Parameter::builder().name("name").parameter_in(ParameterIn::Path).schema(Some("string")).build(),
        ]))
        .operation(
            utoipa::openapi::HttpMethod::Get,
            Operation::builder()
                .response(
                    "200",
                    Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.ProjectNetwork"))).build()).build(),
                )
                .build(),
        )
        .operation(
            utoipa::openapi::HttpMethod::Delete,
            Operation::builder()
                .response(
                    "200",
                    Response::builder().content( "application/json", Content::builder().schema(Some(Ref::new( "#/components/schemas/serveradmin.innogames.de.v1.ProjectNetwork"))).build()).build(),
                )
                .build(),
        )
        .operation(
            utoipa::openapi::HttpMethod::Patch,
            Operation::builder()
                .request_body(Some(RequestBody::builder().content("application/json", Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.ProjectNetwork"))).build()).build(),))
                .response(
                    "200",
                    Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.ProjectNetwork"))).build()).build(),
                )
                .build(),
        )
        .operation(
            utoipa::openapi::HttpMethod::Put,
            Operation::builder()
                .request_body(Some(RequestBody::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.ProjectNetwork"))).build()).build()))
                .response(
                    "200",
                    Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.ProjectNetwork"))).build()).build(),
                )
                .build(),
        )
        .build(),
)
    

.path(
    "/apis/serveradmin.innogames.de/v1/namespace/{namespace}/vms",
    PathItem::builder()
        .parameters(Some(vec![
            Parameter::builder().name("namespace").schema(Some("string")).parameter_in(ParameterIn::Path).build(),
        ]))
        .operation(utoipa::openapi::HttpMethod::Get, Operation::builder()
            .response("200", Response::builder()
                .content("application/json", Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.VmList"))).build())
                .build(),
            )
        .build())
        .operation(utoipa::openapi::HttpMethod::Post, Operation::builder()
            .request_body(Some(RequestBody::builder().content("application/json", Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.Vm"))).build()).build()))
            .response(
                "200",
                Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.VmList"))).build()).build(),
            )
            .build(),
        )
        .build(),
)
.path(
    "/apis/serveradmin.innogames.de/v1/namespace/{namespace}/vms/{name}",
    PathItem::builder()
        .parameters(Some(vec![
            Parameter::builder().name("namespace").schema(Some("string")).parameter_in(ParameterIn::Path).build(),
            Parameter::builder().name("name").parameter_in(ParameterIn::Path).schema(Some("string")).build(),
        ]))
        .operation(
            utoipa::openapi::HttpMethod::Get,
            Operation::builder()
                .response(
                    "200",
                    Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.Vm"))).build()).build(),
                )
                .build(),
        )
        .operation(
            utoipa::openapi::HttpMethod::Delete,
            Operation::builder()
                .response(
                    "200",
                    Response::builder().content( "application/json", Content::builder().schema(Some(Ref::new( "#/components/schemas/serveradmin.innogames.de.v1.Vm"))).build()).build(),
                )
                .build(),
        )
        .operation(
            utoipa::openapi::HttpMethod::Patch,
            Operation::builder()
                .request_body(Some(RequestBody::builder().content("application/json", Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.Vm"))).build()).build(),))
                .response(
                    "200",
                    Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.Vm"))).build()).build(),
                )
                .build(),
        )
        .operation(
            utoipa::openapi::HttpMethod::Put,
            Operation::builder()
                .request_body(Some(RequestBody::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.Vm"))).build()).build()))
                .response(
                    "200",
                    Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.Vm"))).build()).build(),
                )
                .build(),
        )
        .build(),
)
    

.path(
    "/apis/serveradmin.innogames.de/v1/namespace/{namespace}/accesspoints",
    PathItem::builder()
        .parameters(Some(vec![
            Parameter::builder().name("namespace").schema(Some("string")).parameter_in(ParameterIn::Path).build(),
        ]))
        .operation(utoipa::openapi::HttpMethod::Get, Operation::builder()
            .response("200", Response::builder()
                .content("application/json", Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.AccesspointList"))).build())
                .build(),
            )
        .build())
        .operation(utoipa::openapi::HttpMethod::Post, Operation::builder()
            .request_body(Some(RequestBody::builder().content("application/json", Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.Accesspoint"))).build()).build()))
            .response(
                "200",
                Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.AccesspointList"))).build()).build(),
            )
            .build(),
        )
        .build(),
)
.path(
    "/apis/serveradmin.innogames.de/v1/namespace/{namespace}/accesspoints/{name}",
    PathItem::builder()
        .parameters(Some(vec![
            Parameter::builder().name("namespace").schema(Some("string")).parameter_in(ParameterIn::Path).build(),
            Parameter::builder().name("name").parameter_in(ParameterIn::Path).schema(Some("string")).build(),
        ]))
        .operation(
            utoipa::openapi::HttpMethod::Get,
            Operation::builder()
                .response(
                    "200",
                    Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.Accesspoint"))).build()).build(),
                )
                .build(),
        )
        .operation(
            utoipa::openapi::HttpMethod::Delete,
            Operation::builder()
                .response(
                    "200",
                    Response::builder().content( "application/json", Content::builder().schema(Some(Ref::new( "#/components/schemas/serveradmin.innogames.de.v1.Accesspoint"))).build()).build(),
                )
                .build(),
        )
        .operation(
            utoipa::openapi::HttpMethod::Patch,
            Operation::builder()
                .request_body(Some(RequestBody::builder().content("application/json", Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.Accesspoint"))).build()).build(),))
                .response(
                    "200",
                    Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.Accesspoint"))).build()).build(),
                )
                .build(),
        )
        .operation(
            utoipa::openapi::HttpMethod::Put,
            Operation::builder()
                .request_body(Some(RequestBody::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.Accesspoint"))).build()).build()))
                .response(
                    "200",
                    Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.Accesspoint"))).build()).build(),
                )
                .build(),
        )
        .build(),
)
    

.path(
    "/apis/serveradmin.innogames.de/v1/namespace/{namespace}/printers",
    PathItem::builder()
        .parameters(Some(vec![
            Parameter::builder().name("namespace").schema(Some("string")).parameter_in(ParameterIn::Path).build(),
        ]))
        .operation(utoipa::openapi::HttpMethod::Get, Operation::builder()
            .response("200", Response::builder()
                .content("application/json", Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.PrinterList"))).build())
                .build(),
            )
        .build())
        .operation(utoipa::openapi::HttpMethod::Post, Operation::builder()
            .request_body(Some(RequestBody::builder().content("application/json", Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.Printer"))).build()).build()))
            .response(
                "200",
                Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.PrinterList"))).build()).build(),
            )
            .build(),
        )
        .build(),
)
.path(
    "/apis/serveradmin.innogames.de/v1/namespace/{namespace}/printers/{name}",
    PathItem::builder()
        .parameters(Some(vec![
            Parameter::builder().name("namespace").schema(Some("string")).parameter_in(ParameterIn::Path).build(),
            Parameter::builder().name("name").parameter_in(ParameterIn::Path).schema(Some("string")).build(),
        ]))
        .operation(
            utoipa::openapi::HttpMethod::Get,
            Operation::builder()
                .response(
                    "200",
                    Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.Printer"))).build()).build(),
                )
                .build(),
        )
        .operation(
            utoipa::openapi::HttpMethod::Delete,
            Operation::builder()
                .response(
                    "200",
                    Response::builder().content( "application/json", Content::builder().schema(Some(Ref::new( "#/components/schemas/serveradmin.innogames.de.v1.Printer"))).build()).build(),
                )
                .build(),
        )
        .operation(
            utoipa::openapi::HttpMethod::Patch,
            Operation::builder()
                .request_body(Some(RequestBody::builder().content("application/json", Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.Printer"))).build()).build(),))
                .response(
                    "200",
                    Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.Printer"))).build()).build(),
                )
                .build(),
        )
        .operation(
            utoipa::openapi::HttpMethod::Put,
            Operation::builder()
                .request_body(Some(RequestBody::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.Printer"))).build()).build()))
                .response(
                    "200",
                    Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.Printer"))).build()).build(),
                )
                .build(),
        )
        .build(),
)
    

.path(
    "/apis/serveradmin.innogames.de/v1/namespace/{namespace}/cloudnetworks",
    PathItem::builder()
        .parameters(Some(vec![
            Parameter::builder().name("namespace").schema(Some("string")).parameter_in(ParameterIn::Path).build(),
        ]))
        .operation(utoipa::openapi::HttpMethod::Get, Operation::builder()
            .response("200", Response::builder()
                .content("application/json", Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.CloudNetworkList"))).build())
                .build(),
            )
        .build())
        .operation(utoipa::openapi::HttpMethod::Post, Operation::builder()
            .request_body(Some(RequestBody::builder().content("application/json", Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.CloudNetwork"))).build()).build()))
            .response(
                "200",
                Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.CloudNetworkList"))).build()).build(),
            )
            .build(),
        )
        .build(),
)
.path(
    "/apis/serveradmin.innogames.de/v1/namespace/{namespace}/cloudnetworks/{name}",
    PathItem::builder()
        .parameters(Some(vec![
            Parameter::builder().name("namespace").schema(Some("string")).parameter_in(ParameterIn::Path).build(),
            Parameter::builder().name("name").parameter_in(ParameterIn::Path).schema(Some("string")).build(),
        ]))
        .operation(
            utoipa::openapi::HttpMethod::Get,
            Operation::builder()
                .response(
                    "200",
                    Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.CloudNetwork"))).build()).build(),
                )
                .build(),
        )
        .operation(
            utoipa::openapi::HttpMethod::Delete,
            Operation::builder()
                .response(
                    "200",
                    Response::builder().content( "application/json", Content::builder().schema(Some(Ref::new( "#/components/schemas/serveradmin.innogames.de.v1.CloudNetwork"))).build()).build(),
                )
                .build(),
        )
        .operation(
            utoipa::openapi::HttpMethod::Patch,
            Operation::builder()
                .request_body(Some(RequestBody::builder().content("application/json", Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.CloudNetwork"))).build()).build(),))
                .response(
                    "200",
                    Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.CloudNetwork"))).build()).build(),
                )
                .build(),
        )
        .operation(
            utoipa::openapi::HttpMethod::Put,
            Operation::builder()
                .request_body(Some(RequestBody::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.CloudNetwork"))).build()).build()))
                .response(
                    "200",
                    Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.CloudNetwork"))).build()).build(),
                )
                .build(),
        )
        .build(),
)
    

.path(
    "/apis/serveradmin.innogames.de/v1/namespace/{namespace}/ipmis",
    PathItem::builder()
        .parameters(Some(vec![
            Parameter::builder().name("namespace").schema(Some("string")).parameter_in(ParameterIn::Path).build(),
        ]))
        .operation(utoipa::openapi::HttpMethod::Get, Operation::builder()
            .response("200", Response::builder()
                .content("application/json", Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.IpmiList"))).build())
                .build(),
            )
        .build())
        .operation(utoipa::openapi::HttpMethod::Post, Operation::builder()
            .request_body(Some(RequestBody::builder().content("application/json", Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.Ipmi"))).build()).build()))
            .response(
                "200",
                Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.IpmiList"))).build()).build(),
            )
            .build(),
        )
        .build(),
)
.path(
    "/apis/serveradmin.innogames.de/v1/namespace/{namespace}/ipmis/{name}",
    PathItem::builder()
        .parameters(Some(vec![
            Parameter::builder().name("namespace").schema(Some("string")).parameter_in(ParameterIn::Path).build(),
            Parameter::builder().name("name").parameter_in(ParameterIn::Path).schema(Some("string")).build(),
        ]))
        .operation(
            utoipa::openapi::HttpMethod::Get,
            Operation::builder()
                .response(
                    "200",
                    Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.Ipmi"))).build()).build(),
                )
                .build(),
        )
        .operation(
            utoipa::openapi::HttpMethod::Delete,
            Operation::builder()
                .response(
                    "200",
                    Response::builder().content( "application/json", Content::builder().schema(Some(Ref::new( "#/components/schemas/serveradmin.innogames.de.v1.Ipmi"))).build()).build(),
                )
                .build(),
        )
        .operation(
            utoipa::openapi::HttpMethod::Patch,
            Operation::builder()
                .request_body(Some(RequestBody::builder().content("application/json", Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.Ipmi"))).build()).build(),))
                .response(
                    "200",
                    Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.Ipmi"))).build()).build(),
                )
                .build(),
        )
        .operation(
            utoipa::openapi::HttpMethod::Put,
            Operation::builder()
                .request_body(Some(RequestBody::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.Ipmi"))).build()).build()))
                .response(
                    "200",
                    Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.Ipmi"))).build()).build(),
                )
                .build(),
        )
        .build(),
)
    

.path(
    "/apis/serveradmin.innogames.de/v1/namespace/{namespace}/voipphones",
    PathItem::builder()
        .parameters(Some(vec![
            Parameter::builder().name("namespace").schema(Some("string")).parameter_in(ParameterIn::Path).build(),
        ]))
        .operation(utoipa::openapi::HttpMethod::Get, Operation::builder()
            .response("200", Response::builder()
                .content("application/json", Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.VoipPhoneList"))).build())
                .build(),
            )
        .build())
        .operation(utoipa::openapi::HttpMethod::Post, Operation::builder()
            .request_body(Some(RequestBody::builder().content("application/json", Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.VoipPhone"))).build()).build()))
            .response(
                "200",
                Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.VoipPhoneList"))).build()).build(),
            )
            .build(),
        )
        .build(),
)
.path(
    "/apis/serveradmin.innogames.de/v1/namespace/{namespace}/voipphones/{name}",
    PathItem::builder()
        .parameters(Some(vec![
            Parameter::builder().name("namespace").schema(Some("string")).parameter_in(ParameterIn::Path).build(),
            Parameter::builder().name("name").parameter_in(ParameterIn::Path).schema(Some("string")).build(),
        ]))
        .operation(
            utoipa::openapi::HttpMethod::Get,
            Operation::builder()
                .response(
                    "200",
                    Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.VoipPhone"))).build()).build(),
                )
                .build(),
        )
        .operation(
            utoipa::openapi::HttpMethod::Delete,
            Operation::builder()
                .response(
                    "200",
                    Response::builder().content( "application/json", Content::builder().schema(Some(Ref::new( "#/components/schemas/serveradmin.innogames.de.v1.VoipPhone"))).build()).build(),
                )
                .build(),
        )
        .operation(
            utoipa::openapi::HttpMethod::Patch,
            Operation::builder()
                .request_body(Some(RequestBody::builder().content("application/json", Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.VoipPhone"))).build()).build(),))
                .response(
                    "200",
                    Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.VoipPhone"))).build()).build(),
                )
                .build(),
        )
        .operation(
            utoipa::openapi::HttpMethod::Put,
            Operation::builder()
                .request_body(Some(RequestBody::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.VoipPhone"))).build()).build()))
                .response(
                    "200",
                    Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.VoipPhone"))).build()).build(),
                )
                .build(),
        )
        .build(),
)
    

.path(
    "/apis/serveradmin.innogames.de/v1/namespace/{namespace}/projectdomains",
    PathItem::builder()
        .parameters(Some(vec![
            Parameter::builder().name("namespace").schema(Some("string")).parameter_in(ParameterIn::Path).build(),
        ]))
        .operation(utoipa::openapi::HttpMethod::Get, Operation::builder()
            .response("200", Response::builder()
                .content("application/json", Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.ProjectDomainList"))).build())
                .build(),
            )
        .build())
        .operation(utoipa::openapi::HttpMethod::Post, Operation::builder()
            .request_body(Some(RequestBody::builder().content("application/json", Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.ProjectDomain"))).build()).build()))
            .response(
                "200",
                Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.ProjectDomainList"))).build()).build(),
            )
            .build(),
        )
        .build(),
)
.path(
    "/apis/serveradmin.innogames.de/v1/namespace/{namespace}/projectdomains/{name}",
    PathItem::builder()
        .parameters(Some(vec![
            Parameter::builder().name("namespace").schema(Some("string")).parameter_in(ParameterIn::Path).build(),
            Parameter::builder().name("name").parameter_in(ParameterIn::Path).schema(Some("string")).build(),
        ]))
        .operation(
            utoipa::openapi::HttpMethod::Get,
            Operation::builder()
                .response(
                    "200",
                    Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.ProjectDomain"))).build()).build(),
                )
                .build(),
        )
        .operation(
            utoipa::openapi::HttpMethod::Delete,
            Operation::builder()
                .response(
                    "200",
                    Response::builder().content( "application/json", Content::builder().schema(Some(Ref::new( "#/components/schemas/serveradmin.innogames.de.v1.ProjectDomain"))).build()).build(),
                )
                .build(),
        )
        .operation(
            utoipa::openapi::HttpMethod::Patch,
            Operation::builder()
                .request_body(Some(RequestBody::builder().content("application/json", Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.ProjectDomain"))).build()).build(),))
                .response(
                    "200",
                    Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.ProjectDomain"))).build()).build(),
                )
                .build(),
        )
        .operation(
            utoipa::openapi::HttpMethod::Put,
            Operation::builder()
                .request_body(Some(RequestBody::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.ProjectDomain"))).build()).build()))
                .response(
                    "200",
                    Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.ProjectDomain"))).build()).build(),
                )
                .build(),
        )
        .build(),
)
    

.path(
    "/apis/serveradmin.innogames.de/v1/namespace/{namespace}/racks",
    PathItem::builder()
        .parameters(Some(vec![
            Parameter::builder().name("namespace").schema(Some("string")).parameter_in(ParameterIn::Path).build(),
        ]))
        .operation(utoipa::openapi::HttpMethod::Get, Operation::builder()
            .response("200", Response::builder()
                .content("application/json", Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.RackList"))).build())
                .build(),
            )
        .build())
        .operation(utoipa::openapi::HttpMethod::Post, Operation::builder()
            .request_body(Some(RequestBody::builder().content("application/json", Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.Rack"))).build()).build()))
            .response(
                "200",
                Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.RackList"))).build()).build(),
            )
            .build(),
        )
        .build(),
)
.path(
    "/apis/serveradmin.innogames.de/v1/namespace/{namespace}/racks/{name}",
    PathItem::builder()
        .parameters(Some(vec![
            Parameter::builder().name("namespace").schema(Some("string")).parameter_in(ParameterIn::Path).build(),
            Parameter::builder().name("name").parameter_in(ParameterIn::Path).schema(Some("string")).build(),
        ]))
        .operation(
            utoipa::openapi::HttpMethod::Get,
            Operation::builder()
                .response(
                    "200",
                    Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.Rack"))).build()).build(),
                )
                .build(),
        )
        .operation(
            utoipa::openapi::HttpMethod::Delete,
            Operation::builder()
                .response(
                    "200",
                    Response::builder().content( "application/json", Content::builder().schema(Some(Ref::new( "#/components/schemas/serveradmin.innogames.de.v1.Rack"))).build()).build(),
                )
                .build(),
        )
        .operation(
            utoipa::openapi::HttpMethod::Patch,
            Operation::builder()
                .request_body(Some(RequestBody::builder().content("application/json", Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.Rack"))).build()).build(),))
                .response(
                    "200",
                    Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.Rack"))).build()).build(),
                )
                .build(),
        )
        .operation(
            utoipa::openapi::HttpMethod::Put,
            Operation::builder()
                .request_body(Some(RequestBody::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.Rack"))).build()).build()))
                .response(
                    "200",
                    Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.Rack"))).build()).build(),
                )
                .build(),
        )
        .build(),
)
    

.path(
    "/apis/serveradmin.innogames.de/v1/namespace/{namespace}/hardwareexternals",
    PathItem::builder()
        .parameters(Some(vec![
            Parameter::builder().name("namespace").schema(Some("string")).parameter_in(ParameterIn::Path).build(),
        ]))
        .operation(utoipa::openapi::HttpMethod::Get, Operation::builder()
            .response("200", Response::builder()
                .content("application/json", Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.HardwareExternalList"))).build())
                .build(),
            )
        .build())
        .operation(utoipa::openapi::HttpMethod::Post, Operation::builder()
            .request_body(Some(RequestBody::builder().content("application/json", Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.HardwareExternal"))).build()).build()))
            .response(
                "200",
                Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.HardwareExternalList"))).build()).build(),
            )
            .build(),
        )
        .build(),
)
.path(
    "/apis/serveradmin.innogames.de/v1/namespace/{namespace}/hardwareexternals/{name}",
    PathItem::builder()
        .parameters(Some(vec![
            Parameter::builder().name("namespace").schema(Some("string")).parameter_in(ParameterIn::Path).build(),
            Parameter::builder().name("name").parameter_in(ParameterIn::Path).schema(Some("string")).build(),
        ]))
        .operation(
            utoipa::openapi::HttpMethod::Get,
            Operation::builder()
                .response(
                    "200",
                    Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.HardwareExternal"))).build()).build(),
                )
                .build(),
        )
        .operation(
            utoipa::openapi::HttpMethod::Delete,
            Operation::builder()
                .response(
                    "200",
                    Response::builder().content( "application/json", Content::builder().schema(Some(Ref::new( "#/components/schemas/serveradmin.innogames.de.v1.HardwareExternal"))).build()).build(),
                )
                .build(),
        )
        .operation(
            utoipa::openapi::HttpMethod::Patch,
            Operation::builder()
                .request_body(Some(RequestBody::builder().content("application/json", Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.HardwareExternal"))).build()).build(),))
                .response(
                    "200",
                    Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.HardwareExternal"))).build()).build(),
                )
                .build(),
        )
        .operation(
            utoipa::openapi::HttpMethod::Put,
            Operation::builder()
                .request_body(Some(RequestBody::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.HardwareExternal"))).build()).build()))
                .response(
                    "200",
                    Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.HardwareExternal"))).build()).build(),
                )
                .build(),
        )
        .build(),
)
    

.path(
    "/apis/serveradmin.innogames.de/v1/namespace/{namespace}/vpns",
    PathItem::builder()
        .parameters(Some(vec![
            Parameter::builder().name("namespace").schema(Some("string")).parameter_in(ParameterIn::Path).build(),
        ]))
        .operation(utoipa::openapi::HttpMethod::Get, Operation::builder()
            .response("200", Response::builder()
                .content("application/json", Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.VpnList"))).build())
                .build(),
            )
        .build())
        .operation(utoipa::openapi::HttpMethod::Post, Operation::builder()
            .request_body(Some(RequestBody::builder().content("application/json", Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.Vpn"))).build()).build()))
            .response(
                "200",
                Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.VpnList"))).build()).build(),
            )
            .build(),
        )
        .build(),
)
.path(
    "/apis/serveradmin.innogames.de/v1/namespace/{namespace}/vpns/{name}",
    PathItem::builder()
        .parameters(Some(vec![
            Parameter::builder().name("namespace").schema(Some("string")).parameter_in(ParameterIn::Path).build(),
            Parameter::builder().name("name").parameter_in(ParameterIn::Path).schema(Some("string")).build(),
        ]))
        .operation(
            utoipa::openapi::HttpMethod::Get,
            Operation::builder()
                .response(
                    "200",
                    Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.Vpn"))).build()).build(),
                )
                .build(),
        )
        .operation(
            utoipa::openapi::HttpMethod::Delete,
            Operation::builder()
                .response(
                    "200",
                    Response::builder().content( "application/json", Content::builder().schema(Some(Ref::new( "#/components/schemas/serveradmin.innogames.de.v1.Vpn"))).build()).build(),
                )
                .build(),
        )
        .operation(
            utoipa::openapi::HttpMethod::Patch,
            Operation::builder()
                .request_body(Some(RequestBody::builder().content("application/json", Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.Vpn"))).build()).build(),))
                .response(
                    "200",
                    Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.Vpn"))).build()).build(),
                )
                .build(),
        )
        .operation(
            utoipa::openapi::HttpMethod::Put,
            Operation::builder()
                .request_body(Some(RequestBody::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.Vpn"))).build()).build()))
                .response(
                    "200",
                    Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.Vpn"))).build()).build(),
                )
                .build(),
        )
        .build(),
)
    

.path(
    "/apis/serveradmin.innogames.de/v1/namespace//providerdomains",
    PathItem::builder()
        .parameters(Some(vec![
            
        ]))
        .operation(utoipa::openapi::HttpMethod::Get, Operation::builder()
            .response("200", Response::builder()
                .content("application/json", Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.ProviderDomainList"))).build())
                .build(),
            )
        .build())
        .operation(utoipa::openapi::HttpMethod::Post, Operation::builder()
            .request_body(Some(RequestBody::builder().content("application/json", Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.ProviderDomain"))).build()).build()))
            .response(
                "200",
                Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.ProviderDomainList"))).build()).build(),
            )
            .build(),
        )
        .build(),
)
.path(
    "/apis/serveradmin.innogames.de/v1/namespace//providerdomains/{name}",
    PathItem::builder()
        .parameters(Some(vec![
            
            Parameter::builder().name("name").parameter_in(ParameterIn::Path).schema(Some("string")).build(),
        ]))
        .operation(
            utoipa::openapi::HttpMethod::Get,
            Operation::builder()
                .response(
                    "200",
                    Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.ProviderDomain"))).build()).build(),
                )
                .build(),
        )
        .operation(
            utoipa::openapi::HttpMethod::Delete,
            Operation::builder()
                .response(
                    "200",
                    Response::builder().content( "application/json", Content::builder().schema(Some(Ref::new( "#/components/schemas/serveradmin.innogames.de.v1.ProviderDomain"))).build()).build(),
                )
                .build(),
        )
        .operation(
            utoipa::openapi::HttpMethod::Patch,
            Operation::builder()
                .request_body(Some(RequestBody::builder().content("application/json", Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.ProviderDomain"))).build()).build(),))
                .response(
                    "200",
                    Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.ProviderDomain"))).build()).build(),
                )
                .build(),
        )
        .operation(
            utoipa::openapi::HttpMethod::Put,
            Operation::builder()
                .request_body(Some(RequestBody::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.ProviderDomain"))).build()).build()))
                .response(
                    "200",
                    Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.ProviderDomain"))).build()).build(),
                )
                .build(),
        )
        .build(),
)
    

.path(
    "/apis/serveradmin.innogames.de/v1/namespace/{namespace}/lbpools",
    PathItem::builder()
        .parameters(Some(vec![
            Parameter::builder().name("namespace").schema(Some("string")).parameter_in(ParameterIn::Path).build(),
        ]))
        .operation(utoipa::openapi::HttpMethod::Get, Operation::builder()
            .response("200", Response::builder()
                .content("application/json", Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.LbPoolList"))).build())
                .build(),
            )
        .build())
        .operation(utoipa::openapi::HttpMethod::Post, Operation::builder()
            .request_body(Some(RequestBody::builder().content("application/json", Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.LbPool"))).build()).build()))
            .response(
                "200",
                Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.LbPoolList"))).build()).build(),
            )
            .build(),
        )
        .build(),
)
.path(
    "/apis/serveradmin.innogames.de/v1/namespace/{namespace}/lbpools/{name}",
    PathItem::builder()
        .parameters(Some(vec![
            Parameter::builder().name("namespace").schema(Some("string")).parameter_in(ParameterIn::Path).build(),
            Parameter::builder().name("name").parameter_in(ParameterIn::Path).schema(Some("string")).build(),
        ]))
        .operation(
            utoipa::openapi::HttpMethod::Get,
            Operation::builder()
                .response(
                    "200",
                    Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.LbPool"))).build()).build(),
                )
                .build(),
        )
        .operation(
            utoipa::openapi::HttpMethod::Delete,
            Operation::builder()
                .response(
                    "200",
                    Response::builder().content( "application/json", Content::builder().schema(Some(Ref::new( "#/components/schemas/serveradmin.innogames.de.v1.LbPool"))).build()).build(),
                )
                .build(),
        )
        .operation(
            utoipa::openapi::HttpMethod::Patch,
            Operation::builder()
                .request_body(Some(RequestBody::builder().content("application/json", Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.LbPool"))).build()).build(),))
                .response(
                    "200",
                    Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.LbPool"))).build()).build(),
                )
                .build(),
        )
        .operation(
            utoipa::openapi::HttpMethod::Put,
            Operation::builder()
                .request_body(Some(RequestBody::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.LbPool"))).build()).build()))
                .response(
                    "200",
                    Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.LbPool"))).build()).build(),
                )
                .build(),
        )
        .build(),
)
    

.path(
    "/apis/serveradmin.innogames.de/v1/namespace/{namespace}/bondinterfaces",
    PathItem::builder()
        .parameters(Some(vec![
            Parameter::builder().name("namespace").schema(Some("string")).parameter_in(ParameterIn::Path).build(),
        ]))
        .operation(utoipa::openapi::HttpMethod::Get, Operation::builder()
            .response("200", Response::builder()
                .content("application/json", Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.BondInterfaceList"))).build())
                .build(),
            )
        .build())
        .operation(utoipa::openapi::HttpMethod::Post, Operation::builder()
            .request_body(Some(RequestBody::builder().content("application/json", Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.BondInterface"))).build()).build()))
            .response(
                "200",
                Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.BondInterfaceList"))).build()).build(),
            )
            .build(),
        )
        .build(),
)
.path(
    "/apis/serveradmin.innogames.de/v1/namespace/{namespace}/bondinterfaces/{name}",
    PathItem::builder()
        .parameters(Some(vec![
            Parameter::builder().name("namespace").schema(Some("string")).parameter_in(ParameterIn::Path).build(),
            Parameter::builder().name("name").parameter_in(ParameterIn::Path).schema(Some("string")).build(),
        ]))
        .operation(
            utoipa::openapi::HttpMethod::Get,
            Operation::builder()
                .response(
                    "200",
                    Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.BondInterface"))).build()).build(),
                )
                .build(),
        )
        .operation(
            utoipa::openapi::HttpMethod::Delete,
            Operation::builder()
                .response(
                    "200",
                    Response::builder().content( "application/json", Content::builder().schema(Some(Ref::new( "#/components/schemas/serveradmin.innogames.de.v1.BondInterface"))).build()).build(),
                )
                .build(),
        )
        .operation(
            utoipa::openapi::HttpMethod::Patch,
            Operation::builder()
                .request_body(Some(RequestBody::builder().content("application/json", Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.BondInterface"))).build()).build(),))
                .response(
                    "200",
                    Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.BondInterface"))).build()).build(),
                )
                .build(),
        )
        .operation(
            utoipa::openapi::HttpMethod::Put,
            Operation::builder()
                .request_body(Some(RequestBody::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.BondInterface"))).build()).build()))
                .response(
                    "200",
                    Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.BondInterface"))).build()).build(),
                )
                .build(),
        )
        .build(),
)
    

.path(
    "/apis/serveradmin.innogames.de/v1/namespace/{namespace}/floatingaddresss",
    PathItem::builder()
        .parameters(Some(vec![
            Parameter::builder().name("namespace").schema(Some("string")).parameter_in(ParameterIn::Path).build(),
        ]))
        .operation(utoipa::openapi::HttpMethod::Get, Operation::builder()
            .response("200", Response::builder()
                .content("application/json", Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.FloatingAddressList"))).build())
                .build(),
            )
        .build())
        .operation(utoipa::openapi::HttpMethod::Post, Operation::builder()
            .request_body(Some(RequestBody::builder().content("application/json", Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.FloatingAddress"))).build()).build()))
            .response(
                "200",
                Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.FloatingAddressList"))).build()).build(),
            )
            .build(),
        )
        .build(),
)
.path(
    "/apis/serveradmin.innogames.de/v1/namespace/{namespace}/floatingaddresss/{name}",
    PathItem::builder()
        .parameters(Some(vec![
            Parameter::builder().name("namespace").schema(Some("string")).parameter_in(ParameterIn::Path).build(),
            Parameter::builder().name("name").parameter_in(ParameterIn::Path).schema(Some("string")).build(),
        ]))
        .operation(
            utoipa::openapi::HttpMethod::Get,
            Operation::builder()
                .response(
                    "200",
                    Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.FloatingAddress"))).build()).build(),
                )
                .build(),
        )
        .operation(
            utoipa::openapi::HttpMethod::Delete,
            Operation::builder()
                .response(
                    "200",
                    Response::builder().content( "application/json", Content::builder().schema(Some(Ref::new( "#/components/schemas/serveradmin.innogames.de.v1.FloatingAddress"))).build()).build(),
                )
                .build(),
        )
        .operation(
            utoipa::openapi::HttpMethod::Patch,
            Operation::builder()
                .request_body(Some(RequestBody::builder().content("application/json", Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.FloatingAddress"))).build()).build(),))
                .response(
                    "200",
                    Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.FloatingAddress"))).build()).build(),
                )
                .build(),
        )
        .operation(
            utoipa::openapi::HttpMethod::Put,
            Operation::builder()
                .request_body(Some(RequestBody::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.FloatingAddress"))).build()).build()))
                .response(
                    "200",
                    Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.FloatingAddress"))).build()).build(),
                )
                .build(),
        )
        .build(),
)
    

.path(
    "/apis/serveradmin.innogames.de/v1/namespace/{namespace}/vmexternals",
    PathItem::builder()
        .parameters(Some(vec![
            Parameter::builder().name("namespace").schema(Some("string")).parameter_in(ParameterIn::Path).build(),
        ]))
        .operation(utoipa::openapi::HttpMethod::Get, Operation::builder()
            .response("200", Response::builder()
                .content("application/json", Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.VmExternalList"))).build())
                .build(),
            )
        .build())
        .operation(utoipa::openapi::HttpMethod::Post, Operation::builder()
            .request_body(Some(RequestBody::builder().content("application/json", Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.VmExternal"))).build()).build()))
            .response(
                "200",
                Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.VmExternalList"))).build()).build(),
            )
            .build(),
        )
        .build(),
)
.path(
    "/apis/serveradmin.innogames.de/v1/namespace/{namespace}/vmexternals/{name}",
    PathItem::builder()
        .parameters(Some(vec![
            Parameter::builder().name("namespace").schema(Some("string")).parameter_in(ParameterIn::Path).build(),
            Parameter::builder().name("name").parameter_in(ParameterIn::Path).schema(Some("string")).build(),
        ]))
        .operation(
            utoipa::openapi::HttpMethod::Get,
            Operation::builder()
                .response(
                    "200",
                    Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.VmExternal"))).build()).build(),
                )
                .build(),
        )
        .operation(
            utoipa::openapi::HttpMethod::Delete,
            Operation::builder()
                .response(
                    "200",
                    Response::builder().content( "application/json", Content::builder().schema(Some(Ref::new( "#/components/schemas/serveradmin.innogames.de.v1.VmExternal"))).build()).build(),
                )
                .build(),
        )
        .operation(
            utoipa::openapi::HttpMethod::Patch,
            Operation::builder()
                .request_body(Some(RequestBody::builder().content("application/json", Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.VmExternal"))).build()).build(),))
                .response(
                    "200",
                    Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.VmExternal"))).build()).build(),
                )
                .build(),
        )
        .operation(
            utoipa::openapi::HttpMethod::Put,
            Operation::builder()
                .request_body(Some(RequestBody::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.VmExternal"))).build()).build()))
                .response(
                    "200",
                    Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("#/components/schemas/serveradmin.innogames.de.v1.VmExternal"))).build()).build(),
                )
                .build(),
        )
        .build(),
)
    
    )
    .components(Some(
        Components::builder()
            .schema_from::<MemoryModule>()
.schema_from::<MemoryModuleList>()
.schema_from::<MemoryModuleSpec>()
.schema_from::<HdmiTransmitter>()
.schema_from::<HdmiTransmitterList>()
.schema_from::<HdmiTransmitterSpec>()
.schema_from::<ServiceGroup>()
.schema_from::<ServiceGroupList>()
.schema_from::<ServiceGroupSpec>()
.schema_from::<MonitoringZone>()
.schema_from::<MonitoringZoneList>()
.schema_from::<MonitoringZoneSpec>()
.schema_from::<HwLoadbalancer>()
.schema_from::<HwLoadbalancerList>()
.schema_from::<HwLoadbalancerSpec>()
.schema_from::<Hardware>()
.schema_from::<HardwareList>()
.schema_from::<HardwareSpec>()
.schema_from::<Hypervisor>()
.schema_from::<HypervisorList>()
.schema_from::<HypervisorSpec>()
.schema_from::<ExternalNetwork>()
.schema_from::<ExternalNetworkList>()
.schema_from::<ExternalNetworkSpec>()
.schema_from::<Loadbalancer>()
.schema_from::<LoadbalancerList>()
.schema_from::<LoadbalancerSpec>()
.schema_from::<Switch>()
.schema_from::<SwitchList>()
.schema_from::<SwitchSpec>()
.schema_from::<DatacenterType>()
.schema_from::<DatacenterTypeList>()
.schema_from::<DatacenterTypeSpec>()
.schema_from::<HwLoadbalancerGroup>()
.schema_from::<HwLoadbalancerGroupList>()
.schema_from::<HwLoadbalancerGroupSpec>()
.schema_from::<Router>()
.schema_from::<RouterList>()
.schema_from::<RouterSpec>()
.schema_from::<Everything>()
.schema_from::<EverythingList>()
.schema_from::<EverythingSpec>()
.schema_from::<Doorlock>()
.schema_from::<DoorlockList>()
.schema_from::<DoorlockSpec>()
.schema_from::<ProviderNetwork>()
.schema_from::<ProviderNetworkList>()
.schema_from::<ProviderNetworkSpec>()
.schema_from::<MonitoringGroup>()
.schema_from::<MonitoringGroupList>()
.schema_from::<MonitoringGroupSpec>()
.schema_from::<VlanInterface>()
.schema_from::<VlanInterfaceList>()
.schema_from::<VlanInterfaceSpec>()
.schema_from::<ExternalDomain>()
.schema_from::<ExternalDomainList>()
.schema_from::<ExternalDomainSpec>()
.schema_from::<Provider>()
.schema_from::<ProviderList>()
.schema_from::<ProviderSpec>()
.schema_from::<HealthCheck>()
.schema_from::<HealthCheckList>()
.schema_from::<HealthCheckSpec>()
.schema_from::<StorageModule>()
.schema_from::<StorageModuleList>()
.schema_from::<StorageModuleSpec>()
.schema_from::<BladeEnclosure>()
.schema_from::<BladeEnclosureList>()
.schema_from::<BladeEnclosureSpec>()
.schema_from::<TunnelInterface>()
.schema_from::<TunnelInterfaceList>()
.schema_from::<TunnelInterfaceSpec>()
.schema_from::<RouteNetwork>()
.schema_from::<RouteNetworkList>()
.schema_from::<RouteNetworkSpec>()
.schema_from::<Datacenter>()
.schema_from::<DatacenterList>()
.schema_from::<DatacenterSpec>()
.schema_from::<MonitoringQueue>()
.schema_from::<MonitoringQueueList>()
.schema_from::<MonitoringQueueSpec>()
.schema_from::<NatGateway>()
.schema_from::<NatGatewayList>()
.schema_from::<NatGatewaySpec>()
.schema_from::<MediaDevice>()
.schema_from::<MediaDeviceList>()
.schema_from::<MediaDeviceSpec>()
.schema_from::<Project>()
.schema_from::<ProjectList>()
.schema_from::<ProjectSpec>()
.schema_from::<Natbox>()
.schema_from::<NatboxList>()
.schema_from::<NatboxSpec>()
.schema_from::<Acl>()
.schema_from::<AclList>()
.schema_from::<AclSpec>()
.schema_from::<RoomTablet>()
.schema_from::<RoomTabletList>()
.schema_from::<RoomTabletSpec>()
.schema_from::<AzureNetwork>()
.schema_from::<AzureNetworkList>()
.schema_from::<AzureNetworkSpec>()
.schema_from::<PublicDomain>()
.schema_from::<PublicDomainList>()
.schema_from::<PublicDomainSpec>()
.schema_from::<HwInternalrouter>()
.schema_from::<HwInternalrouterList>()
.schema_from::<HwInternalrouterSpec>()
.schema_from::<ProjectNetwork>()
.schema_from::<ProjectNetworkList>()
.schema_from::<ProjectNetworkSpec>()
.schema_from::<Vm>()
.schema_from::<VmList>()
.schema_from::<VmSpec>()
.schema_from::<Accesspoint>()
.schema_from::<AccesspointList>()
.schema_from::<AccesspointSpec>()
.schema_from::<Printer>()
.schema_from::<PrinterList>()
.schema_from::<PrinterSpec>()
.schema_from::<CloudNetwork>()
.schema_from::<CloudNetworkList>()
.schema_from::<CloudNetworkSpec>()
.schema_from::<Ipmi>()
.schema_from::<IpmiList>()
.schema_from::<IpmiSpec>()
.schema_from::<VoipPhone>()
.schema_from::<VoipPhoneList>()
.schema_from::<VoipPhoneSpec>()
.schema_from::<ProjectDomain>()
.schema_from::<ProjectDomainList>()
.schema_from::<ProjectDomainSpec>()
.schema_from::<Rack>()
.schema_from::<RackList>()
.schema_from::<RackSpec>()
.schema_from::<HardwareExternal>()
.schema_from::<HardwareExternalList>()
.schema_from::<HardwareExternalSpec>()
.schema_from::<Vpn>()
.schema_from::<VpnList>()
.schema_from::<VpnSpec>()
.schema_from::<ProviderDomain>()
.schema_from::<ProviderDomainList>()
.schema_from::<ProviderDomainSpec>()
.schema_from::<LbPool>()
.schema_from::<LbPoolList>()
.schema_from::<LbPoolSpec>()
.schema_from::<BondInterface>()
.schema_from::<BondInterfaceList>()
.schema_from::<BondInterfaceSpec>()
.schema_from::<FloatingAddress>()
.schema_from::<FloatingAddressList>()
.schema_from::<FloatingAddressSpec>()
.schema_from::<VmExternal>()
.schema_from::<VmExternalList>()
.schema_from::<VmExternalSpec>()
            .schema_from::<ServerObjectMetadata>()
            .security_scheme(
                "bearerAuth",
                utoipa::openapi::security::SecurityScheme::Http(
                    utoipa::openapi::security::Http::builder()
                        .scheme(utoipa::openapi::security::HttpAuthScheme::Bearer)
                        .build(),
                ),
            )
            .build(),
    ))
    .security(Some(vec![utoipa::openapi::SecurityRequirement::new::<
        _,
        _,
        String,
    >("bearerAuth", vec![])]))
    .build()
}
    
pub fn servertypes() -> std::collections::HashMap<String, Vec<Attribute>> {
        let mut servertypes = std::collections::HashMap::new();
        use AttributeType::*;

        servertypes.insert("memory_module".to_string(), { let mut vec = Vec::new(); vec.push(Attribute {
name: "capacity".to_string(),
typ: Number,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A[0-9]+\\Z".to_string(),
readonly: false,
hovertext: "Capacity of a memory or a storage module. Measured in B.".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "hardware_model".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "serial_number".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A[a-zA-Z0-9\\-]+\\Z".to_string(),
readonly: false,
hovertext: "A unique string identifying hardware, like:\r\n- Dell ServiceTag\r\n- Memory module serial number\r\n- Disk WWN address".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "state".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(retired|maintenance|cold_standby|deploy_offline|deploy_online|online_reserved|online)\\Z".to_string(),
readonly: false,
hovertext: "Retired: No DNS, no Nagios, lb downtimed\r\nMaintenance: DNS, Nagios downtimed, lb downtimed\r\nDeploy_offline: DNS, Nagios active, lb downtimed\r\nDeploy_online: DNS, Nagios downtimed, lb active\r\nOnline: DNS, Nagios active, lb active".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "installed_in".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A[a-z0-9\\-\\.]+\\Z".to_string(),
readonly: false,
hovertext: "A physical device in which the hardware component is installed.".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "hostname".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty hostname attribute. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "servertype".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty servertype attribute. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "object_id".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty object_id attribute. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
}); vec });
servertypes.insert("hdmi_transmitter".to_string(), { let mut vec = Vec::new(); vec.push(Attribute {
name: "state".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(retired|maintenance|cold_standby|deploy_offline|deploy_online|online_reserved|online)\\Z".to_string(),
readonly: false,
hovertext: "Retired: No DNS, no Nagios, lb downtimed\r\nMaintenance: DNS, Nagios downtimed, lb downtimed\r\nDeploy_offline: DNS, Nagios active, lb downtimed\r\nDeploy_online: DNS, Nagios downtimed, lb active\r\nOnline: DNS, Nagios active, lb active".to_string(),
required: true,
default: "online".to_string(),
});vec.push(Attribute {
name: "project".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: true,
default: "officeit".to_string(),
});vec.push(Attribute {
name: "mac".to_string(),
typ: Macaddr,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "route_network".to_string(),
typ: Supernet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: true,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "network_zones".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "A zone defining routing and firewall rules, a kind of sub-datacenter. It is a multi attribute for \"network_zone\"".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "officetool_id".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\Ai?[0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "ipv4".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty IPv4 address. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "ipv6".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty IPv6 address. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "hostname".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty hostname attribute. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "servertype".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty servertype attribute. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "object_id".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty object_id attribute. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
}); vec });
servertypes.insert("service_group".to_string(), { let mut vec = Vec::new(); vec.push(Attribute {
name: "function".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "Function of host".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "no_logging".to_string(),
typ: Boolean,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(true|false)\\Z".to_string(),
readonly: false,
hovertext: "To disable logging of traffic on firewall".to_string(),
required: false,
default: "true".to_string(),
});vec.push(Attribute {
name: "project".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "state".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(retired|maintenance|cold_standby|deploy_offline|deploy_online|online_reserved|online)\\Z".to_string(),
readonly: false,
hovertext: "Retired: No DNS, no Nagios, lb downtimed\r\nMaintenance: DNS, Nagios downtimed, lb downtimed\r\nDeploy_offline: DNS, Nagios active, lb downtimed\r\nDeploy_online: DNS, Nagios downtimed, lb active\r\nOnline: DNS, Nagios active, lb active".to_string(),
required: true,
default: "online".to_string(),
});vec.push(Attribute {
name: "subproject".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A[a-z][a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "responsible_admin".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A[a-z][a-z\\._]+[a-z]\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "sg_allow_from".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "sg_allow_from_reverse".to_string(),
typ: Relation,
reversed_attribute_id: "sg_allow_from".to_string(),
multi: true,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: true,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "sg_allow_to_reverse".to_string(),
typ: Relation,
reversed_attribute_id: "sg_allow_to".to_string(),
multi: true,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: true,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "sg_allow_to".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "service_group_members".to_string(),
typ: Relation,
reversed_attribute_id: "service_groups".to_string(),
multi: true,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: true,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "protocol_ports_inbound".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A((esp|ah|ping)|((tcp|udp)([0-9]+)(-[0-9]+)?([lt]+)?))\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "protocol_ports_outbound".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A((esp|ah|ping)|((tcp|udp)([0-9]+)(-[0-9]+)?([lt]+)?))\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "environment".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(testing|staging|production)\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "provider".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "hostname".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty hostname attribute. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "servertype".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty servertype attribute. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "object_id".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty object_id attribute. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
}); vec });
servertypes.insert("monitoring_zone".to_string(), { let mut vec = Vec::new(); vec.push(Attribute {
name: "hostname".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty hostname attribute. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "servertype".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty servertype attribute. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "object_id".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty object_id attribute. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
}); vec });
servertypes.insert("hw_loadbalancer".to_string(), { let mut vec = Vec::new(); vec.push(Attribute {
name: "order_date".to_string(),
typ: Date,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "When this server was ordered, for warranty and to find the order".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "order_id".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "some reference to the ticked, bill or order for this one".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "graphite_graphs".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A[a-z][a-z0-9_\\.\\-]*\\Z".to_string(),
readonly: false,
hovertext: "a list of graphs to reference which graphs should be displayed on this server to be used in the collections (doku link).".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "primary_ip6".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A[^\\.]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "switchports".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "Switchports attribute is used by Foreman to configure switches. That includes creating proper LAGGs and assigning VLANs to them. ".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "puppet_master".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "service_groups".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "no_monitoring".to_string(),
typ: Boolean,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(true|false)\\Z".to_string(),
readonly: false,
hovertext: "if set to true this server will be ignored by the nagios generation script".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "security_availability".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(L|M|H|C)\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "hw_loadbalancer_group".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "monitoring_groups".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "repositories".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A\\w+:\\w+:\\w+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "dell_servicetag".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "The Service Tag for Dell to be used in the support interface and to identify the hardware".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "storage_hardware".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A(raid|adaptec|threeware|perc700|perc710|perc730|perc310|perc330|perc200|sas1068|non-raid)\\Z".to_string(),
readonly: false,
hovertext: "The type of the raid controller if installed".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "security_confidentiality".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(L|M|H|C)\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "hardware_components".to_string(),
typ: Relation,
reversed_attribute_id: "installed_in".to_string(),
multi: true,
regexp: "\\A.*\\Z".to_string(),
readonly: true,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "num_cpu".to_string(),
typ: Number,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(0|[1-9][0-9]*)\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "disk_size_gib".to_string(),
typ: Number,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(0|[1-9][0-9]*)\\Z".to_string(),
readonly: false,
hovertext: "This is the current disk size of the server in gibibytes.  It should be filled while the VM is created or maintained by other scripts.\r\n".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "memory".to_string(),
typ: Number,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(0|[1-9][0-9]*)\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "security_integrity".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(L|M|H|C)\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "provisioning_step".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A([a-z0-9]+[_])*[a-z0-9]+\\|([a-z0-9]+[\\.\\-])*[a-z0-9]+\\|\\d{10}\\|\\d{10}\\.\\d{6}\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "cpu_clock_mhz".to_string(),
typ: Number,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(0|[1-9][0-9]*)\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "ipmi".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "function".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "Function of host".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "monitoring_checks".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A[a-z][a-z0-9_\\-@]*\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "ping".to_string(),
});vec.push(Attribute {
name: "bladecenter".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "puppet_environment".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(?!(testing|staging|production))[a-z][a-z0-9_]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "hardware_model".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "harddisk_type".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(ssd|hdd|unknown)\\Z".to_string(),
readonly: false,
hovertext: "this is either hdd or ssd to give an indication on the performance and io scheduler to be used".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "cpu_model".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "puppet_disabled".to_string(),
typ: Boolean,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(true|false)\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "false".to_string(),
});vec.push(Attribute {
name: "legal_personal_data".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(nothing|personal|highly_personal)\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "vlan_interfaces".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "memory_modules".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "bladecenter_slot".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "rack".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "responsible_admin".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A[a-z][a-z\\._]+[a-z]\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: true,
default: "kajetan.staszkiewicz".to_string(),
});vec.push(Attribute {
name: "network_zones".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "A zone defining routing and firewall rules, a kind of sub-datacenter. It is a multi attribute for \"network_zone\"".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "rack_height".to_string(),
typ: Number,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A[0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "rack_position".to_string(),
typ: Number,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A[0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "sshfp".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A[1-4] (1 [0-9a-f]{40}|2 [0-9a-f]{64})\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "firmware_versions".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A(BMC|BIOS|MIDPLANE)_[0-9a-z\\.]+\\Z".to_string(),
readonly: false,
hovertext: "Versions of various firmware components. See regexp for notation and supported attributes.".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "network_zone".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "A zone defining routing and firewall rules, a kind of sub-datacenter.".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "puppet_classes".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A[a-z][a-z0-9_]*(::[a-z][a-z0-9_]*)*\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "route_network".to_string(),
typ: Supernet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: true,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "project_network".to_string(),
typ: Supernet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: true,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "state".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(retired|maintenance|cold_standby|deploy_offline|deploy_online|online_reserved|online)\\Z".to_string(),
readonly: false,
hovertext: "Retired: No DNS, no Nagios, lb downtimed\r\nMaintenance: DNS, Nagios downtimed, lb downtimed\r\nDeploy_offline: DNS, Nagios active, lb downtimed\r\nDeploy_online: DNS, Nagios downtimed, lb active\r\nOnline: DNS, Nagios active, lb active".to_string(),
required: true,
default: "online".to_string(),
});vec.push(Attribute {
name: "environment".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(testing|staging|production)\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: true,
default: "production".to_string(),
});vec.push(Attribute {
name: "subproject".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A[a-z][a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "public_networks".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "monitoring_queue".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "Set the gearman_queue for a route_network".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "datacenter".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "vlan_networks".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "project".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: true,
default: "ndco".to_string(),
});vec.push(Attribute {
name: "datacenter_type".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(aws|digitalocean|hetzner|hyperv|kvm)\\.dct\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "puppet_ca".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "specifies the puppet ca server which the server should use".to_string(),
required: true,
default: "puppetca.innogames.de".to_string(),
});vec.push(Attribute {
name: "os".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(android|cumulus|centos64|wheezy|jessie|stretch|buster|bullseye|bookworm|bionic|freebsd_[0-9]+\\.[0-9]+|macos_.*|tvos|other|esx|ciscoios|ios12.2|iDRAC6|windows(2022_std|2019_std|2019_dc|2016_std|2016_dc|2012r2_std|2012r2_dc|2012_std|2012_dc|2008r2_std|2008r2_dc|7|8|10|11))\\Z".to_string(),
readonly: false,
hovertext: "Operation system, e.g. wheezy, jessie or freebsd100 or macos_*".to_string(),
required: true,
default: "freebsd_11.2".to_string(),
});vec.push(Attribute {
name: "arch".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: true,
default: "amd64".to_string(),
});vec.push(Attribute {
name: "mac".to_string(),
typ: Macaddr,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "repositories_host".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A([a-z0-9]+(-[a-z0-9]+)*\\.)+[a-z]{2,}\\Z".to_string(),
readonly: false,
hovertext: "Repositories hostname to fetch packages from. On Debian this is where apt fetches debian packages from (aptly). For FreeBSD this is where pkg fetches FreeBSD packages from.\r\nIt is used to form the correct configs on the hosts.".to_string(),
required: false,
default: "aptly.innogames.de".to_string(),
});vec.push(Attribute {
name: "network_link_type".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(kernel|ovs)_(single|bond_lacp|bond_ab)\\Z".to_string(),
readonly: false,
hovertext: "The type of physical connection between the hardware server and switches. The first part either 'ovs' if you use OpenVSwitch or 'kernel' if it's just plain Linux/Windows. Second part is either 'single' for devices with a single network cable or 'bond_ab' or 'bond_lacp' for servers with redundant network link.".to_string(),
required: true,
default: "kernel_bond_ab".to_string(),
});vec.push(Attribute {
name: "nessus_scan_id".to_string(),
typ: Number,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A[0-9]+\\Z".to_string(),
readonly: false,
hovertext: "The id of the scan in nessus".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "ipv4".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty IPv4 address. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "ipv6".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty IPv6 address. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "hostname".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty hostname attribute. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "servertype".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty servertype attribute. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "object_id".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty object_id attribute. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
}); vec });
servertypes.insert("hardware".to_string(), { let mut vec = Vec::new(); vec.push(Attribute {
name: "backup_state".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(idle|local_backup_running|local_backup_done|local_backup_failed|restic_running|restic_failed|waiting_local_backup_start|please_backup_now|dead)\\Z".to_string(),
readonly: false,
hovertext: "Current state of the backup. Used by backup system, might not be always 100% accurate (by design).".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "security_availability".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(L|M|H|C)\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "vlan_networks".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "repositories_host".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A([a-z0-9]+(-[a-z0-9]+)*\\.)+[a-z]{2,}\\Z".to_string(),
readonly: false,
hovertext: "Repositories hostname to fetch packages from. On Debian this is where apt fetches debian packages from (aptly). For FreeBSD this is where pkg fetches FreeBSD packages from.\r\nIt is used to form the correct configs on the hosts.".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "provisioning_step".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A([a-z0-9]+[_])*[a-z0-9]+\\|([a-z0-9]+[\\.\\-])*[a-z0-9]+\\|\\d{10}\\|\\d{10}\\.\\d{6}\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "rack_height".to_string(),
typ: Number,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A[0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "monitoring_queues".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "gearman_queues a worker should be responsible for".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "rack_position".to_string(),
typ: Number,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A[0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "security_confidentiality".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(L|M|H|C)\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "backup_prio".to_string(),
typ: Number,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(0|[1-9][0-9]*)\\Z".to_string(),
readonly: false,
hovertext: "Backup priority of the host. Defaults to 300. Backups will be eventually processed regardless of the value of this, but higher number means the dump and transfer of the backup will be prioritized.".to_string(),
required: false,
default: "300".to_string(),
});vec.push(Attribute {
name: "order_date".to_string(),
typ: Date,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "When this server was ordered, for warranty and to find the order".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "order_id".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "some reference to the ticked, bill or order for this one".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "no_monitoring".to_string(),
typ: Boolean,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(true|false)\\Z".to_string(),
readonly: false,
hovertext: "if set to true this server will be ignored by the nagios generation script".to_string(),
required: false,
default: "false".to_string(),
});vec.push(Attribute {
name: "puppet_disabled".to_string(),
typ: Boolean,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(true|false)\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "False".to_string(),
});vec.push(Attribute {
name: "harddisk_type".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(ssd|hdd|unknown)\\Z".to_string(),
readonly: false,
hovertext: "this is either hdd or ssd to give an indication on the performance and io scheduler to be used".to_string(),
required: true,
default: "ssd".to_string(),
});vec.push(Attribute {
name: "primary_ip6".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A[^\\.]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "num_cpu".to_string(),
typ: Number,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(0|[1-9][0-9]*)\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "loadbalancer".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "A list of LB Pools to which this Node belongs.".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "security_integrity".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(L|M|H|C)\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "disk_size_gib".to_string(),
typ: Number,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(0|[1-9][0-9]*)\\Z".to_string(),
readonly: false,
hovertext: "This is the current disk size of the server in gibibytes.  It should be filled while the VM is created or maintained by other scripts.\r\n".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "memory".to_string(),
typ: Number,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(0|[1-9][0-9]*)\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "legal_personal_data".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(nothing|personal|highly_personal)\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "state".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(retired|maintenance|cold_standby|deploy_offline|deploy_online|online_reserved|online)\\Z".to_string(),
readonly: false,
hovertext: "Retired: No DNS, no Nagios, lb downtimed\r\nMaintenance: DNS, Nagios downtimed, lb downtimed\r\nDeploy_offline: DNS, Nagios active, lb downtimed\r\nDeploy_online: DNS, Nagios downtimed, lb active\r\nOnline: DNS, Nagios active, lb active".to_string(),
required: true,
default: "online".to_string(),
});vec.push(Attribute {
name: "cpu_clock_mhz".to_string(),
typ: Number,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(0|[1-9][0-9]*)\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "ipmi".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "puppet_environment".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(?!(testing|staging|production))[a-z][a-z0-9_]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "function".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "Function of host".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "graphite_graphs".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A[a-z][a-z0-9_\\.\\-]*\\Z".to_string(),
readonly: false,
hovertext: "a list of graphs to reference which graphs should be displayed on this server to be used in the collections (doku link).".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "bladecenter".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "route_network".to_string(),
typ: Supernet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: true,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "dell_servicetag".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "The Service Tag for Dell to be used in the support interface and to identify the hardware".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "repositories".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A\\w+:\\w+:\\w+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "allow_to".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: true,
hovertext: "If set on any server in a segment it will block all access from servers in this segment to the internet unless they are explicitly given a value here.".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "environment".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(testing|staging|production)\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: true,
default: "production".to_string(),
});vec.push(Attribute {
name: "monitoring_queue".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "Set the gearman_queue for a route_network".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "hardware_model".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "provider_network".to_string(),
typ: Supernet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: true,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "cpu_model".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "memory_modules".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "bladecenter_slot".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "assigned_to".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "os_product_license".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "project_network".to_string(),
typ: Supernet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: true,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "responsible_admin".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A[a-z][a-z\\._]+[a-z]\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "subproject".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A[a-z][a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "network_zones".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "A zone defining routing and firewall rules, a kind of sub-datacenter. It is a multi attribute for \"network_zone\"".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "fixup_enabled".to_string(),
typ: Boolean,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(true|false)\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "description".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "Intended to give an external auditor a very general idea of what the system is about.".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "sshfp".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A[1-4] (1 [0-9a-f]{40}|2 [0-9a-f]{64})\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "puppet_classes".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A[a-z][a-z0-9_]*(::[a-z][a-z0-9_]*)*\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "monitoring_checks".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A[a-z][a-z0-9_\\-@]*\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "ping".to_string(),
});vec.push(Attribute {
name: "apt_repository_url".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A([a-z0-9]+(-[a-z0-9]+)*\\.)+[a-z]{2,}\\Z".to_string(),
readonly: false,
hovertext: "Debian repository URL for apt e.g. update.innogames.de which will be used to form the entries in /etc/apt/sources.list together with repositories attribute.".to_string(),
required: false,
default: "aptly.innogames.de".to_string(),
});vec.push(Attribute {
name: "allow_from".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: true,
hovertext: "List of Access lists from which access to this LB Pool is allowed. Connections from outside of allowed networks will be silently dropped.".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "datacenter".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "acl".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: true,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "network_zone".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "A zone defining routing and firewall rules, a kind of sub-datacenter.".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "monitoring_groups".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "service_groups".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "backup_storage".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "Storage server for the host's backups. This is normally auto-filled by the system while doing the first backup. Changes needs a puppet run on both the host and new storage.".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "project".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "restic_keep_monthly".to_string(),
typ: Number,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A[1-9][0-9]?$|^100\\Z".to_string(),
readonly: false,
hovertext: "Monthly backup count to keep for this host, 1-100".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "restic_keep_yearly".to_string(),
typ: Number,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A[0-9]$|^10\\Z".to_string(),
readonly: false,
hovertext: "Yearly backup count to keep for this host, 1-10".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "firmware_versions".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A(BMC|BIOS|MIDPLANE)_[0-9a-z\\.]+\\Z".to_string(),
readonly: false,
hovertext: "Versions of various firmware components. See regexp for notation and supported attributes.".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "restic_group".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A[a-z0-9][a-z0-9-]+[a-z0-9]\\Z".to_string(),
readonly: false,
hovertext: "Name of the backup bucket of this server, project (+ \"-subproject\" if defined) when empty by default".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "restic_keep_daily".to_string(),
typ: Number,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A[1-9][0-9]?$|^100\\Z".to_string(),
readonly: false,
hovertext: "Daily backup count to keep for this host, 1-100".to_string(),
required: false,
default: "3".to_string(),
});vec.push(Attribute {
name: "restic_keep_last".to_string(),
typ: Number,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A([1-3][0-9][0-9]|[0-9][0-9]|[0-9])\\Z".to_string(),
readonly: false,
hovertext: "Latest backup count to keep for this host, 1-399".to_string(),
required: false,
default: "3".to_string(),
});vec.push(Attribute {
name: "restic_keep_weekly".to_string(),
typ: Number,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A[1-9][0-9]?$|^100\\Z".to_string(),
readonly: false,
hovertext: "Weekly backup count to keep for this host, 1-100".to_string(),
required: false,
default: "2".to_string(),
});vec.push(Attribute {
name: "datacenter_type".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(aws|digitalocean|hetzner|hyperv|kvm)\\.dct\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "arch".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "auto_update".to_string(),
typ: Boolean,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(true|false)\\Z".to_string(),
readonly: false,
hovertext: "This is used by automated scripts to decide if this server should be automatically updated in certain situations".to_string(),
required: true,
default: "1".to_string(),
});vec.push(Attribute {
name: "storage_hardware".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A(raid|adaptec|threeware|perc700|perc710|perc730|perc310|perc330|perc200|sas1068|non-raid)\\Z".to_string(),
readonly: false,
hovertext: "The type of the raid controller if installed".to_string(),
required: true,
default: "raid".to_string(),
});vec.push(Attribute {
name: "backup_disabled".to_string(),
typ: Boolean,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(true|false)\\Z".to_string(),
readonly: false,
hovertext: "Set to true if you do not want this server backed up.".to_string(),
required: true,
default: "1".to_string(),
});vec.push(Attribute {
name: "backup_canshrink".to_string(),
typ: Boolean,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(true|false)\\Z".to_string(),
readonly: false,
hovertext: "Monitoring system will warn in case last backup size of the host drops below 50% of its last 10 backups' median size. Set this to true if you expect a shrinking backup (e.g. on \"speed worlds\" which are reset from time to time) so it doesn't complain on suddenly shrinking backup size.".to_string(),
required: false,
default: "0".to_string(),
});vec.push(Attribute {
name: "backup_maxage".to_string(),
typ: Number,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(0|[1-9][0-9]*)\\Z".to_string(),
readonly: false,
hovertext: "The expected freshness of the latest registered backup of the host. Defaults to 24 hours. If you have multiple backup windows, you might consider lowering this. 0 is a special value, disables backup age checking completely.".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "backup_window".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A\\d+-\\d+\\Z".to_string(),
readonly: false,
hovertext: "This is a list of hour ranges (in CEST) when the backup should be scheduled on the server locally. E.g. \"3-4\" means create one backup between 03:00 and 04:00. The backups will be transferred in order of backup_prio attribute once the local script ran through and reported a success so not guaranteed to be within the backup_window (but mostly does fit in).".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "powerports".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "switchports".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "Switchports attribute is used by Foreman to configure switches. That includes creating proper LAGGs and assigning VLANs to them. ".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "mac".to_string(),
typ: Macaddr,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "os".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(android|cumulus|centos64|wheezy|jessie|stretch|buster|bullseye|bookworm|bionic|freebsd_[0-9]+\\.[0-9]+|macos_.*|tvos|other|esx|ciscoios|ios12.2|iDRAC6|windows(2022_std|2019_std|2019_dc|2016_std|2016_dc|2012r2_std|2012r2_dc|2012_std|2012_dc|2008r2_std|2008r2_dc|7|8|10|11))\\Z".to_string(),
readonly: false,
hovertext: "Operation system, e.g. wheezy, jessie or freebsd100 or macos_*".to_string(),
required: true,
default: "stretch".to_string(),
});vec.push(Attribute {
name: "backup_timestamp".to_string(),
typ: Datetime,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "Last action time of backup system (UTC). Should be set automatically by backup manager.\r\nWe're utilizing this like a cache to quickly check the freshness of the state (real source of truth is the client itself, but not always).".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "network_link_type".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(kernel|ovs)_(single|bond_lacp|bond_ab)\\Z".to_string(),
readonly: false,
hovertext: "The type of physical connection between the hardware server and switches. The first part either 'ovs' if you use OpenVSwitch or 'kernel' if it's just plain Linux/Windows. Second part is either 'single' for devices with a single network cable or 'bond_ab' or 'bond_lacp' for servers with redundant network link.".to_string(),
required: true,
default: "kernel_bond_ab".to_string(),
});vec.push(Attribute {
name: "nessus_scan_id".to_string(),
typ: Number,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A[0-9]+\\Z".to_string(),
readonly: false,
hovertext: "The id of the scan in nessus".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "puppet_ca".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "specifies the puppet ca server which the server should use".to_string(),
required: false,
default: "puppetca.innogames.de".to_string(),
});vec.push(Attribute {
name: "puppet_master".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "rack".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "officetool_id".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\Ai?[0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "serial_number".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A[a-zA-Z0-9\\-]+\\Z".to_string(),
readonly: false,
hovertext: "A unique string identifying hardware, like:\r\n- Dell ServiceTag\r\n- Memory module serial number\r\n- Disk WWN address".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "ipv4".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty IPv4 address. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "ipv6".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty IPv6 address. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "hostname".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty hostname attribute. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "servertype".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty servertype attribute. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "object_id".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty object_id attribute. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
}); vec });
servertypes.insert("hypervisor".to_string(), { let mut vec = Vec::new(); vec.push(Attribute {
name: "provisioning_step".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A([a-z0-9]+[_])*[a-z0-9]+\\|([a-z0-9]+[\\.\\-])*[a-z0-9]+\\|\\d{10}\\|\\d{10}\\.\\d{6}\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "graphite_graphs".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A[a-z][a-z0-9_\\.\\-]*\\Z".to_string(),
readonly: false,
hovertext: "a list of graphs to reference which graphs should be displayed on this server to be used in the collections (doku link).".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "route_network".to_string(),
typ: Supernet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: true,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "igvm_locked".to_string(),
typ: Datetime,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "Set when object is being manipulated by igvm, e.g. being build or migrated.".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "responsible_admin".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A[a-z][a-z\\._]+[a-z]\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "firmware_versions".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A(BMC|BIOS|MIDPLANE)_[0-9a-z\\.]+\\Z".to_string(),
readonly: false,
hovertext: "Versions of various firmware components. See regexp for notation and supported attributes.".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "cpu_util_pct_tmp".to_string(),
typ: Number,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(0|[1-9]{1,2})\\Z".to_string(),
readonly: true,
hovertext: "".to_string(),
required: true,
default: "0".to_string(),
});vec.push(Attribute {
name: "puppet_master".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "backup_state".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(idle|local_backup_running|local_backup_done|local_backup_failed|restic_running|restic_failed|waiting_local_backup_start|please_backup_now|dead)\\Z".to_string(),
readonly: false,
hovertext: "Current state of the backup. Used by backup system, might not be always 100% accurate (by design).".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "vms".to_string(),
typ: Relation,
reversed_attribute_id: "hypervisor".to_string(),
multi: true,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: true,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "repositories".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A\\w+:\\w+:\\w+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "backup_prio".to_string(),
typ: Number,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(0|[1-9][0-9]*)\\Z".to_string(),
readonly: false,
hovertext: "Backup priority of the host. Defaults to 300. Backups will be eventually processed regardless of the value of this, but higher number means the dump and transfer of the backup will be prioritized.".to_string(),
required: false,
default: "300".to_string(),
});vec.push(Attribute {
name: "order_date".to_string(),
typ: Date,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "When this server was ordered, for warranty and to find the order".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "order_id".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "some reference to the ticked, bill or order for this one".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "no_monitoring".to_string(),
typ: Boolean,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(true|false)\\Z".to_string(),
readonly: false,
hovertext: "if set to true this server will be ignored by the nagios generation script".to_string(),
required: false,
default: "false".to_string(),
});vec.push(Attribute {
name: "puppet_disabled".to_string(),
typ: Boolean,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(true|false)\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "False".to_string(),
});vec.push(Attribute {
name: "harddisk_type".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(ssd|hdd|unknown)\\Z".to_string(),
readonly: false,
hovertext: "this is either hdd or ssd to give an indication on the performance and io scheduler to be used".to_string(),
required: true,
default: "ssd".to_string(),
});vec.push(Attribute {
name: "security_availability".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(L|M|H|C)\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "primary_ip6".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A[^\\.]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "monitoring_groups".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "security_confidentiality".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(L|M|H|C)\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "num_cpu".to_string(),
typ: Number,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(0|[1-9][0-9]*)\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "memory".to_string(),
typ: Number,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(0|[1-9][0-9]*)\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "disk_size_gib".to_string(),
typ: Number,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(0|[1-9][0-9]*)\\Z".to_string(),
readonly: false,
hovertext: "This is the current disk size of the server in gibibytes.  It should be filled while the VM is created or maintained by other scripts.\r\n".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "security_integrity".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(L|M|H|C)\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "cpu_clock_mhz".to_string(),
typ: Number,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(0|[1-9][0-9]*)\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "bladecenter".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "ipmi".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "function".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "Function of host".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "dell_servicetag".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "The Service Tag for Dell to be used in the support interface and to identify the hardware".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "puppet_environment".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(?!(testing|staging|production))[a-z][a-z0-9_]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "environment".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(testing|staging|production)\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: true,
default: "production".to_string(),
});vec.push(Attribute {
name: "cpu_util_vm_pct".to_string(),
typ: Number,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(0|[1-9][0-9]*)\\Z".to_string(),
readonly: true,
hovertext: "Show the maximum vCPU usage of the last 7 days".to_string(),
required: true,
default: "0".to_string(),
});vec.push(Attribute {
name: "cpu_util_pct".to_string(),
typ: Number,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(0|[1-9]{1,2})\\Z".to_string(),
readonly: true,
hovertext: "".to_string(),
required: true,
default: "0".to_string(),
});vec.push(Attribute {
name: "hardware_model".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "cpu_model".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "provider_network".to_string(),
typ: Supernet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: true,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "legal_personal_data".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(nothing|personal|highly_personal)\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "memory_usage_gib".to_string(),
typ: Number,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(0|[1-9][0-9]*)\\Z".to_string(),
readonly: true,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "disk_free_gib".to_string(),
typ: Number,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(0|[1-9][0-9]*)\\Z".to_string(),
readonly: true,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "load_avg".to_string(),
typ: Number,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(0|[1-9][0-9]*)\\Z".to_string(),
readonly: true,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "iops_avg".to_string(),
typ: Number,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(0|[1-9][0-9]*)\\Z".to_string(),
readonly: true,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "memory_modules".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "bladecenter_slot".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "assigned_to".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "os_product_license".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "sshfp".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A[1-4] (1 [0-9a-f]{40}|2 [0-9a-f]{64})\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "hardware_components".to_string(),
typ: Relation,
reversed_attribute_id: "installed_in".to_string(),
multi: true,
regexp: "\\A.*\\Z".to_string(),
readonly: true,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "monitoring_checks".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A[a-z][a-z0-9_\\-@]*\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "ping".to_string(),
});vec.push(Attribute {
name: "puppet_classes".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A[a-z][a-z0-9_]*(::[a-z][a-z0-9_]*)*\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "rack_height".to_string(),
typ: Number,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A[0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "rack_position".to_string(),
typ: Number,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A[0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "vlan_networks".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "rack".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "state".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(retired|maintenance|cold_standby|deploy_offline|deploy_online|online_reserved|online)\\Z".to_string(),
readonly: false,
hovertext: "Retired: No DNS, no Nagios, lb downtimed\r\nMaintenance: DNS, Nagios downtimed, lb downtimed\r\nDeploy_offline: DNS, Nagios active, lb downtimed\r\nDeploy_online: DNS, Nagios downtimed, lb active\r\nOnline: DNS, Nagios active, lb active".to_string(),
required: true,
default: "online".to_string(),
});vec.push(Attribute {
name: "project_network".to_string(),
typ: Supernet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: true,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "subproject".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A[a-z][a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "network_zone".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "A zone defining routing and firewall rules, a kind of sub-datacenter.".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "monitoring_queue".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "Set the gearman_queue for a route_network".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "network_zones".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "A zone defining routing and firewall rules, a kind of sub-datacenter. It is a multi attribute for \"network_zone\"".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "datacenter".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "service_groups".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "backup_storage".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "Storage server for the host's backups. This is normally auto-filled by the system while doing the first backup. Changes needs a puppet run on both the host and new storage.".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "project".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: true,
default: "ndco".to_string(),
});vec.push(Attribute {
name: "libvirt_pool_free_gib".to_string(),
typ: Number,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(0|[1-9][0-9]*)\\Z".to_string(),
readonly: true,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "libvirt_pool_total_gib".to_string(),
typ: Number,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(0|[1-9][0-9]*)\\Z".to_string(),
readonly: true,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "libvirt_memory_total_gib".to_string(),
typ: Number,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(0|[1-9][0-9]*)\\Z".to_string(),
readonly: true,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "libvirt_memory_free_gib".to_string(),
typ: Number,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(0|[1-9][0-9]*)\\Z".to_string(),
readonly: true,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "libvirt_memory_used_gib".to_string(),
typ: Number,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(0|[1-9][0-9]*)\\Z".to_string(),
readonly: true,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "libvirt_pool_used_gib".to_string(),
typ: Number,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(0|[1-9][0-9]*)\\Z".to_string(),
readonly: true,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "restic_keep_monthly".to_string(),
typ: Number,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A[1-9][0-9]?$|^100\\Z".to_string(),
readonly: false,
hovertext: "Monthly backup count to keep for this host, 1-100".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "restic_keep_yearly".to_string(),
typ: Number,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A[0-9]$|^10\\Z".to_string(),
readonly: false,
hovertext: "Yearly backup count to keep for this host, 1-10".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "datacenter_type".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(aws|digitalocean|hetzner|hyperv|kvm)\\.dct\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "restic_group".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A[a-z0-9][a-z0-9-]+[a-z0-9]\\Z".to_string(),
readonly: false,
hovertext: "Name of the backup bucket of this server, project (+ \"-subproject\" if defined) when empty by default".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "puppet_ca".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "specifies the puppet ca server which the server should use".to_string(),
required: true,
default: "puppetca.innogames.de".to_string(),
});vec.push(Attribute {
name: "restic_keep_daily".to_string(),
typ: Number,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A[1-9][0-9]?$|^100\\Z".to_string(),
readonly: false,
hovertext: "Daily backup count to keep for this host, 1-100".to_string(),
required: false,
default: "3".to_string(),
});vec.push(Attribute {
name: "restic_keep_last".to_string(),
typ: Number,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A([1-3][0-9][0-9]|[0-9][0-9]|[0-9])\\Z".to_string(),
readonly: false,
hovertext: "Latest backup count to keep for this host, 1-399".to_string(),
required: false,
default: "3".to_string(),
});vec.push(Attribute {
name: "restic_keep_weekly".to_string(),
typ: Number,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A[1-9][0-9]?$|^100\\Z".to_string(),
readonly: false,
hovertext: "Weekly backup count to keep for this host, 1-100".to_string(),
required: false,
default: "2".to_string(),
});vec.push(Attribute {
name: "loadbalancer".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "A list of LB Pools to which this Node belongs.".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "arch".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "auto_update".to_string(),
typ: Boolean,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(true|false)\\Z".to_string(),
readonly: false,
hovertext: "This is used by automated scripts to decide if this server should be automatically updated in certain situations".to_string(),
required: true,
default: "1".to_string(),
});vec.push(Attribute {
name: "storage_hardware".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A(raid|adaptec|threeware|perc700|perc710|perc730|perc310|perc330|perc200|sas1068|non-raid)\\Z".to_string(),
readonly: false,
hovertext: "The type of the raid controller if installed".to_string(),
required: true,
default: "perc310".to_string(),
});vec.push(Attribute {
name: "backup_disabled".to_string(),
typ: Boolean,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(true|false)\\Z".to_string(),
readonly: false,
hovertext: "Set to true if you do not want this server backed up.".to_string(),
required: true,
default: "1".to_string(),
});vec.push(Attribute {
name: "backup_canshrink".to_string(),
typ: Boolean,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(true|false)\\Z".to_string(),
readonly: false,
hovertext: "Monitoring system will warn in case last backup size of the host drops below 50% of its last 10 backups' median size. Set this to true if you expect a shrinking backup (e.g. on \"speed worlds\" which are reset from time to time) so it doesn't complain on suddenly shrinking backup size.".to_string(),
required: false,
default: "0".to_string(),
});vec.push(Attribute {
name: "backup_maxage".to_string(),
typ: Number,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(0|[1-9][0-9]*)\\Z".to_string(),
readonly: false,
hovertext: "The expected freshness of the latest registered backup of the host. Defaults to 24 hours. If you have multiple backup windows, you might consider lowering this. 0 is a special value, disables backup age checking completely.".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "backup_window".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A\\d+-\\d+\\Z".to_string(),
readonly: false,
hovertext: "This is a list of hour ranges (in CEST) when the backup should be scheduled on the server locally. E.g. \"3-4\" means create one backup between 03:00 and 04:00. The backups will be transferred in order of backup_prio attribute once the local script ran through and reported a success so not guaranteed to be within the backup_window (but mostly does fit in).".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "powerports".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "switchports".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "Switchports attribute is used by Foreman to configure switches. That includes creating proper LAGGs and assigning VLANs to them. ".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "mac".to_string(),
typ: Macaddr,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "os".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(android|cumulus|centos64|wheezy|jessie|stretch|buster|bullseye|bookworm|bionic|freebsd_[0-9]+\\.[0-9]+|macos_.*|tvos|other|esx|ciscoios|ios12.2|iDRAC6|windows(2022_std|2019_std|2019_dc|2016_std|2016_dc|2012r2_std|2012r2_dc|2012_std|2012_dc|2008r2_std|2008r2_dc|7|8|10|11))\\Z".to_string(),
readonly: false,
hovertext: "Operation system, e.g. wheezy, jessie or freebsd100 or macos_*".to_string(),
required: true,
default: "wheezy".to_string(),
});vec.push(Attribute {
name: "igvm_migration_log".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A[0-9]{10}\\s[-|+][0-9]{1,3}\\Z".to_string(),
readonly: false,
hovertext: "Log for IGVM migrations to properly calculate the destination load of a hypervisor.".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "cpu_perffactor".to_string(),
typ: Number,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A[0-9]+\\Z".to_string(),
readonly: true,
hovertext: "".to_string(),
required: false,
default: "1000".to_string(),
});vec.push(Attribute {
name: "backup_timestamp".to_string(),
typ: Datetime,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "Last action time of backup system (UTC). Should be set automatically by backup manager.\r\nWe're utilizing this like a cache to quickly check the freshness of the state (real source of truth is the client itself, but not always).".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "repositories_host".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A([a-z0-9]+(-[a-z0-9]+)*\\.)+[a-z]{2,}\\Z".to_string(),
readonly: false,
hovertext: "Repositories hostname to fetch packages from. On Debian this is where apt fetches debian packages from (aptly). For FreeBSD this is where pkg fetches FreeBSD packages from.\r\nIt is used to form the correct configs on the hosts.".to_string(),
required: false,
default: "aptly.innogames.de".to_string(),
});vec.push(Attribute {
name: "network_link_type".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(kernel|ovs)_(single|bond_lacp|bond_ab)\\Z".to_string(),
readonly: false,
hovertext: "The type of physical connection between the hardware server and switches. The first part either 'ovs' if you use OpenVSwitch or 'kernel' if it's just plain Linux/Windows. Second part is either 'single' for devices with a single network cable or 'bond_ab' or 'bond_lacp' for servers with redundant network link.".to_string(),
required: true,
default: "ovs_bond_ab".to_string(),
});vec.push(Attribute {
name: "officetool_id".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\Ai?[0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "serial_number".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A[a-zA-Z0-9\\-]+\\Z".to_string(),
readonly: false,
hovertext: "A unique string identifying hardware, like:\r\n- Dell ServiceTag\r\n- Memory module serial number\r\n- Disk WWN address".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "ipv4".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty IPv4 address. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "ipv6".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty IPv6 address. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "hostname".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty hostname attribute. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "servertype".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty servertype attribute. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "object_id".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty object_id attribute. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
}); vec });
servertypes.insert("external_network".to_string(), { let mut vec = Vec::new(); vec.push(Attribute {
name: "acl".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: true,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "state".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(retired|maintenance|cold_standby|deploy_offline|deploy_online|online_reserved|online)\\Z".to_string(),
readonly: false,
hovertext: "Retired: No DNS, no Nagios, lb downtimed\r\nMaintenance: DNS, Nagios downtimed, lb downtimed\r\nDeploy_offline: DNS, Nagios active, lb downtimed\r\nDeploy_online: DNS, Nagios downtimed, lb active\r\nOnline: DNS, Nagios active, lb active".to_string(),
required: true,
default: "online".to_string(),
});vec.push(Attribute {
name: "service_groups".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "primary_ip6".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A[^\\.]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "ipv4".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty IPv4 address. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "ipv6".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty IPv6 address. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "hostname".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty hostname attribute. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "servertype".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty servertype attribute. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "object_id".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty object_id attribute. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
}); vec });
servertypes.insert("loadbalancer".to_string(), { let mut vec = Vec::new(); vec.push(Attribute {
name: "network_type".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(loopback|transit_(internal_igp|public_(hwlbs|uplink|igp))|public|internal|public_dmz|vpn_clients|internal_vpn_clients|cloud)\\Z".to_string(),
readonly: false,
hovertext: "Select what kind of connection you want to switch\r\nsingle: single interface without lacp or anything other special\r\nbond_ab: Bonding without LACP but 2 links with failover\r\nbond_lacp: bonding with lacp and 2 active links".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "project_network".to_string(),
typ: Supernet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: true,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "backup_pool_of".to_string(),
typ: Relation,
reversed_attribute_id: "backup_pool".to_string(),
multi: true,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: true,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "no_monitoring".to_string(),
typ: Boolean,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(true|false)\\Z".to_string(),
readonly: false,
hovertext: "if set to true this server will be ignored by the nagios generation script".to_string(),
required: false,
default: "0".to_string(),
});vec.push(Attribute {
name: "state_limit".to_string(),
typ: Number,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(0|[1-9][0-9]*)\\Z".to_string(),
readonly: false,
hovertext: "Maximum amount of states which can be created by traffic to this LB Pool. Any new traffic above this number will be silently dropped.".to_string(),
required: true,
default: "15000".to_string(),
});vec.push(Attribute {
name: "graphite_graphs".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A[a-z][a-z0-9_\\.\\-]*\\Z".to_string(),
readonly: false,
hovertext: "a list of graphs to reference which graphs should be displayed on this server to be used in the collections (doku link).".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "backup_pool".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "The name of another LB Pool to which the traffic for this one should be redirected once there is no active node left.".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "primary_ip6".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A[^\\.]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "allow_from".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: true,
hovertext: "List of Access lists from which access to this LB Pool is allowed. Connections from outside of allowed networks will be silently dropped.".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "symmetric_nat".to_string(),
typ: Boolean,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(true|false)\\Z".to_string(),
readonly: false,
hovertext: "This attribute will create a rule causing any Nodes belonging to this Pool to use its IP address for outgoing NAT.".to_string(),
required: false,
default: "true".to_string(),
});vec.push(Attribute {
name: "group_company".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A[a-zA-Z]+\\Z".to_string(),
readonly: false,
hovertext: "Company this host is serving e.g. Hutch, Congregate (Default: InnoGames)".to_string(),
required: false,
default: "InnoGames".to_string(),
});vec.push(Attribute {
name: "min_nodes".to_string(),
typ: Number,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(0|[1-9][0-9]*)\\Z".to_string(),
readonly: false,
hovertext: "Minimum number of alive hosts required. If number goes below this value, min_nodes_action is checked".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "domain".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A(\\*\\.)?([a-z0-9]+(\\.|-+))*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "list of all domains which are assigned to this server or load balancer".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "function".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "Function of host".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "protocol_port".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A((any|esp|ah)|((tcp|udp)([0-9]+([lt]+)?|[0-9]+(-[0-9]+)?(_[0-9]+)?))|((ping)|((tcp|udp)([0-9]+t?(-[0-9]+)?))))\\Z".to_string(),
readonly: true,
hovertext: "Ports and protocols to be forwarded to Nodes belonging to this LB Pool.".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "min_nodes_action".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(force_down|force_up|backup_pool)\\Z".to_string(),
readonly: false,
hovertext: "force_down: switch to backup_pool if active nodes drop below min_nodes\r\nforce_up: always send traffic to last min_nodes hosts, even if they are considered offline".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "game_type".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "Currently only used to mark Tribalwars Speed (s), Classic (c), and Casual (p) special worlds".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "default_snat".to_string(),
typ: Boolean,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(true|false)\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "max_nodes".to_string(),
typ: Number,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(0|[1-9][0-9]*)\\Z".to_string(),
readonly: false,
hovertext: "Maximum number of nodes serving pool's traffic even if more are alive.".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "responsible_admin".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A[a-z][a-z\\._]+[a-z]\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "acl".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: true,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "monitoring_queue".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "Set the gearman_queue for a route_network".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "state".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(retired|maintenance|cold_standby|deploy_offline|deploy_online|online_reserved|online)\\Z".to_string(),
readonly: false,
hovertext: "Retired: No DNS, no Nagios, lb downtimed\r\nMaintenance: DNS, Nagios downtimed, lb downtimed\r\nDeploy_offline: DNS, Nagios active, lb downtimed\r\nDeploy_online: DNS, Nagios downtimed, lb active\r\nOnline: DNS, Nagios active, lb active".to_string(),
required: true,
default: "online".to_string(),
});vec.push(Attribute {
name: "lb_nodes".to_string(),
typ: Relation,
reversed_attribute_id: "loadbalancer".to_string(),
multi: true,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: true,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "health_checks".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "monitoring_checks".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A[a-z][a-z0-9_\\-@]*\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "provider_network".to_string(),
typ: Supernet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: true,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "route_network".to_string(),
typ: Supernet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: true,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "subproject".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A[a-z][a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "monitoring_queues".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "gearman_queues a worker should be responsible for".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "environment".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(testing|staging|production)\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: true,
default: "production".to_string(),
});vec.push(Attribute {
name: "datacenter".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "backup_storage".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "Storage server for the host's backups. This is normally auto-filled by the system while doing the first backup. Changes needs a puppet run on both the host and new storage.".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "service_groups".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "project".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "project_domain".to_string(),
typ: Domain,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: true,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "network_zones".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "A zone defining routing and firewall rules, a kind of sub-datacenter. It is a multi attribute for \"network_zone\"".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "monitoring_groups".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "game_market".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A[a-z][a-z]|foe|war\\Z".to_string(),
readonly: false,
hovertext: "The 2 letter short code for the game market, see docu for allowed values.".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "game_world".to_string(),
typ: Number,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(0|[1-9][0-9]*)\\Z".to_string(),
readonly: false,
hovertext: "The numerical part of the world id for games 0 is reserved for master or servers with market wide functionality".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "network_zone".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "A zone defining routing and firewall rules, a kind of sub-datacenter.".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "nessus_scan_id".to_string(),
typ: Number,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A[0-9]+\\Z".to_string(),
readonly: false,
hovertext: "The id of the scan in nessus".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "ipv4".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty IPv4 address. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "ipv6".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty IPv6 address. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "hostname".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty hostname attribute. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "servertype".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty servertype attribute. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "object_id".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty object_id attribute. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
}); vec });
servertypes.insert("switch".to_string(), { let mut vec = Vec::new(); vec.push(Attribute {
name: "datacenter_type".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(aws|digitalocean|hetzner|hyperv|kvm)\\.dct\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "provider_network".to_string(),
typ: Supernet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: true,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "order_date".to_string(),
typ: Date,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "When this server was ordered, for warranty and to find the order".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "order_id".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "some reference to the ticked, bill or order for this one".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "puppet_disabled".to_string(),
typ: Boolean,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(true|false)\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "True".to_string(),
});vec.push(Attribute {
name: "route_network".to_string(),
typ: Supernet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: true,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "primary_ip6".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A[^\\.]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "vlan_networks".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "mac".to_string(),
typ: Macaddr,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "powerports".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "switchports".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "Switchports attribute is used by Foreman to configure switches. That includes creating proper LAGGs and assigning VLANs to them. ".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "no_monitoring".to_string(),
typ: Boolean,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(true|false)\\Z".to_string(),
readonly: false,
hovertext: "if set to true this server will be ignored by the nagios generation script".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "datacenter".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "project_network".to_string(),
typ: Supernet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: true,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "dell_servicetag".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "The Service Tag for Dell to be used in the support interface and to identify the hardware".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "sshfp".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A[1-4] (1 [0-9a-f]{40}|2 [0-9a-f]{64})\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "rack_height".to_string(),
typ: Number,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A[0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "rack_position".to_string(),
typ: Number,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A[0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "function".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "Function of host".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "hardware_model".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "bladecenter".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "vlan_interfaces".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "bladecenter_slot".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "environment".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(testing|staging|production)\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: true,
default: "production".to_string(),
});vec.push(Attribute {
name: "responsible_admin".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A[a-z][a-z\\._]+[a-z]\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: true,
default: "kajetan.staszkiewicz".to_string(),
});vec.push(Attribute {
name: "state".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(retired|maintenance|cold_standby|deploy_offline|deploy_online|online_reserved|online)\\Z".to_string(),
readonly: false,
hovertext: "Retired: No DNS, no Nagios, lb downtimed\r\nMaintenance: DNS, Nagios downtimed, lb downtimed\r\nDeploy_offline: DNS, Nagios active, lb downtimed\r\nDeploy_online: DNS, Nagios downtimed, lb active\r\nOnline: DNS, Nagios active, lb active".to_string(),
required: true,
default: "online".to_string(),
});vec.push(Attribute {
name: "network_zone".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "A zone defining routing and firewall rules, a kind of sub-datacenter.".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "rack".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "monitoring_queue".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "Set the gearman_queue for a route_network".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "project".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: true,
default: "ndco".to_string(),
});vec.push(Attribute {
name: "puppet_environment".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(?!(testing|staging|production))[a-z][a-z0-9_]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "network_zones".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "A zone defining routing and firewall rules, a kind of sub-datacenter. It is a multi attribute for \"network_zone\"".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "puppet_classes".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A[a-z][a-z0-9_]*(::[a-z][a-z0-9_]*)*\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "puppet_master".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "puppet_ca".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "specifies the puppet ca server which the server should use".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "os".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(android|cumulus|centos64|wheezy|jessie|stretch|buster|bullseye|bookworm|bionic|freebsd_[0-9]+\\.[0-9]+|macos_.*|tvos|other|esx|ciscoios|ios12.2|iDRAC6|windows(2022_std|2019_std|2019_dc|2016_std|2016_dc|2012r2_std|2012r2_dc|2012_std|2012_dc|2008r2_std|2008r2_dc|7|8|10|11))\\Z".to_string(),
readonly: false,
hovertext: "Operation system, e.g. wheezy, jessie or freebsd100 or macos_*".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "graphite_graphs".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A[a-z][a-z0-9_\\.\\-]*\\Z".to_string(),
readonly: false,
hovertext: "a list of graphs to reference which graphs should be displayed on this server to be used in the collections (doku link).".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "repositories".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A\\w+:\\w+:\\w+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "num_cpu".to_string(),
typ: Number,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(0|[1-9][0-9]*)\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "memory".to_string(),
typ: Number,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(0|[1-9][0-9]*)\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "disk_size_gib".to_string(),
typ: Number,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(0|[1-9][0-9]*)\\Z".to_string(),
readonly: false,
hovertext: "This is the current disk size of the server in gibibytes.  It should be filled while the VM is created or maintained by other scripts.\r\n".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "repositories_host".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A([a-z0-9]+(-[a-z0-9]+)*\\.)+[a-z]{2,}\\Z".to_string(),
readonly: false,
hovertext: "Repositories hostname to fetch packages from. On Debian this is where apt fetches debian packages from (aptly). For FreeBSD this is where pkg fetches FreeBSD packages from.\r\nIt is used to form the correct configs on the hosts.".to_string(),
required: false,
default: "aptly.innogames.de".to_string(),
});vec.push(Attribute {
name: "monitoring_checks".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A[a-z][a-z0-9_\\-@]*\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "switch_cpu_des,switch_links_des".to_string(),
});vec.push(Attribute {
name: "monitoring_groups".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "service_groups".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "nessus_scan_id".to_string(),
typ: Number,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A[0-9]+\\Z".to_string(),
readonly: false,
hovertext: "The id of the scan in nessus".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "officetool_id".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\Ai?[0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "serial_number".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A[a-zA-Z0-9\\-]+\\Z".to_string(),
readonly: false,
hovertext: "A unique string identifying hardware, like:\r\n- Dell ServiceTag\r\n- Memory module serial number\r\n- Disk WWN address".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "ipv4".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty IPv4 address. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "ipv6".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty IPv6 address. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "hostname".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty hostname attribute. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "servertype".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty servertype attribute. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "object_id".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty object_id attribute. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
}); vec });
servertypes.insert("datacenter_type".to_string(), { let mut vec = Vec::new(); vec.push(Attribute {
name: "hostname".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty hostname attribute. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "servertype".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty servertype attribute. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "object_id".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty object_id attribute. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
}); vec });
servertypes.insert("hw_loadbalancer_group".to_string(), { let mut vec = Vec::new(); vec.push(Attribute {
name: "state".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(retired|maintenance|cold_standby|deploy_offline|deploy_online|online_reserved|online)\\Z".to_string(),
readonly: false,
hovertext: "Retired: No DNS, no Nagios, lb downtimed\r\nMaintenance: DNS, Nagios downtimed, lb downtimed\r\nDeploy_offline: DNS, Nagios active, lb downtimed\r\nDeploy_online: DNS, Nagios downtimed, lb active\r\nOnline: DNS, Nagios active, lb active".to_string(),
required: true,
default: "online".to_string(),
});vec.push(Attribute {
name: "vlan_networks".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "public_networks".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "subproject".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A[a-z][a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "project".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "network_link_type".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(kernel|ovs)_(single|bond_lacp|bond_ab)\\Z".to_string(),
readonly: false,
hovertext: "The type of physical connection between the hardware server and switches. The first part either 'ovs' if you use OpenVSwitch or 'kernel' if it's just plain Linux/Windows. Second part is either 'single' for devices with a single network cable or 'bond_ab' or 'bond_lacp' for servers with redundant network link.".to_string(),
required: true,
default: "kernel_bond_ab".to_string(),
});vec.push(Attribute {
name: "hostname".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty hostname attribute. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "servertype".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty servertype attribute. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "object_id".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty object_id attribute. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
}); vec });
servertypes.insert("router".to_string(), { let mut vec = Vec::new(); vec.push(Attribute {
name: "backup_prio".to_string(),
typ: Number,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(0|[1-9][0-9]*)\\Z".to_string(),
readonly: false,
hovertext: "Backup priority of the host. Defaults to 300. Backups will be eventually processed regardless of the value of this, but higher number means the dump and transfer of the backup will be prioritized.".to_string(),
required: false,
default: "300".to_string(),
});vec.push(Attribute {
name: "monitoring_groups".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "project_network".to_string(),
typ: Supernet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: true,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "auto_update".to_string(),
typ: Boolean,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(true|false)\\Z".to_string(),
readonly: false,
hovertext: "This is used by automated scripts to decide if this server should be automatically updated in certain situations".to_string(),
required: true,
default: "0".to_string(),
});vec.push(Attribute {
name: "backup_disabled".to_string(),
typ: Boolean,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(true|false)\\Z".to_string(),
readonly: false,
hovertext: "Set to true if you do not want this server backed up.".to_string(),
required: true,
default: "1".to_string(),
});vec.push(Attribute {
name: "backup_canshrink".to_string(),
typ: Boolean,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(true|false)\\Z".to_string(),
readonly: false,
hovertext: "Monitoring system will warn in case last backup size of the host drops below 50% of its last 10 backups' median size. Set this to true if you expect a shrinking backup (e.g. on \"speed worlds\" which are reset from time to time) so it doesn't complain on suddenly shrinking backup size.".to_string(),
required: false,
default: "0".to_string(),
});vec.push(Attribute {
name: "backup_maxage".to_string(),
typ: Number,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(0|[1-9][0-9]*)\\Z".to_string(),
readonly: false,
hovertext: "The expected freshness of the latest registered backup of the host. Defaults to 24 hours. If you have multiple backup windows, you might consider lowering this. 0 is a special value, disables backup age checking completely.".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "backup_window".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A\\d+-\\d+\\Z".to_string(),
readonly: false,
hovertext: "This is a list of hour ranges (in CEST) when the backup should be scheduled on the server locally. E.g. \"3-4\" means create one backup between 03:00 and 04:00. The backups will be transferred in order of backup_prio attribute once the local script ran through and reported a success so not guaranteed to be within the backup_window (but mostly does fit in).".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "order_date".to_string(),
typ: Date,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "When this server was ordered, for warranty and to find the order".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "order_id".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "some reference to the ticked, bill or order for this one".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "no_monitoring".to_string(),
typ: Boolean,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(true|false)\\Z".to_string(),
readonly: false,
hovertext: "if set to true this server will be ignored by the nagios generation script".to_string(),
required: false,
default: "0".to_string(),
});vec.push(Attribute {
name: "routed_networks".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "datacenter_type".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(aws|digitalocean|hetzner|hyperv|kvm)\\.dct\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "primary_ip6".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A[^\\.]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "os".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(android|cumulus|centos64|wheezy|jessie|stretch|buster|bullseye|bookworm|bionic|freebsd_[0-9]+\\.[0-9]+|macos_.*|tvos|other|esx|ciscoios|ios12.2|iDRAC6|windows(2022_std|2019_std|2019_dc|2016_std|2016_dc|2012r2_std|2012r2_dc|2012_std|2012_dc|2008r2_std|2008r2_dc|7|8|10|11))\\Z".to_string(),
readonly: false,
hovertext: "Operation system, e.g. wheezy, jessie or freebsd100 or macos_*".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "mac".to_string(),
typ: Macaddr,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "puppet_disabled".to_string(),
typ: Boolean,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(true|false)\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "1".to_string(),
});vec.push(Attribute {
name: "responsible_admin".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A[a-z][a-z\\._]+[a-z]\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: true,
default: "kajetan.staszkiewicz".to_string(),
});vec.push(Attribute {
name: "sshfp".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A[1-4] (1 [0-9a-f]{40}|2 [0-9a-f]{64})\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "dell_servicetag".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "The Service Tag for Dell to be used in the support interface and to identify the hardware".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "function".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "Function of host".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "route_network".to_string(),
typ: Supernet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: true,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "vlan_interfaces".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "memory".to_string(),
typ: Number,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(0|[1-9][0-9]*)\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "environment".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(testing|staging|production)\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: true,
default: "production".to_string(),
});vec.push(Attribute {
name: "rack".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "hardware_model".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "monitoring_checks".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A[a-z][a-z0-9_\\-@]*\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "ping".to_string(),
});vec.push(Attribute {
name: "allow_from".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: true,
hovertext: "List of Access lists from which access to this LB Pool is allowed. Connections from outside of allowed networks will be silently dropped.".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "state".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(retired|maintenance|cold_standby|deploy_offline|deploy_online|online_reserved|online)\\Z".to_string(),
readonly: false,
hovertext: "Retired: No DNS, no Nagios, lb downtimed\r\nMaintenance: DNS, Nagios downtimed, lb downtimed\r\nDeploy_offline: DNS, Nagios active, lb downtimed\r\nDeploy_online: DNS, Nagios downtimed, lb active\r\nOnline: DNS, Nagios active, lb active".to_string(),
required: true,
default: "online".to_string(),
});vec.push(Attribute {
name: "monitoring_queue".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "Set the gearman_queue for a route_network".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "datacenter".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "subproject".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A[a-z][a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "backup_storage".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "Storage server for the host's backups. This is normally auto-filled by the system while doing the first backup. Changes needs a puppet run on both the host and new storage.".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "project".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: true,
default: "ndco".to_string(),
});vec.push(Attribute {
name: "network_zones".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "A zone defining routing and firewall rules, a kind of sub-datacenter. It is a multi attribute for \"network_zone\"".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "vlan_networks".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "puppet_classes".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A[a-z][a-z0-9_]*(::[a-z][a-z0-9_]*)*\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "puppet_ca".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "specifies the puppet ca server which the server should use".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "num_cpu".to_string(),
typ: Number,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(0|[1-9][0-9]*)\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "disk_size_gib".to_string(),
typ: Number,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(0|[1-9][0-9]*)\\Z".to_string(),
readonly: false,
hovertext: "This is the current disk size of the server in gibibytes.  It should be filled while the VM is created or maintained by other scripts.\r\n".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "repositories".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A\\w+:\\w+:\\w+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "graphite_graphs".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A[a-z][a-z0-9_\\.\\-]*\\Z".to_string(),
readonly: false,
hovertext: "a list of graphs to reference which graphs should be displayed on this server to be used in the collections (doku link).".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "puppet_master".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "puppet_environment".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(?!(testing|staging|production))[a-z][a-z0-9_]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "network_zone".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "A zone defining routing and firewall rules, a kind of sub-datacenter.".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "repositories_host".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A([a-z0-9]+(-[a-z0-9]+)*\\.)+[a-z]{2,}\\Z".to_string(),
readonly: false,
hovertext: "Repositories hostname to fetch packages from. On Debian this is where apt fetches debian packages from (aptly). For FreeBSD this is where pkg fetches FreeBSD packages from.\r\nIt is used to form the correct configs on the hosts.".to_string(),
required: false,
default: "aptly.innogames.de".to_string(),
});vec.push(Attribute {
name: "loadbalancer".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "A list of LB Pools to which this Node belongs.".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "service_groups".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "ipv4".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty IPv4 address. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "ipv6".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty IPv6 address. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "hostname".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty hostname attribute. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "servertype".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty servertype attribute. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "object_id".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty object_id attribute. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
}); vec });
servertypes.insert("everything".to_string(), { let mut vec = Vec::new(); vec.push(Attribute {
name: "acl".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: true,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "responsible_admin".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A[a-z][a-z\\._]+[a-z]\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "state".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(retired|maintenance|cold_standby|deploy_offline|deploy_online|online_reserved|online)\\Z".to_string(),
readonly: false,
hovertext: "Retired: No DNS, no Nagios, lb downtimed\r\nMaintenance: DNS, Nagios downtimed, lb downtimed\r\nDeploy_offline: DNS, Nagios active, lb downtimed\r\nDeploy_online: DNS, Nagios downtimed, lb active\r\nOnline: DNS, Nagios active, lb active".to_string(),
required: true,
default: "online".to_string(),
});vec.push(Attribute {
name: "service_groups".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "project".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: true,
default: "acl".to_string(),
});vec.push(Attribute {
name: "ipv4".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty IPv4 address. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "ipv6".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty IPv6 address. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "hostname".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty hostname attribute. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "servertype".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty servertype attribute. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "object_id".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty object_id attribute. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
}); vec });
servertypes.insert("doorlock".to_string(), { let mut vec = Vec::new(); vec.push(Attribute {
name: "datacenter".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "route_network".to_string(),
typ: Supernet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: true,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "mac".to_string(),
typ: Macaddr,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "project".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: true,
default: "officeit".to_string(),
});vec.push(Attribute {
name: "network_zones".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "A zone defining routing and firewall rules, a kind of sub-datacenter. It is a multi attribute for \"network_zone\"".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "state".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(retired|maintenance|cold_standby|deploy_offline|deploy_online|online_reserved|online)\\Z".to_string(),
readonly: false,
hovertext: "Retired: No DNS, no Nagios, lb downtimed\r\nMaintenance: DNS, Nagios downtimed, lb downtimed\r\nDeploy_offline: DNS, Nagios active, lb downtimed\r\nDeploy_online: DNS, Nagios downtimed, lb active\r\nOnline: DNS, Nagios active, lb active".to_string(),
required: true,
default: "online".to_string(),
});vec.push(Attribute {
name: "ipv4".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty IPv4 address. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "ipv6".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty IPv6 address. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "hostname".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty hostname attribute. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "servertype".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty servertype attribute. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "object_id".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty object_id attribute. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
}); vec });
servertypes.insert("provider_network".to_string(), { let mut vec = Vec::new(); vec.push(Attribute {
name: "datacenter".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "network_type".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(loopback|transit_(internal_igp|public_(hwlbs|uplink|igp))|public|internal|public_dmz|vpn_clients|internal_vpn_clients|cloud)\\Z".to_string(),
readonly: false,
hovertext: "Select what kind of connection you want to switch\r\nsingle: single interface without lacp or anything other special\r\nbond_ab: Bonding without LACP but 2 links with failover\r\nbond_lacp: bonding with lacp and 2 active links".to_string(),
required: false,
default: "public".to_string(),
});vec.push(Attribute {
name: "network_zone".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "A zone defining routing and firewall rules, a kind of sub-datacenter.".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "network_zones".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "A zone defining routing and firewall rules, a kind of sub-datacenter. It is a multi attribute for \"network_zone\"".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "monitoring_queue".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "Set the gearman_queue for a route_network".to_string(),
required: false,
default: "default.queue".to_string(),
});vec.push(Attribute {
name: "primary_ip6".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A[^\\.]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "state".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(retired|maintenance|cold_standby|deploy_offline|deploy_online|online_reserved|online)\\Z".to_string(),
readonly: false,
hovertext: "Retired: No DNS, no Nagios, lb downtimed\r\nMaintenance: DNS, Nagios downtimed, lb downtimed\r\nDeploy_offline: DNS, Nagios active, lb downtimed\r\nDeploy_online: DNS, Nagios downtimed, lb active\r\nOnline: DNS, Nagios active, lb active".to_string(),
required: true,
default: "online".to_string(),
});vec.push(Attribute {
name: "acl".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: true,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "vlan_tag".to_string(),
typ: Number,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(0|[1-9][0-9]*)\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "default_gateway".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "service_groups".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "project".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: true,
default: "ndco".to_string(),
});vec.push(Attribute {
name: "ipv4".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty IPv4 address. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "ipv6".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty IPv6 address. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "hostname".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty hostname attribute. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "servertype".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty servertype attribute. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "object_id".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty object_id attribute. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
}); vec });
servertypes.insert("monitoring_group".to_string(), { let mut vec = Vec::new(); vec.push(Attribute {
name: "state".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(retired|maintenance|cold_standby|deploy_offline|deploy_online|online_reserved|online)\\Z".to_string(),
readonly: false,
hovertext: "Retired: No DNS, no Nagios, lb downtimed\r\nMaintenance: DNS, Nagios downtimed, lb downtimed\r\nDeploy_offline: DNS, Nagios active, lb downtimed\r\nDeploy_online: DNS, Nagios downtimed, lb active\r\nOnline: DNS, Nagios active, lb active".to_string(),
required: true,
default: "online".to_string(),
});vec.push(Attribute {
name: "monitoring_group_members".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "responsible_admin".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A[a-z][a-z\\._]+[a-z]\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "hostname".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty hostname attribute. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "servertype".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty servertype attribute. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "object_id".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty object_id attribute. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
}); vec });
servertypes.insert("vlan_interface".to_string(), { let mut vec = Vec::new(); vec.push(Attribute {
name: "network_type".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(loopback|transit_(internal_igp|public_(hwlbs|uplink|igp))|public|internal|public_dmz|vpn_clients|internal_vpn_clients|cloud)\\Z".to_string(),
readonly: false,
hovertext: "Select what kind of connection you want to switch\r\nsingle: single interface without lacp or anything other special\r\nbond_ab: Bonding without LACP but 2 links with failover\r\nbond_lacp: bonding with lacp and 2 active links".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "parent_bond".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "primary_ip6".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A[^\\.]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "no_monitoring".to_string(),
typ: Boolean,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(true|false)\\Z".to_string(),
readonly: false,
hovertext: "if set to true this server will be ignored by the nagios generation script".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "project_network".to_string(),
typ: Supernet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: true,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "function".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "Function of host".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "vlan_interface_of".to_string(),
typ: Relation,
reversed_attribute_id: "vlan_interfaces".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: true,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "acl".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: true,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "service_groups".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "vlan_tag".to_string(),
typ: Number,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(0|[1-9][0-9]*)\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "route_network".to_string(),
typ: Supernet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: true,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "monitoring_checks".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A[a-z][a-z0-9_\\-@]*\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "ping".to_string(),
});vec.push(Attribute {
name: "sshfp".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A[1-4] (1 [0-9a-f]{40}|2 [0-9a-f]{64})\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "state".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(retired|maintenance|cold_standby|deploy_offline|deploy_online|online_reserved|online)\\Z".to_string(),
readonly: false,
hovertext: "Retired: No DNS, no Nagios, lb downtimed\r\nMaintenance: DNS, Nagios downtimed, lb downtimed\r\nDeploy_offline: DNS, Nagios active, lb downtimed\r\nDeploy_online: DNS, Nagios downtimed, lb active\r\nOnline: DNS, Nagios active, lb active".to_string(),
required: true,
default: "online".to_string(),
});vec.push(Attribute {
name: "environment".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(testing|staging|production)\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: true,
default: "production".to_string(),
});vec.push(Attribute {
name: "monitoring_queue".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "Set the gearman_queue for a route_network".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "bond_no".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "project".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: true,
default: "ndco".to_string(),
});vec.push(Attribute {
name: "network_zones".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "A zone defining routing and firewall rules, a kind of sub-datacenter. It is a multi attribute for \"network_zone\"".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "datacenter".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "datacenter_type".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(aws|digitalocean|hetzner|hyperv|kvm)\\.dct\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "network_zone".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "A zone defining routing and firewall rules, a kind of sub-datacenter.".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "ipv4".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty IPv4 address. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "ipv6".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty IPv6 address. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "hostname".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty hostname attribute. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "servertype".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty servertype attribute. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "object_id".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty object_id attribute. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
}); vec });
servertypes.insert("external_domain".to_string(), { let mut vec = Vec::new(); vec.push(Attribute {
name: "acl".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: true,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "monitoring_checks".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A[a-z][a-z0-9_\\-@]*\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "responsible_admin".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A[a-z][a-z\\._]+[a-z]\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "subproject".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A[a-z][a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "service_groups".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "no_monitoring".to_string(),
typ: Boolean,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(true|false)\\Z".to_string(),
readonly: false,
hovertext: "if set to true this server will be ignored by the nagios generation script".to_string(),
required: true,
default: "false".to_string(),
});vec.push(Attribute {
name: "project".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "state".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(retired|maintenance|cold_standby|deploy_offline|deploy_online|online_reserved|online)\\Z".to_string(),
readonly: false,
hovertext: "Retired: No DNS, no Nagios, lb downtimed\r\nMaintenance: DNS, Nagios downtimed, lb downtimed\r\nDeploy_offline: DNS, Nagios active, lb downtimed\r\nDeploy_online: DNS, Nagios downtimed, lb active\r\nOnline: DNS, Nagios active, lb active".to_string(),
required: true,
default: "online".to_string(),
});vec.push(Attribute {
name: "hostname".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty hostname attribute. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "servertype".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty servertype attribute. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "object_id".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty object_id attribute. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
}); vec });
servertypes.insert("provider".to_string(), { let mut vec = Vec::new(); vec.push(Attribute {
name: "state".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(retired|maintenance|cold_standby|deploy_offline|deploy_online|online_reserved|online)\\Z".to_string(),
readonly: false,
hovertext: "Retired: No DNS, no Nagios, lb downtimed\r\nMaintenance: DNS, Nagios downtimed, lb downtimed\r\nDeploy_offline: DNS, Nagios active, lb downtimed\r\nDeploy_online: DNS, Nagios downtimed, lb active\r\nOnline: DNS, Nagios active, lb active".to_string(),
required: true,
default: "online".to_string(),
});vec.push(Attribute {
name: "provider_asn".to_string(),
typ: Number,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A[0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "hostname".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty hostname attribute. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "servertype".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty servertype attribute. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "object_id".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty object_id attribute. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
}); vec });
servertypes.insert("health_check".to_string(), { let mut vec = Vec::new(); vec.push(Attribute {
name: "hc_type".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(dns|http|https|ping|postgres|tcp)\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: true,
default: "ping".to_string(),
});vec.push(Attribute {
name: "hc_interval".to_string(),
typ: Number,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(0|[1-9][0-9]*)\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: true,
default: "2".to_string(),
});vec.push(Attribute {
name: "hc_max_failed".to_string(),
typ: Number,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(0|[1-9][0-9]*)\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: true,
default: "3".to_string(),
});vec.push(Attribute {
name: "hc_host".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "hc_timeout".to_string(),
typ: Number,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(0|[1-9][0-9]*)\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: true,
default: "1500".to_string(),
});vec.push(Attribute {
name: "hc_ok_codes".to_string(),
typ: Number,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A(0|[1-9][0-9]*)\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "subproject".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A[a-z][a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "hc_port".to_string(),
typ: Number,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(0|[1-9][0-9]*)\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "hc_dbname".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "hc_user".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "hc_query".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "project".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "hc_drain_codes".to_string(),
typ: Number,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A(0|[1-9][0-9]*)\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "hostname".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty hostname attribute. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "servertype".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty servertype attribute. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "object_id".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty object_id attribute. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
}); vec });
servertypes.insert("storage_module".to_string(), { let mut vec = Vec::new(); vec.push(Attribute {
name: "state".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(retired|maintenance|cold_standby|deploy_offline|deploy_online|online_reserved|online)\\Z".to_string(),
readonly: false,
hovertext: "Retired: No DNS, no Nagios, lb downtimed\r\nMaintenance: DNS, Nagios downtimed, lb downtimed\r\nDeploy_offline: DNS, Nagios active, lb downtimed\r\nDeploy_online: DNS, Nagios downtimed, lb active\r\nOnline: DNS, Nagios active, lb active".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "capacity".to_string(),
typ: Number,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A[0-9]+\\Z".to_string(),
readonly: false,
hovertext: "Capacity of a memory or a storage module. Measured in B.".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "serial_number".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A[a-zA-Z0-9\\-]+\\Z".to_string(),
readonly: false,
hovertext: "A unique string identifying hardware, like:\r\n- Dell ServiceTag\r\n- Memory module serial number\r\n- Disk WWN address".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "installed_in".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A[a-z0-9\\-\\.]+\\Z".to_string(),
readonly: false,
hovertext: "A physical device in which the hardware component is installed.".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "harddisk_type".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(ssd|hdd|unknown)\\Z".to_string(),
readonly: false,
hovertext: "this is either hdd or ssd to give an indication on the performance and io scheduler to be used".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "hardware_model".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "hostname".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty hostname attribute. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "servertype".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty servertype attribute. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "object_id".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty object_id attribute. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
}); vec });
servertypes.insert("blade_enclosure".to_string(), { let mut vec = Vec::new(); vec.push(Attribute {
name: "backup_canshrink".to_string(),
typ: Boolean,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(true|false)\\Z".to_string(),
readonly: false,
hovertext: "Monitoring system will warn in case last backup size of the host drops below 50% of its last 10 backups' median size. Set this to true if you expect a shrinking backup (e.g. on \"speed worlds\" which are reset from time to time) so it doesn't complain on suddenly shrinking backup size.".to_string(),
required: false,
default: "0".to_string(),
});vec.push(Attribute {
name: "backup_maxage".to_string(),
typ: Number,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(0|[1-9][0-9]*)\\Z".to_string(),
readonly: false,
hovertext: "The expected freshness of the latest registered backup of the host. Defaults to 24 hours. If you have multiple backup windows, you might consider lowering this. 0 is a special value, disables backup age checking completely.".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "backup_window".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A\\d+-\\d+\\Z".to_string(),
readonly: false,
hovertext: "This is a list of hour ranges (in CEST) when the backup should be scheduled on the server locally. E.g. \"3-4\" means create one backup between 03:00 and 04:00. The backups will be transferred in order of backup_prio attribute once the local script ran through and reported a success so not guaranteed to be within the backup_window (but mostly does fit in).".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "backup_prio".to_string(),
typ: Number,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(0|[1-9][0-9]*)\\Z".to_string(),
readonly: false,
hovertext: "Backup priority of the host. Defaults to 300. Backups will be eventually processed regardless of the value of this, but higher number means the dump and transfer of the backup will be prioritized.".to_string(),
required: false,
default: "300".to_string(),
});vec.push(Attribute {
name: "rack_height".to_string(),
typ: Number,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A[0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "rack_position".to_string(),
typ: Number,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A[0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "order_id".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "some reference to the ticked, bill or order for this one".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "auto_update".to_string(),
typ: Boolean,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(true|false)\\Z".to_string(),
readonly: false,
hovertext: "This is used by automated scripts to decide if this server should be automatically updated in certain situations".to_string(),
required: true,
default: "0".to_string(),
});vec.push(Attribute {
name: "backup_disabled".to_string(),
typ: Boolean,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(true|false)\\Z".to_string(),
readonly: false,
hovertext: "Set to true if you do not want this server backed up.".to_string(),
required: true,
default: "1".to_string(),
});vec.push(Attribute {
name: "order_date".to_string(),
typ: Date,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "When this server was ordered, for warranty and to find the order".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "no_monitoring".to_string(),
typ: Boolean,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(true|false)\\Z".to_string(),
readonly: false,
hovertext: "if set to true this server will be ignored by the nagios generation script".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "primary_ip6".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A[^\\.]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "dell_servicetag".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "The Service Tag for Dell to be used in the support interface and to identify the hardware".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "route_network".to_string(),
typ: Supernet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: true,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "monitoring_groups".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "monitoring_checks".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A[a-z][a-z0-9_\\-@]*\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "ping".to_string(),
});vec.push(Attribute {
name: "function".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "Function of host".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "hardware_model".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "sshfp".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A[1-4] (1 [0-9a-f]{40}|2 [0-9a-f]{64})\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "responsible_admin".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A[a-z][a-z\\._]+[a-z]\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: true,
default: "kajetan.staszkiewicz".to_string(),
});vec.push(Attribute {
name: "state".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(retired|maintenance|cold_standby|deploy_offline|deploy_online|online_reserved|online)\\Z".to_string(),
readonly: false,
hovertext: "Retired: No DNS, no Nagios, lb downtimed\r\nMaintenance: DNS, Nagios downtimed, lb downtimed\r\nDeploy_offline: DNS, Nagios active, lb downtimed\r\nDeploy_online: DNS, Nagios downtimed, lb active\r\nOnline: DNS, Nagios active, lb active".to_string(),
required: true,
default: "online".to_string(),
});vec.push(Attribute {
name: "datacenter".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "rack".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "project_network".to_string(),
typ: Supernet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: true,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "environment".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(testing|staging|production)\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: true,
default: "production".to_string(),
});vec.push(Attribute {
name: "monitoring_queue".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "Set the gearman_queue for a route_network".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "backup_storage".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "Storage server for the host's backups. This is normally auto-filled by the system while doing the first backup. Changes needs a puppet run on both the host and new storage.".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "project".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: true,
default: "ndco".to_string(),
});vec.push(Attribute {
name: "network_zones".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "A zone defining routing and firewall rules, a kind of sub-datacenter. It is a multi attribute for \"network_zone\"".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "firmware_versions".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A(BMC|BIOS|MIDPLANE)_[0-9a-z\\.]+\\Z".to_string(),
readonly: false,
hovertext: "Versions of various firmware components. See regexp for notation and supported attributes.".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "mac".to_string(),
typ: Macaddr,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "os".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(android|cumulus|centos64|wheezy|jessie|stretch|buster|bullseye|bookworm|bionic|freebsd_[0-9]+\\.[0-9]+|macos_.*|tvos|other|esx|ciscoios|ios12.2|iDRAC6|windows(2022_std|2019_std|2019_dc|2016_std|2016_dc|2012r2_std|2012r2_dc|2012_std|2012_dc|2008r2_std|2008r2_dc|7|8|10|11))\\Z".to_string(),
readonly: false,
hovertext: "Operation system, e.g. wheezy, jessie or freebsd100 or macos_*".to_string(),
required: true,
default: "iDRAC6".to_string(),
});vec.push(Attribute {
name: "powerports".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "ipv4".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty IPv4 address. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "ipv6".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty IPv6 address. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "hostname".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty hostname attribute. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "servertype".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty servertype attribute. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "object_id".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty object_id attribute. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
}); vec });
servertypes.insert("tunnel_interface".to_string(), { let mut vec = Vec::new(); vec.push(Attribute {
name: "state".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(retired|maintenance|cold_standby|deploy_offline|deploy_online|online_reserved|online)\\Z".to_string(),
readonly: false,
hovertext: "Retired: No DNS, no Nagios, lb downtimed\r\nMaintenance: DNS, Nagios downtimed, lb downtimed\r\nDeploy_offline: DNS, Nagios active, lb downtimed\r\nDeploy_online: DNS, Nagios downtimed, lb active\r\nOnline: DNS, Nagios active, lb active".to_string(),
required: true,
default: "online".to_string(),
});vec.push(Attribute {
name: "primary_ip6".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A[^\\.]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "project".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: true,
default: "ndco".to_string(),
});vec.push(Attribute {
name: "monitoring_queue".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "Set the gearman_queue for a route_network".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "network_type".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(loopback|transit_(internal_igp|public_(hwlbs|uplink|igp))|public|internal|public_dmz|vpn_clients|internal_vpn_clients|cloud)\\Z".to_string(),
readonly: false,
hovertext: "Select what kind of connection you want to switch\r\nsingle: single interface without lacp or anything other special\r\nbond_ab: Bonding without LACP but 2 links with failover\r\nbond_lacp: bonding with lacp and 2 active links".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "network_zone".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "A zone defining routing and firewall rules, a kind of sub-datacenter.".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "network_zones".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "A zone defining routing and firewall rules, a kind of sub-datacenter. It is a multi attribute for \"network_zone\"".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "service_groups".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "tunnel_interface_of".to_string(),
typ: Relation,
reversed_attribute_id: "tunnel_interfaces".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: true,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "route_network".to_string(),
typ: Supernet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: true,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "ipv4".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty IPv4 address. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "ipv6".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty IPv6 address. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "hostname".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty hostname attribute. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "servertype".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty servertype attribute. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "object_id".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty object_id attribute. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
}); vec });
servertypes.insert("route_network".to_string(), { let mut vec = Vec::new(); vec.push(Attribute {
name: "parent_bond".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "datacenter".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "public_networks".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "network_zone".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "A zone defining routing and firewall rules, a kind of sub-datacenter.".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "provider".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "table_priority".to_string(),
typ: Number,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(0|[1-9][0-9]*)\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "internal_gateway_vhid".to_string(),
typ: Number,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(0|[1-9][0-9]*)\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "network_zones".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "A zone defining routing and firewall rules, a kind of sub-datacenter. It is a multi attribute for \"network_zone\"".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "default_gateway".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "vlan_tag".to_string(),
typ: Number,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(0|[1-9][0-9]*)\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "primary_ip6".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A[^\\.]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "state".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(retired|maintenance|cold_standby|deploy_offline|deploy_online|online_reserved|online)\\Z".to_string(),
readonly: false,
hovertext: "Retired: No DNS, no Nagios, lb downtimed\r\nMaintenance: DNS, Nagios downtimed, lb downtimed\r\nDeploy_offline: DNS, Nagios active, lb downtimed\r\nDeploy_online: DNS, Nagios downtimed, lb active\r\nOnline: DNS, Nagios active, lb active".to_string(),
required: true,
default: "online".to_string(),
});vec.push(Attribute {
name: "acl".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: true,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "routing_table".to_string(),
typ: Number,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(0|[1-9][0-9]*)\\Z".to_string(),
readonly: false,
hovertext: "Number of routing table used for routing to and from this subnet".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "network_type".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(loopback|transit_(internal_igp|public_(hwlbs|uplink|igp))|public|internal|public_dmz|vpn_clients|internal_vpn_clients|cloud)\\Z".to_string(),
readonly: false,
hovertext: "Select what kind of connection you want to switch\r\nsingle: single interface without lacp or anything other special\r\nbond_ab: Bonding without LACP but 2 links with failover\r\nbond_lacp: bonding with lacp and 2 active links".to_string(),
required: true,
default: "internal".to_string(),
});vec.push(Attribute {
name: "provider_network".to_string(),
typ: Supernet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: true,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "assigned_to".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "internal_gateway".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "bond_no".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "service_groups".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "project".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: true,
default: "ndco".to_string(),
});vec.push(Attribute {
name: "datacenter_type".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(aws|digitalocean|hetzner|hyperv|kvm)\\.dct\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "aws_vpc_id".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\Avpc-[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "Aws VpcId".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "monitoring_queue".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "Set the gearman_queue for a route_network".to_string(),
required: true,
default: "default.queue".to_string(),
});vec.push(Attribute {
name: "ospf_active".to_string(),
typ: Boolean,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(true|false)\\Z".to_string(),
readonly: false,
hovertext: "Make routers having vlan_interfaces in this network active in OSPF.".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "snat_lb_pool".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "LB Pool used for providing default SNAT for IPv4.".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "network_autoconfiguration".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(none|dhcp_static|dhcp_dynamic)\\Z".to_string(),
readonly: false,
hovertext: "Configure automatic assignment of IP addresses of devices in this network:\r\n- none - no RAs, no DHCP\r\n- dhcp_static - no SLAAC due to RAs send without the \"auto\" flag, DHCPv4 and DHCPv6 only with entries from Serveradmin\r\n- dhcp_dynamic - SLAAC due to RAs send with the \"auto\" flag, DHCPv4 and DHCPv6 with entries from Serveradmin and from dynamic pool".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "ipv4".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty IPv4 address. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "ipv6".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty IPv6 address. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "hostname".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty hostname attribute. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "servertype".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty servertype attribute. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "object_id".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty object_id attribute. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
}); vec });
servertypes.insert("datacenter".to_string(), { let mut vec = Vec::new(); vec.push(Attribute {
name: "no_monitoring".to_string(),
typ: Boolean,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(true|false)\\Z".to_string(),
readonly: false,
hovertext: "if set to true this server will be ignored by the nagios generation script".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "datacenter_colo".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "responsible_admin".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A[a-z][a-z\\._]+[a-z]\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "state".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(retired|maintenance|cold_standby|deploy_offline|deploy_online|online_reserved|online)\\Z".to_string(),
readonly: false,
hovertext: "Retired: No DNS, no Nagios, lb downtimed\r\nMaintenance: DNS, Nagios downtimed, lb downtimed\r\nDeploy_offline: DNS, Nagios active, lb downtimed\r\nDeploy_online: DNS, Nagios downtimed, lb active\r\nOnline: DNS, Nagios active, lb active".to_string(),
required: true,
default: "online".to_string(),
});vec.push(Attribute {
name: "project".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: true,
default: "ndco".to_string(),
});vec.push(Attribute {
name: "hostname".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty hostname attribute. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "servertype".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty servertype attribute. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "object_id".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty object_id attribute. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
}); vec });
servertypes.insert("monitoring_queue".to_string(), { let mut vec = Vec::new(); vec.push(Attribute {
name: "project".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: true,
default: "admin".to_string(),
});vec.push(Attribute {
name: "subproject".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A[a-z][a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: true,
default: "monitoring".to_string(),
});vec.push(Attribute {
name: "monitoring_workers".to_string(),
typ: Relation,
reversed_attribute_id: "monitoring_queues".to_string(),
multi: true,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: true,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "monitoring_networks".to_string(),
typ: Relation,
reversed_attribute_id: "monitoring_queue".to_string(),
multi: true,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: true,
hovertext: "shows the route_networks for a gearman_queue".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "hostname".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty hostname attribute. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "servertype".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty servertype attribute. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "object_id".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty object_id attribute. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
}); vec });
servertypes.insert("nat_gateway".to_string(), { let mut vec = Vec::new(); vec.push(Attribute {
name: "network_type".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(loopback|transit_(internal_igp|public_(hwlbs|uplink|igp))|public|internal|public_dmz|vpn_clients|internal_vpn_clients|cloud)\\Z".to_string(),
readonly: false,
hovertext: "Select what kind of connection you want to switch\r\nsingle: single interface without lacp or anything other special\r\nbond_ab: Bonding without LACP but 2 links with failover\r\nbond_lacp: bonding with lacp and 2 active links".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "datacenter_type".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(aws|digitalocean|hetzner|hyperv|kvm)\\.dct\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "primary_ip6".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A[^\\.]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "default_snat".to_string(),
typ: Boolean,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(true|false)\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "false".to_string(),
});vec.push(Attribute {
name: "monitoring_checks".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A[a-z][a-z0-9_\\-@]*\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "ping".to_string(),
});vec.push(Attribute {
name: "function".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "Function of host".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "project_network".to_string(),
typ: Supernet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: true,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "vrrp_id".to_string(),
typ: Number,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(0|[1-9][0-9]*)\\Z".to_string(),
readonly: false,
hovertext: "This is VHID for CARP or VRID for VRRP.".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "environment".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(testing|staging|production)\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: true,
default: "production".to_string(),
});vec.push(Attribute {
name: "route_network".to_string(),
typ: Supernet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: true,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "responsible_admin".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A[a-z][a-z\\._]+[a-z]\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "acl".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: true,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "no_monitoring".to_string(),
typ: Boolean,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(true|false)\\Z".to_string(),
readonly: false,
hovertext: "if set to true this server will be ignored by the nagios generation script".to_string(),
required: false,
default: "false".to_string(),
});vec.push(Attribute {
name: "monitoring_queue".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "Set the gearman_queue for a route_network".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "gateway_type".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "state".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(retired|maintenance|cold_standby|deploy_offline|deploy_online|online_reserved|online)\\Z".to_string(),
readonly: false,
hovertext: "Retired: No DNS, no Nagios, lb downtimed\r\nMaintenance: DNS, Nagios downtimed, lb downtimed\r\nDeploy_offline: DNS, Nagios active, lb downtimed\r\nDeploy_online: DNS, Nagios downtimed, lb active\r\nOnline: DNS, Nagios active, lb active".to_string(),
required: true,
default: "online".to_string(),
});vec.push(Attribute {
name: "datacenter".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "service_groups".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "project".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: true,
default: "ndco".to_string(),
});vec.push(Attribute {
name: "network_zones".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "A zone defining routing and firewall rules, a kind of sub-datacenter. It is a multi attribute for \"network_zone\"".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "network_zone".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "A zone defining routing and firewall rules, a kind of sub-datacenter.".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "monitoring_groups".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "ipv4".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty IPv4 address. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "ipv6".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty IPv6 address. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "hostname".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty hostname attribute. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "servertype".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty servertype attribute. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "object_id".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty object_id attribute. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
}); vec });
servertypes.insert("media_device".to_string(), { let mut vec = Vec::new(); vec.push(Attribute {
name: "primary_ip6".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A[^\\.]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "state".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(retired|maintenance|cold_standby|deploy_offline|deploy_online|online_reserved|online)\\Z".to_string(),
readonly: false,
hovertext: "Retired: No DNS, no Nagios, lb downtimed\r\nMaintenance: DNS, Nagios downtimed, lb downtimed\r\nDeploy_offline: DNS, Nagios active, lb downtimed\r\nDeploy_online: DNS, Nagios downtimed, lb active\r\nOnline: DNS, Nagios active, lb active".to_string(),
required: true,
default: "online".to_string(),
});vec.push(Attribute {
name: "hardware_model".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "hardware_type".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "mandatory field on all objects such as amp, beamer, recorder etc.".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "responsible_admin".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A[a-z][a-z\\._]+[a-z]\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "responsible_external".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "extern company who monitory/maintains device or system".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "access_uri".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A(ssh://|ipv4://|ipv6:|https://|http://).*\\Z".to_string(),
readonly: false,
hovertext: "URI to access the device".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "serial_number".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A[a-zA-Z0-9\\-]+\\Z".to_string(),
readonly: false,
hovertext: "A unique string identifying hardware, like:\r\n- Dell ServiceTag\r\n- Memory module serial number\r\n- Disk WWN address".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "responsible_internal".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "business owner".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "project".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: true,
default: "officeit".to_string(),
});vec.push(Attribute {
name: "allow_from".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: true,
hovertext: "List of Access lists from which access to this LB Pool is allowed. Connections from outside of allowed networks will be silently dropped.".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "route_network".to_string(),
typ: Supernet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: true,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "network_zones".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "A zone defining routing and firewall rules, a kind of sub-datacenter. It is a multi attribute for \"network_zone\"".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "powerports".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: true,
default: "direct".to_string(),
});vec.push(Attribute {
name: "officetool_id".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\Ai?[0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "ipv4".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty IPv4 address. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "ipv6".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty IPv6 address. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "hostname".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty hostname attribute. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "servertype".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty servertype attribute. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "object_id".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty object_id attribute. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
}); vec });
servertypes.insert("project".to_string(), { let mut vec = Vec::new(); vec.push(Attribute {
name: "responsible_admin".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A[a-z][a-z\\._]+[a-z]\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "event_tracking_code".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A([a-z0-9]+)\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "is_game".to_string(),
typ: Boolean,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(true|false)\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: true,
default: "false".to_string(),
});vec.push(Attribute {
name: "jira_project".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A([A-Z]+)\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "project_english_name".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A[a-zA-Z0-9 ]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "short_code".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A([a-z]+)\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "slack_channel".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(#[a-z0-9_-]+)\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "support_code".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A([a-z0-9]+)\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "sentry_team".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "Sentry team name at sentry.io".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "hostname".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty hostname attribute. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "servertype".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty servertype attribute. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "object_id".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty object_id attribute. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
}); vec });
servertypes.insert("natbox".to_string(), { let mut vec = Vec::new(); vec.push(Attribute {
name: "service_groups".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "vlan_networks".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "puppet_ca".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "specifies the puppet ca server which the server should use".to_string(),
required: true,
default: "puppetca.innogames.de".to_string(),
});vec.push(Attribute {
name: "puppet_master".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "arch".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: true,
default: "amd64".to_string(),
});vec.push(Attribute {
name: "order_date".to_string(),
typ: Date,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "When this server was ordered, for warranty and to find the order".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "order_id".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "some reference to the ticked, bill or order for this one".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "puppet_disabled".to_string(),
typ: Boolean,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(true|false)\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "False".to_string(),
});vec.push(Attribute {
name: "primary_ip6".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A[^\\.]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "no_monitoring".to_string(),
typ: Boolean,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(true|false)\\Z".to_string(),
readonly: false,
hovertext: "if set to true this server will be ignored by the nagios generation script".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "route_network".to_string(),
typ: Supernet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: true,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "graphite_graphs".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A[a-z][a-z0-9_\\.\\-]*\\Z".to_string(),
readonly: false,
hovertext: "a list of graphs to reference which graphs should be displayed on this server to be used in the collections (doku link).".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "backup_window".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A\\d+-\\d+\\Z".to_string(),
readonly: false,
hovertext: "This is a list of hour ranges (in CEST) when the backup should be scheduled on the server locally. E.g. \"3-4\" means create one backup between 03:00 and 04:00. The backups will be transferred in order of backup_prio attribute once the local script ran through and reported a success so not guaranteed to be within the backup_window (but mostly does fit in).".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "backup_prio".to_string(),
typ: Number,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(0|[1-9][0-9]*)\\Z".to_string(),
readonly: false,
hovertext: "Backup priority of the host. Defaults to 300. Backups will be eventually processed regardless of the value of this, but higher number means the dump and transfer of the backup will be prioritized.".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "powerports".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "switchports".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "Switchports attribute is used by Foreman to configure switches. That includes creating proper LAGGs and assigning VLANs to them. ".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "mac".to_string(),
typ: Macaddr,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "routed_networks".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "num_cpu".to_string(),
typ: Number,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(0|[1-9][0-9]*)\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "dell_servicetag".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "The Service Tag for Dell to be used in the support interface and to identify the hardware".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "repositories".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A\\w+:\\w+:\\w+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "ipmi".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "function".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "Function of host".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "cpu_model".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "cpu_clock_mhz".to_string(),
typ: Number,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(0|[1-9][0-9]*)\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "puppet_environment".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(?!(testing|staging|production))[a-z][a-z0-9_]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "memory".to_string(),
typ: Number,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(0|[1-9][0-9]*)\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "network_zones".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "A zone defining routing and firewall rules, a kind of sub-datacenter. It is a multi attribute for \"network_zone\"".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "hardware_model".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "environment".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(testing|staging|production)\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: true,
default: "production".to_string(),
});vec.push(Attribute {
name: "memory_modules".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "disk_size_gib".to_string(),
typ: Number,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(0|[1-9][0-9]*)\\Z".to_string(),
readonly: false,
hovertext: "This is the current disk size of the server in gibibytes.  It should be filled while the VM is created or maintained by other scripts.\r\n".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "rack".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "responsible_admin".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A[a-z][a-z\\._]+[a-z]\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: true,
default: "kajetan.staszkiewicz".to_string(),
});vec.push(Attribute {
name: "harddisk_type".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(ssd|hdd|unknown)\\Z".to_string(),
readonly: false,
hovertext: "this is either hdd or ssd to give an indication on the performance and io scheduler to be used".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "monitoring_checks".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A[a-z][a-z0-9_\\-@]*\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "ping".to_string(),
});vec.push(Attribute {
name: "public_networks".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "sshfp".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A[1-4] (1 [0-9a-f]{40}|2 [0-9a-f]{64})\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "vlan_interfaces".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "puppet_classes".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A[a-z][a-z0-9_]*(::[a-z][a-z0-9_]*)*\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "state".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(retired|maintenance|cold_standby|deploy_offline|deploy_online|online_reserved|online)\\Z".to_string(),
readonly: false,
hovertext: "Retired: No DNS, no Nagios, lb downtimed\r\nMaintenance: DNS, Nagios downtimed, lb downtimed\r\nDeploy_offline: DNS, Nagios active, lb downtimed\r\nDeploy_online: DNS, Nagios downtimed, lb active\r\nOnline: DNS, Nagios active, lb active".to_string(),
required: true,
default: "online".to_string(),
});vec.push(Attribute {
name: "backup_disabled".to_string(),
typ: Boolean,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(true|false)\\Z".to_string(),
readonly: false,
hovertext: "Set to true if you do not want this server backed up.".to_string(),
required: true,
default: "1".to_string(),
});vec.push(Attribute {
name: "backup_canshrink".to_string(),
typ: Boolean,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(true|false)\\Z".to_string(),
readonly: false,
hovertext: "Monitoring system will warn in case last backup size of the host drops below 50% of its last 10 backups' median size. Set this to true if you expect a shrinking backup (e.g. on \"speed worlds\" which are reset from time to time) so it doesn't complain on suddenly shrinking backup size.".to_string(),
required: false,
default: "0".to_string(),
});vec.push(Attribute {
name: "network_zone".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "A zone defining routing and firewall rules, a kind of sub-datacenter.".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "datacenter".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "monitoring_queue".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "Set the gearman_queue for a route_network".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "firmware_versions".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A(BMC|BIOS|MIDPLANE)_[0-9a-z\\.]+\\Z".to_string(),
readonly: false,
hovertext: "Versions of various firmware components. See regexp for notation and supported attributes.".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "subproject".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A[a-z][a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "backup_storage".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "Storage server for the host's backups. This is normally auto-filled by the system while doing the first backup. Changes needs a puppet run on both the host and new storage.".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "dhcp_networks".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "hw_loadbalancer_group".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "project".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: true,
default: "ndco".to_string(),
});vec.push(Attribute {
name: "storage_hardware".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A(raid|adaptec|threeware|perc700|perc710|perc730|perc310|perc330|perc200|sas1068|non-raid)\\Z".to_string(),
readonly: false,
hovertext: "The type of the raid controller if installed".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "monitoring_groups".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "auto_update".to_string(),
typ: Boolean,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(true|false)\\Z".to_string(),
readonly: false,
hovertext: "This is used by automated scripts to decide if this server should be automatically updated in certain situations".to_string(),
required: true,
default: "0".to_string(),
});vec.push(Attribute {
name: "backup_maxage".to_string(),
typ: Number,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(0|[1-9][0-9]*)\\Z".to_string(),
readonly: false,
hovertext: "The expected freshness of the latest registered backup of the host. Defaults to 24 hours. If you have multiple backup windows, you might consider lowering this. 0 is a special value, disables backup age checking completely.".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "os".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(android|cumulus|centos64|wheezy|jessie|stretch|buster|bullseye|bookworm|bionic|freebsd_[0-9]+\\.[0-9]+|macos_.*|tvos|other|esx|ciscoios|ios12.2|iDRAC6|windows(2022_std|2019_std|2019_dc|2016_std|2016_dc|2012r2_std|2012r2_dc|2012_std|2012_dc|2008r2_std|2008r2_dc|7|8|10|11))\\Z".to_string(),
readonly: false,
hovertext: "Operation system, e.g. wheezy, jessie or freebsd100 or macos_*".to_string(),
required: true,
default: "jessie".to_string(),
});vec.push(Attribute {
name: "repositories_host".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A([a-z0-9]+(-[a-z0-9]+)*\\.)+[a-z]{2,}\\Z".to_string(),
readonly: false,
hovertext: "Repositories hostname to fetch packages from. On Debian this is where apt fetches debian packages from (aptly). For FreeBSD this is where pkg fetches FreeBSD packages from.\r\nIt is used to form the correct configs on the hosts.".to_string(),
required: false,
default: "aptly.innogames.de".to_string(),
});vec.push(Attribute {
name: "officetool_id".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\Ai?[0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "ipv4".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty IPv4 address. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "ipv6".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty IPv6 address. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "hostname".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty hostname attribute. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "servertype".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty servertype attribute. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "object_id".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty object_id attribute. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
}); vec });
servertypes.insert("acl".to_string(), { let mut vec = Vec::new(); vec.push(Attribute {
name: "acl_members".to_string(),
typ: Relation,
reversed_attribute_id: "acl".to_string(),
multi: true,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: true,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "environment".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(testing|staging|production)\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: true,
default: "production".to_string(),
});vec.push(Attribute {
name: "function".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "Function of host".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "protocol_port".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A((any|esp|ah)|((tcp|udp)([0-9]+([lt]+)?|[0-9]+(-[0-9]+)?(_[0-9]+)?))|((ping)|((tcp|udp)([0-9]+t?(-[0-9]+)?))))\\Z".to_string(),
readonly: true,
hovertext: "Ports and protocols to be forwarded to Nodes belonging to this LB Pool.".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "no_logging".to_string(),
typ: Boolean,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(true|false)\\Z".to_string(),
readonly: false,
hovertext: "To disable logging of traffic on firewall".to_string(),
required: false,
default: "true".to_string(),
});vec.push(Attribute {
name: "subproject".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A[a-z][a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "no_monitoring".to_string(),
typ: Boolean,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(true|false)\\Z".to_string(),
readonly: false,
hovertext: "if set to true this server will be ignored by the nagios generation script".to_string(),
required: false,
default: "true".to_string(),
});vec.push(Attribute {
name: "state".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(retired|maintenance|cold_standby|deploy_offline|deploy_online|online_reserved|online)\\Z".to_string(),
readonly: false,
hovertext: "Retired: No DNS, no Nagios, lb downtimed\r\nMaintenance: DNS, Nagios downtimed, lb downtimed\r\nDeploy_offline: DNS, Nagios active, lb downtimed\r\nDeploy_online: DNS, Nagios downtimed, lb active\r\nOnline: DNS, Nagios active, lb active".to_string(),
required: true,
default: "online".to_string(),
});vec.push(Attribute {
name: "responsible_admin".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A[a-z][a-z\\._]+[a-z]\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "project".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "allow_from_reverse".to_string(),
typ: Relation,
reversed_attribute_id: "allow_from".to_string(),
multi: true,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: true,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "allow_to_reverse".to_string(),
typ: Relation,
reversed_attribute_id: "allow_to".to_string(),
multi: true,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: true,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "hostname".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty hostname attribute. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "servertype".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty servertype attribute. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "object_id".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty object_id attribute. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
}); vec });
servertypes.insert("room_tablet".to_string(), { let mut vec = Vec::new(); vec.push(Attribute {
name: "provider_network".to_string(),
typ: Supernet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: true,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "order_date".to_string(),
typ: Date,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "When this server was ordered, for warranty and to find the order".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "order_id".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "some reference to the ticked, bill or order for this one".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "hardware_model".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "route_network".to_string(),
typ: Supernet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: true,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "mac".to_string(),
typ: Macaddr,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "state".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(retired|maintenance|cold_standby|deploy_offline|deploy_online|online_reserved|online)\\Z".to_string(),
readonly: false,
hovertext: "Retired: No DNS, no Nagios, lb downtimed\r\nMaintenance: DNS, Nagios downtimed, lb downtimed\r\nDeploy_offline: DNS, Nagios active, lb downtimed\r\nDeploy_online: DNS, Nagios downtimed, lb active\r\nOnline: DNS, Nagios active, lb active".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "project".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: true,
default: "officeit".to_string(),
});vec.push(Attribute {
name: "primary_ip6".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A[^\\.]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "network_zones".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "A zone defining routing and firewall rules, a kind of sub-datacenter. It is a multi attribute for \"network_zone\"".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "officetool_id".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\Ai?[0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "ipv4".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty IPv4 address. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "ipv6".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty IPv6 address. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "hostname".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty hostname attribute. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "servertype".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty servertype attribute. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "object_id".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty object_id attribute. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
}); vec });
servertypes.insert("azure_network".to_string(), { let mut vec = Vec::new(); vec.push(Attribute {
name: "ipv4".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty IPv4 address. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "ipv6".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty IPv6 address. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "hostname".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty hostname attribute. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "servertype".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty servertype attribute. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "object_id".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty object_id attribute. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
}); vec });
servertypes.insert("public_domain".to_string(), { let mut vec = Vec::new(); vec.push(Attribute {
name: "project".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "monitoring_checks".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A[a-z][a-z0-9_\\-@]*\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "no_monitoring".to_string(),
typ: Boolean,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(true|false)\\Z".to_string(),
readonly: false,
hovertext: "if set to true this server will be ignored by the nagios generation script".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "subproject".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A[a-z][a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "responsible_admin".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A[a-z][a-z\\._]+[a-z]\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "service_groups".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "letsencrypt_enabled".to_string(),
typ: Boolean,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(true|false)\\Z".to_string(),
readonly: false,
hovertext: "If you enable this for a public domain, our helper tool will try to generate a letsencrypt certificate for this domain and put it into puppet/acmekeys repo (& keep it up-to-date).".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "dns_soa".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A([a-zA-Z0-9-.]+\\.)\\s+([a-zA-Z0-9-.]+\\.)\\s+(\\d+)\\s+(\\d+)\\s+(\\d+)\\s+(\\d+)\\s+(\\d+)\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "dns_ns".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A(?:(?:[a-z01-9\\-]*[a-z01-9]\\.)+(?:[a-z]+))\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "dns_txt".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "dns_cname".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(?:(?:[a-zA-Z01-9\\-]*[a-zA-Z01-9]\\.)+(?:[a-z]+))\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "loadbalancer_nodes".to_string(),
typ: Relation,
reversed_attribute_id: "domain".to_string(),
multi: true,
regexp: "\\A.*\\Z".to_string(),
readonly: true,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "dns_mx".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A.+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "dns_spf".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A.+\\Z".to_string(),
readonly: false,
hovertext: "SPF DNS record:\r\nprio, weight, port, target\r\n\r\nExample:\r\n10 0 5269 jabber.innogames.de".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "dns_srv".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A\\d+\\s\\d+\\s\\d+\\s[a-zA-Z0-9.-]+\\Z".to_string(),
readonly: false,
hovertext: "SRV DNS record".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "hostname".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty hostname attribute. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "servertype".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty servertype attribute. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "object_id".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty object_id attribute. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
}); vec });
servertypes.insert("hw_internalrouter".to_string(), { let mut vec = Vec::new(); vec.push(Attribute {
name: "service_groups".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "order_date".to_string(),
typ: Date,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "When this server was ordered, for warranty and to find the order".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "order_id".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "some reference to the ticked, bill or order for this one".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "puppet_disabled".to_string(),
typ: Boolean,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(true|false)\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "False".to_string(),
});vec.push(Attribute {
name: "graphite_graphs".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A[a-z][a-z0-9_\\.\\-]*\\Z".to_string(),
readonly: false,
hovertext: "a list of graphs to reference which graphs should be displayed on this server to be used in the collections (doku link).".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "project_network".to_string(),
typ: Supernet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: true,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "primary_ip6".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A[^\\.]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "dell_servicetag".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "The Service Tag for Dell to be used in the support interface and to identify the hardware".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "repositories".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A\\w+:\\w+:\\w+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "arch".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: true,
default: "amd64".to_string(),
});vec.push(Attribute {
name: "provisioning_step".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A([a-z0-9]+[_])*[a-z0-9]+\\|([a-z0-9]+[\\.\\-])*[a-z0-9]+\\|\\d{10}\\|\\d{10}\\.\\d{6}\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "puppet_ca".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "specifies the puppet ca server which the server should use".to_string(),
required: true,
default: "puppetca.innogames.de".to_string(),
});vec.push(Attribute {
name: "puppet_master".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "storage_hardware".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A(raid|adaptec|threeware|perc700|perc710|perc730|perc310|perc330|perc200|sas1068|non-raid)\\Z".to_string(),
readonly: false,
hovertext: "The type of the raid controller if installed".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "disk_size_gib".to_string(),
typ: Number,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(0|[1-9][0-9]*)\\Z".to_string(),
readonly: false,
hovertext: "This is the current disk size of the server in gibibytes.  It should be filled while the VM is created or maintained by other scripts.\r\n".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "memory".to_string(),
typ: Number,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(0|[1-9][0-9]*)\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "num_cpu".to_string(),
typ: Number,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(0|[1-9][0-9]*)\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "cpu_clock_mhz".to_string(),
typ: Number,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(0|[1-9][0-9]*)\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "ipmi".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "function".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "Function of host".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "bladecenter".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "puppet_environment".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(?!(testing|staging|production))[a-z][a-z0-9_]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "no_monitoring".to_string(),
typ: Boolean,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(true|false)\\Z".to_string(),
readonly: false,
hovertext: "if set to true this server will be ignored by the nagios generation script".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "hardware_model".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "harddisk_type".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(ssd|hdd|unknown)\\Z".to_string(),
readonly: false,
hovertext: "this is either hdd or ssd to give an indication on the performance and io scheduler to be used".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "cpu_model".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "rack_height".to_string(),
typ: Number,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A[0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "rack_position".to_string(),
typ: Number,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A[0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "hardware_components".to_string(),
typ: Relation,
reversed_attribute_id: "installed_in".to_string(),
multi: true,
regexp: "\\A.*\\Z".to_string(),
readonly: true,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "memory_modules".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "bladecenter_slot".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "datacenter".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "rack".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "responsible_admin".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A[a-z][a-z\\._]+[a-z]\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "kajetan.staszkiewicz".to_string(),
});vec.push(Attribute {
name: "route_network".to_string(),
typ: Supernet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: true,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "monitoring_checks".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A[a-z][a-z0-9_\\-@]*\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "ping".to_string(),
});vec.push(Attribute {
name: "sshfp".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A[1-4] (1 [0-9a-f]{40}|2 [0-9a-f]{64})\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "puppet_classes".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A[a-z][a-z0-9_]*(::[a-z][a-z0-9_]*)*\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "vlan_interfaces".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "vlan_networks".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "state".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(retired|maintenance|cold_standby|deploy_offline|deploy_online|online_reserved|online)\\Z".to_string(),
readonly: false,
hovertext: "Retired: No DNS, no Nagios, lb downtimed\r\nMaintenance: DNS, Nagios downtimed, lb downtimed\r\nDeploy_offline: DNS, Nagios active, lb downtimed\r\nDeploy_online: DNS, Nagios downtimed, lb active\r\nOnline: DNS, Nagios active, lb active".to_string(),
required: true,
default: "online".to_string(),
});vec.push(Attribute {
name: "environment".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(testing|staging|production)\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: true,
default: "production".to_string(),
});vec.push(Attribute {
name: "monitoring_queue".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "Set the gearman_queue for a route_network".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "network_zones".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "A zone defining routing and firewall rules, a kind of sub-datacenter. It is a multi attribute for \"network_zone\"".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "network_zone".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "A zone defining routing and firewall rules, a kind of sub-datacenter.".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "subproject".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A[a-z][a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "backup_storage".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "Storage server for the host's backups. This is normally auto-filled by the system while doing the first backup. Changes needs a puppet run on both the host and new storage.".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "firmware_versions".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A(BMC|BIOS|MIDPLANE)_[0-9a-z\\.]+\\Z".to_string(),
readonly: false,
hovertext: "Versions of various firmware components. See regexp for notation and supported attributes.".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "project".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: true,
default: "ndco".to_string(),
});vec.push(Attribute {
name: "auto_update".to_string(),
typ: Boolean,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(true|false)\\Z".to_string(),
readonly: false,
hovertext: "This is used by automated scripts to decide if this server should be automatically updated in certain situations".to_string(),
required: true,
default: "0".to_string(),
});vec.push(Attribute {
name: "backup_disabled".to_string(),
typ: Boolean,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(true|false)\\Z".to_string(),
readonly: false,
hovertext: "Set to true if you do not want this server backed up.".to_string(),
required: true,
default: "1".to_string(),
});vec.push(Attribute {
name: "backup_canshrink".to_string(),
typ: Boolean,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(true|false)\\Z".to_string(),
readonly: false,
hovertext: "Monitoring system will warn in case last backup size of the host drops below 50% of its last 10 backups' median size. Set this to true if you expect a shrinking backup (e.g. on \"speed worlds\" which are reset from time to time) so it doesn't complain on suddenly shrinking backup size.".to_string(),
required: false,
default: "0".to_string(),
});vec.push(Attribute {
name: "backup_maxage".to_string(),
typ: Number,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(0|[1-9][0-9]*)\\Z".to_string(),
readonly: false,
hovertext: "The expected freshness of the latest registered backup of the host. Defaults to 24 hours. If you have multiple backup windows, you might consider lowering this. 0 is a special value, disables backup age checking completely.".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "backup_window".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A\\d+-\\d+\\Z".to_string(),
readonly: false,
hovertext: "This is a list of hour ranges (in CEST) when the backup should be scheduled on the server locally. E.g. \"3-4\" means create one backup between 03:00 and 04:00. The backups will be transferred in order of backup_prio attribute once the local script ran through and reported a success so not guaranteed to be within the backup_window (but mostly does fit in).".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "backup_prio".to_string(),
typ: Number,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(0|[1-9][0-9]*)\\Z".to_string(),
readonly: false,
hovertext: "Backup priority of the host. Defaults to 300. Backups will be eventually processed regardless of the value of this, but higher number means the dump and transfer of the backup will be prioritized.".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "powerports".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "switchports".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "Switchports attribute is used by Foreman to configure switches. That includes creating proper LAGGs and assigning VLANs to them. ".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "mac".to_string(),
typ: Macaddr,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "os".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(android|cumulus|centos64|wheezy|jessie|stretch|buster|bullseye|bookworm|bionic|freebsd_[0-9]+\\.[0-9]+|macos_.*|tvos|other|esx|ciscoios|ios12.2|iDRAC6|windows(2022_std|2019_std|2019_dc|2016_std|2016_dc|2012r2_std|2012r2_dc|2012_std|2012_dc|2008r2_std|2008r2_dc|7|8|10|11))\\Z".to_string(),
readonly: false,
hovertext: "Operation system, e.g. wheezy, jessie or freebsd100 or macos_*".to_string(),
required: true,
default: "freebsd10".to_string(),
});vec.push(Attribute {
name: "repositories_host".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A([a-z0-9]+(-[a-z0-9]+)*\\.)+[a-z]{2,}\\Z".to_string(),
readonly: false,
hovertext: "Repositories hostname to fetch packages from. On Debian this is where apt fetches debian packages from (aptly). For FreeBSD this is where pkg fetches FreeBSD packages from.\r\nIt is used to form the correct configs on the hosts.".to_string(),
required: false,
default: "aptly.innogames.de".to_string(),
});vec.push(Attribute {
name: "network_link_type".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(kernel|ovs)_(single|bond_lacp|bond_ab)\\Z".to_string(),
readonly: false,
hovertext: "The type of physical connection between the hardware server and switches. The first part either 'ovs' if you use OpenVSwitch or 'kernel' if it's just plain Linux/Windows. Second part is either 'single' for devices with a single network cable or 'bond_ab' or 'bond_lacp' for servers with redundant network link.".to_string(),
required: true,
default: "kernel_bond_ab".to_string(),
});vec.push(Attribute {
name: "monitoring_groups".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "tunnel_interfaces".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "ipv4".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty IPv4 address. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "ipv6".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty IPv6 address. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "hostname".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty hostname attribute. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "servertype".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty servertype attribute. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "object_id".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty object_id attribute. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
}); vec });
servertypes.insert("project_network".to_string(), { let mut vec = Vec::new(); vec.push(Attribute {
name: "function".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "Function of host".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "datacenter".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "subproject".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A[a-z][a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "provider_network".to_string(),
typ: Supernet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: true,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "default_gateway".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "primary_ip6".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A[^\\.]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "internal_gateway".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "acl".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: true,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "service_groups".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "public_networks".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "route_network".to_string(),
typ: Supernet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: true,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "game_market".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A[a-z][a-z]|foe|war\\Z".to_string(),
readonly: false,
hovertext: "The 2 letter short code for the game market, see docu for allowed values.".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "allow_from".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: true,
hovertext: "List of Access lists from which access to this LB Pool is allowed. Connections from outside of allowed networks will be silently dropped.".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "network_type".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(loopback|transit_(internal_igp|public_(hwlbs|uplink|igp))|public|internal|public_dmz|vpn_clients|internal_vpn_clients|cloud)\\Z".to_string(),
readonly: false,
hovertext: "Select what kind of connection you want to switch\r\nsingle: single interface without lacp or anything other special\r\nbond_ab: Bonding without LACP but 2 links with failover\r\nbond_lacp: bonding with lacp and 2 active links".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "monitoring_queue".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "Set the gearman_queue for a route_network".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "state".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(retired|maintenance|cold_standby|deploy_offline|deploy_online|online_reserved|online)\\Z".to_string(),
readonly: false,
hovertext: "Retired: No DNS, no Nagios, lb downtimed\r\nMaintenance: DNS, Nagios downtimed, lb downtimed\r\nDeploy_offline: DNS, Nagios active, lb downtimed\r\nDeploy_online: DNS, Nagios downtimed, lb active\r\nOnline: DNS, Nagios active, lb active".to_string(),
required: true,
default: "online".to_string(),
});vec.push(Attribute {
name: "environment".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(testing|staging|production)\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: true,
default: "production".to_string(),
});vec.push(Attribute {
name: "project".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "network_zones".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "A zone defining routing and firewall rules, a kind of sub-datacenter. It is a multi attribute for \"network_zone\"".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "aws_placement".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A(ap|ca|eu|me|sa|us)\\-(central|east|north|northeast|south|west)\\-[1-9][a-z]\\Z".to_string(),
readonly: false,
hovertext: "AWS Placement".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "aws_subnet_id".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\Asubnet-[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "AWS SubnetId".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "aws_security_group_ids".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\Asg-[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "AWS SecurityGroupIds".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "datacenter_type".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(aws|digitalocean|hetzner|hyperv|kvm)\\.dct\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "aws_vpc_id".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\Avpc-[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "Aws VpcId".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "nessus_folder_name".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(.*)\\Z".to_string(),
readonly: false,
hovertext: "The name of the nessus scan that includes this project_network".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "nessus_scan_id".to_string(),
typ: Number,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A[0-9]+\\Z".to_string(),
readonly: false,
hovertext: "The id of the scan in nessus".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "ipv4".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty IPv4 address. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "ipv6".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty IPv6 address. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "hostname".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty hostname attribute. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "servertype".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty servertype attribute. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "object_id".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty object_id attribute. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
}); vec });
servertypes.insert("vm".to_string(), { let mut vec = Vec::new(); vec.push(Attribute {
name: "group_company".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A[a-zA-Z]+\\Z".to_string(),
readonly: false,
hovertext: "Company this host is serving e.g. Hutch, Congregate (Default: InnoGames)".to_string(),
required: false,
default: "InnoGames".to_string(),
});vec.push(Attribute {
name: "monitoring_satellite_of".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "icinga2".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "mac".to_string(),
typ: Macaddr,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "game_market".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A[a-z][a-z]|foe|war\\Z".to_string(),
readonly: false,
hovertext: "The 2 letter short code for the game market, see docu for allowed values.".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "routed_networks".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "monitoring_queues".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "gearman_queues a worker should be responsible for".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "ipv4_only".to_string(),
typ: Boolean,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(true|false)\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "auto_update".to_string(),
typ: Boolean,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(true|false)\\Z".to_string(),
readonly: false,
hovertext: "This is used by automated scripts to decide if this server should be automatically updated in certain situations".to_string(),
required: false,
default: "True".to_string(),
});vec.push(Attribute {
name: "game_world".to_string(),
typ: Number,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(0|[1-9][0-9]*)\\Z".to_string(),
readonly: false,
hovertext: "The numerical part of the world id for games 0 is reserved for master or servers with market wide functionality".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "backup_disabled".to_string(),
typ: Boolean,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(true|false)\\Z".to_string(),
readonly: false,
hovertext: "Set to true if you do not want this server backed up.".to_string(),
required: false,
default: "False".to_string(),
});vec.push(Attribute {
name: "backup_canshrink".to_string(),
typ: Boolean,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(true|false)\\Z".to_string(),
readonly: false,
hovertext: "Monitoring system will warn in case last backup size of the host drops below 50% of its last 10 backups' median size. Set this to true if you expect a shrinking backup (e.g. on \"speed worlds\" which are reset from time to time) so it doesn't complain on suddenly shrinking backup size.".to_string(),
required: false,
default: "False".to_string(),
});vec.push(Attribute {
name: "os".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(android|cumulus|centos64|wheezy|jessie|stretch|buster|bullseye|bookworm|bionic|freebsd_[0-9]+\\.[0-9]+|macos_.*|tvos|other|esx|ciscoios|ios12.2|iDRAC6|windows(2022_std|2019_std|2019_dc|2016_std|2016_dc|2012r2_std|2012r2_dc|2012_std|2012_dc|2008r2_std|2008r2_dc|7|8|10|11))\\Z".to_string(),
readonly: false,
hovertext: "Operation system, e.g. wheezy, jessie or freebsd100 or macos_*".to_string(),
required: false,
default: "buster".to_string(),
});vec.push(Attribute {
name: "backup_prio".to_string(),
typ: Number,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(0|[1-9][0-9]*)\\Z".to_string(),
readonly: false,
hovertext: "Backup priority of the host. Defaults to 300. Backups will be eventually processed regardless of the value of this, but higher number means the dump and transfer of the backup will be prioritized.".to_string(),
required: false,
default: "300".to_string(),
});vec.push(Attribute {
name: "num_cpu".to_string(),
typ: Number,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(0|[1-9][0-9]*)\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: true,
default: "4".to_string(),
});vec.push(Attribute {
name: "no_monitoring".to_string(),
typ: Boolean,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(true|false)\\Z".to_string(),
readonly: false,
hovertext: "if set to true this server will be ignored by the nagios generation script".to_string(),
required: false,
default: "False".to_string(),
});vec.push(Attribute {
name: "puppet_disabled".to_string(),
typ: Boolean,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(true|false)\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "False".to_string(),
});vec.push(Attribute {
name: "primary_ip6".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A[^\\.]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "memory".to_string(),
typ: Number,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(0|[1-9][0-9]*)\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: true,
default: "2048".to_string(),
});vec.push(Attribute {
name: "disk_size_gib".to_string(),
typ: Number,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(0|[1-9][0-9]*)\\Z".to_string(),
readonly: false,
hovertext: "This is the current disk size of the server in gibibytes.  It should be filled while the VM is created or maintained by other scripts.\r\n".to_string(),
required: true,
default: "6".to_string(),
});vec.push(Attribute {
name: "security_availability".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(L|M|H|C)\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "puppet_ca".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "specifies the puppet ca server which the server should use".to_string(),
required: true,
default: "puppetca.innogames.de".to_string(),
});vec.push(Attribute {
name: "puppet_master".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "backup_state".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(idle|local_backup_running|local_backup_done|local_backup_failed|restic_running|restic_failed|waiting_local_backup_start|please_backup_now|dead)\\Z".to_string(),
readonly: false,
hovertext: "Current state of the backup. Used by backup system, might not be always 100% accurate (by design).".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "provider_network".to_string(),
typ: Supernet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: true,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "route_network".to_string(),
typ: Supernet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: true,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "graphite_graphs".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A[a-z][a-z0-9_\\.\\-]*\\Z".to_string(),
readonly: false,
hovertext: "a list of graphs to reference which graphs should be displayed on this server to be used in the collections (doku link).".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "served_markets".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "Like game_market used to group (backend) servers for multiple market(s)".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "loadbalancer".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "A list of LB Pools to which this Node belongs.".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "datacenter".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "igvm_locked".to_string(),
typ: Datetime,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "Set when object is being manipulated by igvm, e.g. being build or migrated.".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "repositories".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A\\w+:\\w+:\\w+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "int:innogames:stable".to_string(),
});vec.push(Attribute {
name: "security_confidentiality".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(L|M|H|C)\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "fixup_enabled".to_string(),
typ: Boolean,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(true|false)\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "security_integrity".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(L|M|H|C)\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "environment".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(testing|staging|production)\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: true,
default: "production".to_string(),
});vec.push(Attribute {
name: "puppet_environment".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(?!(testing|staging|production))[a-z][a-z0-9_]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "function".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "Function of host".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "os_product_license".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "backup_window".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A\\d+-\\d+\\Z".to_string(),
readonly: false,
hovertext: "This is a list of hour ranges (in CEST) when the backup should be scheduled on the server locally. E.g. \"3-4\" means create one backup between 03:00 and 04:00. The backups will be transferred in order of backup_prio attribute once the local script ran through and reported a success so not guaranteed to be within the backup_window (but mostly does fit in).".to_string(),
required: false,
default: "2-5".to_string(),
});vec.push(Attribute {
name: "vlan_interfaces".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "allow_to".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: true,
hovertext: "If set on any server in a segment it will block all access from servers in this segment to the internet unless they are explicitly given a value here.".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "bladecenter".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "bladecenter_slot".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "game_type".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "Currently only used to mark Tribalwars Speed (s), Classic (c), and Casual (p) special worlds".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "legal_personal_data".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(nothing|personal|highly_personal)\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "served_game".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "Like game_market used to group (backend) servers for one/multiple game(s)".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "project_network".to_string(),
typ: Supernet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: true,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "responsible_admin".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A[a-z][a-z\\._]+[a-z]\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "hypervisor".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "Hostname of the Hypervisor on which this VM resides".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "hypervisor_standby".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "Hostname of the Hypervisor on which hot copy of this VM resides".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "subproject".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A[a-z][a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "sshfp".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A[1-4] (1 [0-9a-f]{40}|2 [0-9a-f]{64})\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "barman_host".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "monitoring_checks".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A[a-z][a-z0-9_\\-@]*\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "ping".to_string(),
});vec.push(Attribute {
name: "puppet_classes".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A[a-z][a-z0-9_]*(::[a-z][a-z0-9_]*)*\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "allow_from".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: true,
hovertext: "List of Access lists from which access to this LB Pool is allowed. Connections from outside of allowed networks will be silently dropped.".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "acl".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: true,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "description".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "Intended to give an external auditor a very general idea of what the system is about.".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "project_domain".to_string(),
typ: Domain,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: true,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "iggop_locked".to_string(),
typ: Boolean,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(true|false)\\Z".to_string(),
readonly: false,
hovertext: "The object or a related object is worked on by iggop and should not be used.".to_string(),
required: false,
default: "False".to_string(),
});vec.push(Attribute {
name: "instance".to_string(),
typ: Number,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A[0-9]+\\Z".to_string(),
readonly: false,
hovertext: "used to index multiple instances of the same function".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "state".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(retired|maintenance|cold_standby|deploy_offline|deploy_online|online_reserved|online)\\Z".to_string(),
readonly: false,
hovertext: "Retired: No DNS, no Nagios, lb downtimed\r\nMaintenance: DNS, Nagios downtimed, lb downtimed\r\nDeploy_offline: DNS, Nagios active, lb downtimed\r\nDeploy_online: DNS, Nagios downtimed, lb active\r\nOnline: DNS, Nagios active, lb active".to_string(),
required: true,
default: "online".to_string(),
});vec.push(Attribute {
name: "aws_subnet_id".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\Asubnet-[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "AWS SubnetId".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "network_zone".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "A zone defining routing and firewall rules, a kind of sub-datacenter.".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "monitoring_queue".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "Set the gearman_queue for a route_network".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "lb_multihoming".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(primary|secondary|noauto)\\Z".to_string(),
readonly: false,
hovertext: "Decide which datacenter is to be used as primary one for LB Pool of this LB Node.".to_string(),
required: false,
default: "secondary".to_string(),
});vec.push(Attribute {
name: "network_zones".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "A zone defining routing and firewall rules, a kind of sub-datacenter. It is a multi attribute for \"network_zone\"".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "backup_storage".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "Storage server for the host's backups. This is normally auto-filled by the system while doing the first backup. Changes needs a puppet run on both the host and new storage.".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "service_groups".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "io_weight".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(default|low)\\Z".to_string(),
readonly: false,
hovertext: "Defines the I/O weight for a VM, setting this to low means other VMs on the same Hypervisor will get preferred I/O.".to_string(),
required: true,
default: "default".to_string(),
});vec.push(Attribute {
name: "project".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "aws_placement".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A(ap|ca|eu|me|sa|us)\\-(central|east|north|northeast|south|west)\\-[1-9][a-z]\\Z".to_string(),
readonly: false,
hovertext: "AWS Placement".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "aws_image_id".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(ami-)([a-z0-9]+)\\Z".to_string(),
readonly: false,
hovertext: "AWS ImageID".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "aws_instance_id".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\Ai-[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "AWS InstanceId".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "aws_instance_type".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A[acimrptxz][0-9]+[a|d|n|e]?\\.[0-9]*x?(large|micro)\\Z".to_string(),
readonly: false,
hovertext: "AWS InstanceType".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "aws_key_name".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A[a-zA-Z0-9@-]+\\Z".to_string(),
readonly: false,
hovertext: "AWS KeyName".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "datacenter_type".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(aws|digitalocean|hetzner|hyperv|kvm)\\.dct\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "aws_security_group_ids".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\Asg-[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "AWS SecurityGroupIds".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "restic_keep_monthly".to_string(),
typ: Number,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A[1-9][0-9]?$|^100\\Z".to_string(),
readonly: false,
hovertext: "Monthly backup count to keep for this host, 1-100".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "restic_keep_yearly".to_string(),
typ: Number,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A[0-9]$|^10\\Z".to_string(),
readonly: false,
hovertext: "Yearly backup count to keep for this host, 1-10".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "aws_vpc_id".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\Avpc-[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "Aws VpcId".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "restic_group".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A[a-z0-9][a-z0-9-]+[a-z0-9]\\Z".to_string(),
readonly: false,
hovertext: "Name of the backup bucket of this server, project (+ \"-subproject\" if defined) when empty by default".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "vlan_networks".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "restic_keep_daily".to_string(),
typ: Number,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A[1-9][0-9]?$|^100\\Z".to_string(),
readonly: false,
hovertext: "Daily backup count to keep for this host, 1-100".to_string(),
required: false,
default: "3".to_string(),
});vec.push(Attribute {
name: "restic_keep_last".to_string(),
typ: Number,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A([1-3][0-9][0-9]|[0-9][0-9]|[0-9])\\Z".to_string(),
readonly: false,
hovertext: "Latest backup count to keep for this host, 1-399".to_string(),
required: false,
default: "3".to_string(),
});vec.push(Attribute {
name: "restic_keep_weekly".to_string(),
typ: Number,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A[1-9][0-9]?$|^100\\Z".to_string(),
readonly: false,
hovertext: "Weekly backup count to keep for this host, 1-100".to_string(),
required: false,
default: "2".to_string(),
});vec.push(Attribute {
name: "load_99".to_string(),
typ: Number,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(-?[0-9]+)\\Z".to_string(),
readonly: true,
hovertext: "".to_string(),
required: false,
default: "0".to_string(),
});vec.push(Attribute {
name: "igvm_action".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(build|migrate|migrate_offline|in_progress|successful|failed)\\Z".to_string(),
readonly: false,
hovertext: "Current state of VM for IGVM daemon".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "powa_enabled".to_string(),
typ: Boolean,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(true|false)\\Z".to_string(),
readonly: false,
hovertext: "True if Powa is enabled on the host. This attribute is managed by puppet, don't set it manually.".to_string(),
required: false,
default: "false".to_string(),
});vec.push(Attribute {
name: "backup_maxage".to_string(),
typ: Number,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(0|[1-9][0-9]*)\\Z".to_string(),
readonly: false,
hovertext: "The expected freshness of the latest registered backup of the host. Defaults to 24 hours. If you have multiple backup windows, you might consider lowering this. 0 is a special value, disables backup age checking completely.".to_string(),
required: false,
default: "24".to_string(),
});vec.push(Attribute {
name: "lb_pool".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "A list of LB Pools to which this Node belongs.".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "rack".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "backup_timestamp".to_string(),
typ: Datetime,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "Last action time of backup system (UTC). Should be set automatically by backup manager.\r\nWe're utilizing this like a cache to quickly check the freshness of the state (real source of truth is the client itself, but not always).".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "repositories_host".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A([a-z0-9]+(-[a-z0-9]+)*\\.)+[a-z]{2,}\\Z".to_string(),
readonly: false,
hovertext: "Repositories hostname to fetch packages from. On Debian this is where apt fetches debian packages from (aptly). For FreeBSD this is where pkg fetches FreeBSD packages from.\r\nIt is used to form the correct configs on the hosts.".to_string(),
required: false,
default: "aptly.innogames.de".to_string(),
});vec.push(Attribute {
name: "monitoring_groups".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "internal_domain".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+(\\.|-+))*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "nessus_scan_id".to_string(),
typ: Number,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A[0-9]+\\Z".to_string(),
readonly: false,
hovertext: "The id of the scan in nessus".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "ipv4".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty IPv4 address. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "ipv6".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty IPv6 address. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "hostname".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty hostname attribute. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "servertype".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty servertype attribute. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "object_id".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty object_id attribute. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
}); vec });
servertypes.insert("accesspoint".to_string(), { let mut vec = Vec::new(); vec.push(Attribute {
name: "mac".to_string(),
typ: Macaddr,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "state".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(retired|maintenance|cold_standby|deploy_offline|deploy_online|online_reserved|online)\\Z".to_string(),
readonly: false,
hovertext: "Retired: No DNS, no Nagios, lb downtimed\r\nMaintenance: DNS, Nagios downtimed, lb downtimed\r\nDeploy_offline: DNS, Nagios active, lb downtimed\r\nDeploy_online: DNS, Nagios downtimed, lb active\r\nOnline: DNS, Nagios active, lb active".to_string(),
required: true,
default: "online".to_string(),
});vec.push(Attribute {
name: "switchports".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "Switchports attribute is used by Foreman to configure switches. That includes creating proper LAGGs and assigning VLANs to them. ".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "dell_servicetag".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "The Service Tag for Dell to be used in the support interface and to identify the hardware".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "primary_ip6".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A[^\\.]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "hardware_model".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "vlan_networks".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "route_network".to_string(),
typ: Supernet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: true,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "sshfp".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A[1-4] (1 [0-9a-f]{40}|2 [0-9a-f]{64})\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "monitoring_groups".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "provider_network".to_string(),
typ: Supernet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: true,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "responsible_admin".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A[a-z][a-z\\._]+[a-z]\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "monitoring_queue".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "Set the gearman_queue for a route_network".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "datacenter".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "description".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "Intended to give an external auditor a very general idea of what the system is about.".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "project".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: true,
default: "officeit".to_string(),
});vec.push(Attribute {
name: "network_zones".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "A zone defining routing and firewall rules, a kind of sub-datacenter. It is a multi attribute for \"network_zone\"".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "officetool_id".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\Ai?[0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "ipv4".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty IPv4 address. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "ipv6".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty IPv6 address. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "hostname".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty hostname attribute. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "servertype".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty servertype attribute. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "object_id".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty object_id attribute. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
}); vec });
servertypes.insert("printer".to_string(), { let mut vec = Vec::new(); vec.push(Attribute {
name: "mac".to_string(),
typ: Macaddr,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "provider_network".to_string(),
typ: Supernet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: true,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "hardware_model".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "primary_ip6".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A[^\\.]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "state".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(retired|maintenance|cold_standby|deploy_offline|deploy_online|online_reserved|online)\\Z".to_string(),
readonly: false,
hovertext: "Retired: No DNS, no Nagios, lb downtimed\r\nMaintenance: DNS, Nagios downtimed, lb downtimed\r\nDeploy_offline: DNS, Nagios active, lb downtimed\r\nDeploy_online: DNS, Nagios downtimed, lb active\r\nOnline: DNS, Nagios active, lb active".to_string(),
required: true,
default: "online".to_string(),
});vec.push(Attribute {
name: "route_network".to_string(),
typ: Supernet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: true,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "project".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: true,
default: "officeit".to_string(),
});vec.push(Attribute {
name: "monitoring_groups".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "network_zones".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "A zone defining routing and firewall rules, a kind of sub-datacenter. It is a multi attribute for \"network_zone\"".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "officetool_id".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\Ai?[0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "ipv4".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty IPv4 address. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "ipv6".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty IPv6 address. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "hostname".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty hostname attribute. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "servertype".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty servertype attribute. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "object_id".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty object_id attribute. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
}); vec });
servertypes.insert("cloud_network".to_string(), { let mut vec = Vec::new(); vec.push(Attribute {
name: "state".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(retired|maintenance|cold_standby|deploy_offline|deploy_online|online_reserved|online)\\Z".to_string(),
readonly: false,
hovertext: "Retired: No DNS, no Nagios, lb downtimed\r\nMaintenance: DNS, Nagios downtimed, lb downtimed\r\nDeploy_offline: DNS, Nagios active, lb downtimed\r\nDeploy_online: DNS, Nagios downtimed, lb active\r\nOnline: DNS, Nagios active, lb active".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "service_groups".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "provider".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "project".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "responsible_admin".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A[a-z][a-z\\._]+[a-z]\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "ipv4".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty IPv4 address. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "ipv6".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty IPv6 address. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "hostname".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty hostname attribute. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "servertype".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty servertype attribute. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "object_id".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty object_id attribute. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
}); vec });
servertypes.insert("ipmi".to_string(), { let mut vec = Vec::new(); vec.push(Attribute {
name: "service_groups".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "project_network".to_string(),
typ: Supernet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: true,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "route_network".to_string(),
typ: Supernet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: true,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "dell_servicetag".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "The Service Tag for Dell to be used in the support interface and to identify the hardware".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "primary_ip6".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A[^\\.]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "no_monitoring".to_string(),
typ: Boolean,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(true|false)\\Z".to_string(),
readonly: false,
hovertext: "if set to true this server will be ignored by the nagios generation script".to_string(),
required: false,
default: "false".to_string(),
});vec.push(Attribute {
name: "function".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "Function of host".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "responsible_admin".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A[a-z][a-z\\._]+[a-z]\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "monitoring_queue".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "Set the gearman_queue for a route_network".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "ipmi_of".to_string(),
typ: Relation,
reversed_attribute_id: "ipmi".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: true,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "bladecenter".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "bladecenter_slot".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "rack".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "storage_hardware".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A(raid|adaptec|threeware|perc700|perc710|perc730|perc310|perc330|perc200|sas1068|non-raid)\\Z".to_string(),
readonly: false,
hovertext: "The type of the raid controller if installed".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "network_type".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(loopback|transit_(internal_igp|public_(hwlbs|uplink|igp))|public|internal|public_dmz|vpn_clients|internal_vpn_clients|cloud)\\Z".to_string(),
readonly: false,
hovertext: "Select what kind of connection you want to switch\r\nsingle: single interface without lacp or anything other special\r\nbond_ab: Bonding without LACP but 2 links with failover\r\nbond_lacp: bonding with lacp and 2 active links".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "sshfp".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A[1-4] (1 [0-9a-f]{40}|2 [0-9a-f]{64})\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "datacenter".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "state".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(retired|maintenance|cold_standby|deploy_offline|deploy_online|online_reserved|online)\\Z".to_string(),
readonly: false,
hovertext: "Retired: No DNS, no Nagios, lb downtimed\r\nMaintenance: DNS, Nagios downtimed, lb downtimed\r\nDeploy_offline: DNS, Nagios active, lb downtimed\r\nDeploy_online: DNS, Nagios downtimed, lb active\r\nOnline: DNS, Nagios active, lb active".to_string(),
required: true,
default: "online".to_string(),
});vec.push(Attribute {
name: "environment".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(testing|staging|production)\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: true,
default: "production".to_string(),
});vec.push(Attribute {
name: "hardware_model".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "network_zone".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "A zone defining routing and firewall rules, a kind of sub-datacenter.".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "mac".to_string(),
typ: Macaddr,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "provider_network".to_string(),
typ: Supernet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: true,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "project".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: true,
default: "ndco".to_string(),
});vec.push(Attribute {
name: "network_zones".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "A zone defining routing and firewall rules, a kind of sub-datacenter. It is a multi attribute for \"network_zone\"".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "rack_colo".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "rack_number".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "rack_row".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "monitoring_checks".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A[a-z][a-z0-9_\\-@]*\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "ping, ping_6, ipmi_sel, ipmi_ssl_cert, ipmi_sensors".to_string(),
});vec.push(Attribute {
name: "nessus_scan_id".to_string(),
typ: Number,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A[0-9]+\\Z".to_string(),
readonly: false,
hovertext: "The id of the scan in nessus".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "serial_number".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A[a-zA-Z0-9\\-]+\\Z".to_string(),
readonly: false,
hovertext: "A unique string identifying hardware, like:\r\n- Dell ServiceTag\r\n- Memory module serial number\r\n- Disk WWN address".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "ipv4".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty IPv4 address. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "ipv6".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty IPv6 address. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "hostname".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty hostname attribute. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "servertype".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty servertype attribute. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "object_id".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty object_id attribute. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
}); vec });
servertypes.insert("voip_phone".to_string(), { let mut vec = Vec::new(); vec.push(Attribute {
name: "project".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "route_network".to_string(),
typ: Supernet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: true,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "primary_ip6".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A[^\\.]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "mac".to_string(),
typ: Macaddr,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "network_zones".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "A zone defining routing and firewall rules, a kind of sub-datacenter. It is a multi attribute for \"network_zone\"".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "datacenter".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "state".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(retired|maintenance|cold_standby|deploy_offline|deploy_online|online_reserved|online)\\Z".to_string(),
readonly: false,
hovertext: "Retired: No DNS, no Nagios, lb downtimed\r\nMaintenance: DNS, Nagios downtimed, lb downtimed\r\nDeploy_offline: DNS, Nagios active, lb downtimed\r\nDeploy_online: DNS, Nagios downtimed, lb active\r\nOnline: DNS, Nagios active, lb active".to_string(),
required: true,
default: "online".to_string(),
});vec.push(Attribute {
name: "officetool_id".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\Ai?[0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "ipv4".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty IPv4 address. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "ipv6".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty IPv6 address. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "hostname".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty hostname attribute. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "servertype".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty servertype attribute. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "object_id".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty object_id attribute. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
}); vec });
servertypes.insert("project_domain".to_string(), { let mut vec = Vec::new(); vec.push(Attribute {
name: "mx".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "project".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "provider_domain".to_string(),
typ: Domain,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: true,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "internal".to_string(),
typ: Boolean,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "hostname".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty hostname attribute. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "servertype".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty servertype attribute. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "object_id".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty object_id attribute. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
}); vec });
servertypes.insert("rack".to_string(), { let mut vec = Vec::new(); vec.push(Attribute {
name: "rack_row".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "rack_number".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "rack_colo".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "datacenter".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "responsible_admin".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A[a-z][a-z\\._]+[a-z]\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "order_date".to_string(),
typ: Date,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "When this server was ordered, for warranty and to find the order".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "order_id".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "some reference to the ticked, bill or order for this one".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "state".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(retired|maintenance|cold_standby|deploy_offline|deploy_online|online_reserved|online)\\Z".to_string(),
readonly: false,
hovertext: "Retired: No DNS, no Nagios, lb downtimed\r\nMaintenance: DNS, Nagios downtimed, lb downtimed\r\nDeploy_offline: DNS, Nagios active, lb downtimed\r\nDeploy_online: DNS, Nagios downtimed, lb active\r\nOnline: DNS, Nagios active, lb active".to_string(),
required: true,
default: "online".to_string(),
});vec.push(Attribute {
name: "datacenter_colo".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "project".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: true,
default: "ndco".to_string(),
});vec.push(Attribute {
name: "hostname".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty hostname attribute. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "servertype".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty servertype attribute. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "object_id".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty object_id attribute. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
}); vec });
servertypes.insert("hardware_external".to_string(), { let mut vec = Vec::new(); vec.push(Attribute {
name: "datacenter".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "security_availability".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(L|M|H|C)\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "dell_servicetag".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "The Service Tag for Dell to be used in the support interface and to identify the hardware".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "rack_height".to_string(),
typ: Number,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A[0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "monitoring_queues".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "gearman_queues a worker should be responsible for".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "security_confidentiality".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(L|M|H|C)\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "backup_prio".to_string(),
typ: Number,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(0|[1-9][0-9]*)\\Z".to_string(),
readonly: false,
hovertext: "Backup priority of the host. Defaults to 300. Backups will be eventually processed regardless of the value of this, but higher number means the dump and transfer of the backup will be prioritized.".to_string(),
required: false,
default: "300".to_string(),
});vec.push(Attribute {
name: "order_date".to_string(),
typ: Date,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "When this server was ordered, for warranty and to find the order".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "order_id".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "some reference to the ticked, bill or order for this one".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "no_monitoring".to_string(),
typ: Boolean,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(true|false)\\Z".to_string(),
readonly: false,
hovertext: "if set to true this server will be ignored by the nagios generation script".to_string(),
required: false,
default: "false".to_string(),
});vec.push(Attribute {
name: "puppet_disabled".to_string(),
typ: Boolean,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(true|false)\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "False".to_string(),
});vec.push(Attribute {
name: "harddisk_type".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(ssd|hdd|unknown)\\Z".to_string(),
readonly: false,
hovertext: "this is either hdd or ssd to give an indication on the performance and io scheduler to be used".to_string(),
required: true,
default: "ssd".to_string(),
});vec.push(Attribute {
name: "primary_ip6".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A[^\\.]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "rack_position".to_string(),
typ: Number,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A[0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "description".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "Intended to give an external auditor a very general idea of what the system is about.".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "arch".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "auto_update".to_string(),
typ: Boolean,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(true|false)\\Z".to_string(),
readonly: false,
hovertext: "This is used by automated scripts to decide if this server should be automatically updated in certain situations".to_string(),
required: true,
default: "1".to_string(),
});vec.push(Attribute {
name: "num_cpu".to_string(),
typ: Number,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(0|[1-9][0-9]*)\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "security_integrity".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(L|M|H|C)\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "puppet_environment".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(?!(testing|staging|production))[a-z][a-z0-9_]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "function".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "Function of host".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "graphite_graphs".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A[a-z][a-z0-9_\\.\\-]*\\Z".to_string(),
readonly: false,
hovertext: "a list of graphs to reference which graphs should be displayed on this server to be used in the collections (doku link).".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "puppet_ca".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "specifies the puppet ca server which the server should use".to_string(),
required: true,
default: "puppetca.innogames.de".to_string(),
});vec.push(Attribute {
name: "puppet_master".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "repositories".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A\\w+:\\w+:\\w+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "allow_to".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: true,
hovertext: "If set on any server in a segment it will block all access from servers in this segment to the internet unless they are explicitly given a value here.".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "legal_personal_data".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(nothing|personal|highly_personal)\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "memory_modules".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "assigned_to".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "route_network".to_string(),
typ: Supernet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: true,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "acl".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: true,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "responsible_admin".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A[a-z][a-z\\._]+[a-z]\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "apt_repository_url".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A([a-z0-9]+(-[a-z0-9]+)*\\.)+[a-z]{2,}\\Z".to_string(),
readonly: false,
hovertext: "Debian repository URL for apt e.g. update.innogames.de which will be used to form the entries in /etc/apt/sources.list together with repositories attribute.".to_string(),
required: false,
default: "aptly.innogames.de".to_string(),
});vec.push(Attribute {
name: "memory".to_string(),
typ: Number,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(0|[1-9][0-9]*)\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "sshfp".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A[1-4] (1 [0-9a-f]{40}|2 [0-9a-f]{64})\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "service_groups".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "disk_size_gib".to_string(),
typ: Number,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(0|[1-9][0-9]*)\\Z".to_string(),
readonly: false,
hovertext: "This is the current disk size of the server in gibibytes.  It should be filled while the VM is created or maintained by other scripts.\r\n".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "loadbalancer".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "A list of LB Pools to which this Node belongs.".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "hardware_model".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "ipmi".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "cpu_clock_mhz".to_string(),
typ: Number,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(0|[1-9][0-9]*)\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "puppet_classes".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A[a-z][a-z0-9_]*(::[a-z][a-z0-9_]*)*\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "monitoring_checks".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A[a-z][a-z0-9_\\-@]*\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "ping".to_string(),
});vec.push(Attribute {
name: "subproject".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A[a-z][a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "state".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(retired|maintenance|cold_standby|deploy_offline|deploy_online|online_reserved|online)\\Z".to_string(),
readonly: false,
hovertext: "Retired: No DNS, no Nagios, lb downtimed\r\nMaintenance: DNS, Nagios downtimed, lb downtimed\r\nDeploy_offline: DNS, Nagios active, lb downtimed\r\nDeploy_online: DNS, Nagios downtimed, lb active\r\nOnline: DNS, Nagios active, lb active".to_string(),
required: true,
default: "online".to_string(),
});vec.push(Attribute {
name: "environment".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(testing|staging|production)\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: true,
default: "production".to_string(),
});vec.push(Attribute {
name: "network_zone".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "A zone defining routing and firewall rules, a kind of sub-datacenter.".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "monitoring_queue".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "Set the gearman_queue for a route_network".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "network_zones".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "A zone defining routing and firewall rules, a kind of sub-datacenter. It is a multi attribute for \"network_zone\"".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "backup_storage".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "Storage server for the host's backups. This is normally auto-filled by the system while doing the first backup. Changes needs a puppet run on both the host and new storage.".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "firmware_versions".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A(BMC|BIOS|MIDPLANE)_[0-9a-z\\.]+\\Z".to_string(),
readonly: false,
hovertext: "Versions of various firmware components. See regexp for notation and supported attributes.".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "cpu_model".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "project".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "vlan_networks".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "monitoring_groups".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "datacenter_type".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(aws|digitalocean|hetzner|hyperv|kvm)\\.dct\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "storage_hardware".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A(raid|adaptec|threeware|perc700|perc710|perc730|perc310|perc330|perc200|sas1068|non-raid)\\Z".to_string(),
readonly: false,
hovertext: "The type of the raid controller if installed".to_string(),
required: true,
default: "raid".to_string(),
});vec.push(Attribute {
name: "backup_disabled".to_string(),
typ: Boolean,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(true|false)\\Z".to_string(),
readonly: false,
hovertext: "Set to true if you do not want this server backed up.".to_string(),
required: true,
default: "1".to_string(),
});vec.push(Attribute {
name: "backup_canshrink".to_string(),
typ: Boolean,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(true|false)\\Z".to_string(),
readonly: false,
hovertext: "Monitoring system will warn in case last backup size of the host drops below 50% of its last 10 backups' median size. Set this to true if you expect a shrinking backup (e.g. on \"speed worlds\" which are reset from time to time) so it doesn't complain on suddenly shrinking backup size.".to_string(),
required: false,
default: "0".to_string(),
});vec.push(Attribute {
name: "backup_maxage".to_string(),
typ: Number,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(0|[1-9][0-9]*)\\Z".to_string(),
readonly: false,
hovertext: "The expected freshness of the latest registered backup of the host. Defaults to 24 hours. If you have multiple backup windows, you might consider lowering this. 0 is a special value, disables backup age checking completely.".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "backup_window".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A\\d+-\\d+\\Z".to_string(),
readonly: false,
hovertext: "This is a list of hour ranges (in CEST) when the backup should be scheduled on the server locally. E.g. \"3-4\" means create one backup between 03:00 and 04:00. The backups will be transferred in order of backup_prio attribute once the local script ran through and reported a success so not guaranteed to be within the backup_window (but mostly does fit in).".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "os".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(android|cumulus|centos64|wheezy|jessie|stretch|buster|bullseye|bookworm|bionic|freebsd_[0-9]+\\.[0-9]+|macos_.*|tvos|other|esx|ciscoios|ios12.2|iDRAC6|windows(2022_std|2019_std|2019_dc|2016_std|2016_dc|2012r2_std|2012r2_dc|2012_std|2012_dc|2008r2_std|2008r2_dc|7|8|10|11))\\Z".to_string(),
readonly: false,
hovertext: "Operation system, e.g. wheezy, jessie or freebsd100 or macos_*".to_string(),
required: true,
default: "wheezy".to_string(),
});vec.push(Attribute {
name: "powerports".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "switchports".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "Switchports attribute is used by Foreman to configure switches. That includes creating proper LAGGs and assigning VLANs to them. ".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "mac".to_string(),
typ: Macaddr,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "ipv4".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty IPv4 address. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "ipv6".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty IPv6 address. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "hostname".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty hostname attribute. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "servertype".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty servertype attribute. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "object_id".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty object_id attribute. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
}); vec });
servertypes.insert("vpn".to_string(), { let mut vec = Vec::new(); vec.push(Attribute {
name: "backup_disabled".to_string(),
typ: Boolean,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(true|false)\\Z".to_string(),
readonly: false,
hovertext: "Set to true if you do not want this server backed up.".to_string(),
required: true,
default: "1".to_string(),
});vec.push(Attribute {
name: "backup_canshrink".to_string(),
typ: Boolean,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(true|false)\\Z".to_string(),
readonly: false,
hovertext: "Monitoring system will warn in case last backup size of the host drops below 50% of its last 10 backups' median size. Set this to true if you expect a shrinking backup (e.g. on \"speed worlds\" which are reset from time to time) so it doesn't complain on suddenly shrinking backup size.".to_string(),
required: false,
default: "0".to_string(),
});vec.push(Attribute {
name: "backup_maxage".to_string(),
typ: Number,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(0|[1-9][0-9]*)\\Z".to_string(),
readonly: false,
hovertext: "The expected freshness of the latest registered backup of the host. Defaults to 24 hours. If you have multiple backup windows, you might consider lowering this. 0 is a special value, disables backup age checking completely.".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "backup_window".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A\\d+-\\d+\\Z".to_string(),
readonly: false,
hovertext: "This is a list of hour ranges (in CEST) when the backup should be scheduled on the server locally. E.g. \"3-4\" means create one backup between 03:00 and 04:00. The backups will be transferred in order of backup_prio attribute once the local script ran through and reported a success so not guaranteed to be within the backup_window (but mostly does fit in).".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "backup_prio".to_string(),
typ: Number,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(0|[1-9][0-9]*)\\Z".to_string(),
readonly: false,
hovertext: "Backup priority of the host. Defaults to 300. Backups will be eventually processed regardless of the value of this, but higher number means the dump and transfer of the backup will be prioritized.".to_string(),
required: false,
default: "300".to_string(),
});vec.push(Attribute {
name: "vlan_networks".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "route_network".to_string(),
typ: Supernet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: true,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "auto_update".to_string(),
typ: Boolean,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(true|false)\\Z".to_string(),
readonly: false,
hovertext: "This is used by automated scripts to decide if this server should be automatically updated in certain situations".to_string(),
required: true,
default: "0".to_string(),
});vec.push(Attribute {
name: "puppet_disabled".to_string(),
typ: Boolean,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(true|false)\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "False".to_string(),
});vec.push(Attribute {
name: "primary_ip6".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A[^\\.]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "puppet_ca".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "specifies the puppet ca server which the server should use".to_string(),
required: false,
default: "puppetca.innogames.de".to_string(),
});vec.push(Attribute {
name: "puppet_master".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "routed_networks".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "domain".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A(\\*\\.)?([a-z0-9]+(\\.|-+))*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "list of all domains which are assigned to this server or load balancer".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "powerports".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "datacenter".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "allow_to".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: true,
hovertext: "If set on any server in a segment it will block all access from servers in this segment to the internet unless they are explicitly given a value here.".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "monitoring_checks".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A[a-z][a-z0-9_\\-@]*\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "ping".to_string(),
});vec.push(Attribute {
name: "function".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "Function of host".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "vlan_interfaces".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "no_monitoring".to_string(),
typ: Boolean,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(true|false)\\Z".to_string(),
readonly: false,
hovertext: "if set to true this server will be ignored by the nagios generation script".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "environment".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(testing|staging|production)\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: true,
default: "production".to_string(),
});vec.push(Attribute {
name: "rack".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "acl".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: true,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "responsible_admin".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A[a-z][a-z\\._]+[a-z]\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "monitoring_queue".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "Set the gearman_queue for a route_network".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "state".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(retired|maintenance|cold_standby|deploy_offline|deploy_online|online_reserved|online)\\Z".to_string(),
readonly: false,
hovertext: "Retired: No DNS, no Nagios, lb downtimed\r\nMaintenance: DNS, Nagios downtimed, lb downtimed\r\nDeploy_offline: DNS, Nagios active, lb downtimed\r\nDeploy_online: DNS, Nagios downtimed, lb active\r\nOnline: DNS, Nagios active, lb active".to_string(),
required: true,
default: "online".to_string(),
});vec.push(Attribute {
name: "service_groups".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "puppet_classes".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A[a-z][a-z0-9_]*(::[a-z][a-z0-9_]*)*\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "sshfp".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A[1-4] (1 [0-9a-f]{40}|2 [0-9a-f]{64})\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "backup_storage".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "Storage server for the host's backups. This is normally auto-filled by the system while doing the first backup. Changes needs a puppet run on both the host and new storage.".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "project".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: true,
default: "ndco".to_string(),
});vec.push(Attribute {
name: "protocol_port".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A((any|esp|ah)|((tcp|udp)([0-9]+([lt]+)?|[0-9]+(-[0-9]+)?(_[0-9]+)?))|((ping)|((tcp|udp)([0-9]+t?(-[0-9]+)?))))\\Z".to_string(),
readonly: true,
hovertext: "Ports and protocols to be forwarded to Nodes belonging to this LB Pool.".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "os".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(android|cumulus|centos64|wheezy|jessie|stretch|buster|bullseye|bookworm|bionic|freebsd_[0-9]+\\.[0-9]+|macos_.*|tvos|other|esx|ciscoios|ios12.2|iDRAC6|windows(2022_std|2019_std|2019_dc|2016_std|2016_dc|2012r2_std|2012r2_dc|2012_std|2012_dc|2008r2_std|2008r2_dc|7|8|10|11))\\Z".to_string(),
readonly: false,
hovertext: "Operation system, e.g. wheezy, jessie or freebsd100 or macos_*".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "repositories".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A\\w+:\\w+:\\w+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "puppet_environment".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(?!(testing|staging|production))[a-z][a-z0-9_]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "network_zones".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "A zone defining routing and firewall rules, a kind of sub-datacenter. It is a multi attribute for \"network_zone\"".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "graphite_graphs".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A[a-z][a-z0-9_\\.\\-]*\\Z".to_string(),
readonly: false,
hovertext: "a list of graphs to reference which graphs should be displayed on this server to be used in the collections (doku link).".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "num_cpu".to_string(),
typ: Number,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(0|[1-9][0-9]*)\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "memory".to_string(),
typ: Number,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(0|[1-9][0-9]*)\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "disk_size_gib".to_string(),
typ: Number,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(0|[1-9][0-9]*)\\Z".to_string(),
readonly: false,
hovertext: "This is the current disk size of the server in gibibytes.  It should be filled while the VM is created or maintained by other scripts.\r\n".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "harddisk_type".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(ssd|hdd|unknown)\\Z".to_string(),
readonly: false,
hovertext: "this is either hdd or ssd to give an indication on the performance and io scheduler to be used".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "subproject".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A[a-z][a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "mac".to_string(),
typ: Macaddr,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "network_zone".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "A zone defining routing and firewall rules, a kind of sub-datacenter.".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "ipmi".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "hardware_model".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "repositories_host".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A([a-z0-9]+(-[a-z0-9]+)*\\.)+[a-z]{2,}\\Z".to_string(),
readonly: false,
hovertext: "Repositories hostname to fetch packages from. On Debian this is where apt fetches debian packages from (aptly). For FreeBSD this is where pkg fetches FreeBSD packages from.\r\nIt is used to form the correct configs on the hosts.".to_string(),
required: false,
default: "aptly.innogames.de".to_string(),
});vec.push(Attribute {
name: "monitoring_groups".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "ipv4".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty IPv4 address. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "ipv6".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty IPv6 address. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "hostname".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty hostname attribute. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "servertype".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty servertype attribute. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "object_id".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty object_id attribute. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
}); vec });
servertypes.insert("provider_domain".to_string(), { let mut vec = Vec::new(); vec.push(Attribute {
name: "mx".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "state".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(retired|maintenance|cold_standby|deploy_offline|deploy_online|online_reserved|online)\\Z".to_string(),
readonly: false,
hovertext: "Retired: No DNS, no Nagios, lb downtimed\r\nMaintenance: DNS, Nagios downtimed, lb downtimed\r\nDeploy_offline: DNS, Nagios active, lb downtimed\r\nDeploy_online: DNS, Nagios downtimed, lb active\r\nOnline: DNS, Nagios active, lb active".to_string(),
required: true,
default: "online".to_string(),
});vec.push(Attribute {
name: "hidden_primary".to_string(),
typ: Boolean,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(true|false)\\Z".to_string(),
readonly: false,
hovertext: "If true the domain is served by one of our external DNS providers (e.g. INWX) to the world and our Nameserver act as hidden master.".to_string(),
required: false,
default: "false".to_string(),
});vec.push(Attribute {
name: "dns_soa".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A([a-zA-Z0-9-.]+\\.)\\s+([a-zA-Z0-9-.]+\\.)\\s+(\\d+)\\s+(\\d+)\\s+(\\d+)\\s+(\\d+)\\s+(\\d+)\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "hostname".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty hostname attribute. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "servertype".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty servertype attribute. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "object_id".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty object_id attribute. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
}); vec });
servertypes.insert("lb_pool".to_string(), { let mut vec = Vec::new(); vec.push(Attribute {
name: "acl".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: true,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "allow_from".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: true,
hovertext: "List of Access lists from which access to this LB Pool is allowed. Connections from outside of allowed networks will be silently dropped.".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "domain".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A(\\*\\.)?([a-z0-9]+(\\.|-+))*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "list of all domains which are assigned to this server or load balancer".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "environment".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(testing|staging|production)\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: true,
default: "production".to_string(),
});vec.push(Attribute {
name: "function".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "Function of host".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "game_market".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A[a-z][a-z]|foe|war\\Z".to_string(),
readonly: false,
hovertext: "The 2 letter short code for the game market, see docu for allowed values.".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "game_type".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "Currently only used to mark Tribalwars Speed (s), Classic (c), and Casual (p) special worlds".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "game_world".to_string(),
typ: Number,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(0|[1-9][0-9]*)\\Z".to_string(),
readonly: false,
hovertext: "The numerical part of the world id for games 0 is reserved for master or servers with market wide functionality".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "monitoring_checks".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A[a-z][a-z0-9_\\-@]*\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "monitoring_queue".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "Set the gearman_queue for a route_network".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "monitoring_queues".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "gearman_queues a worker should be responsible for".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "no_monitoring".to_string(),
typ: Boolean,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(true|false)\\Z".to_string(),
readonly: false,
hovertext: "if set to true this server will be ignored by the nagios generation script".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "project".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "protocol_port".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A((any|esp|ah)|((tcp|udp)([0-9]+([lt]+)?|[0-9]+(-[0-9]+)?(_[0-9]+)?))|((ping)|((tcp|udp)([0-9]+t?(-[0-9]+)?))))\\Z".to_string(),
readonly: true,
hovertext: "Ports and protocols to be forwarded to Nodes belonging to this LB Pool.".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "responsible_admin".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A[a-z][a-z\\._]+[a-z]\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "service_groups".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "state".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(retired|maintenance|cold_standby|deploy_offline|deploy_online|online_reserved|online)\\Z".to_string(),
readonly: false,
hovertext: "Retired: No DNS, no Nagios, lb downtimed\r\nMaintenance: DNS, Nagios downtimed, lb downtimed\r\nDeploy_offline: DNS, Nagios active, lb downtimed\r\nDeploy_online: DNS, Nagios downtimed, lb active\r\nOnline: DNS, Nagios active, lb active".to_string(),
required: true,
default: "online".to_string(),
});vec.push(Attribute {
name: "subproject".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A[a-z][a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "vlan_interfaces".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "lb_pool_nodes".to_string(),
typ: Relation,
reversed_attribute_id: "lb_pool".to_string(),
multi: true,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: true,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "aws_placement".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A(ap|ca|eu|me|sa|us)\\-(central|east|north|northeast|south|west)\\-[1-9][a-z]\\Z".to_string(),
readonly: false,
hovertext: "AWS Placement".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "datacenter".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "health_checks".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "hostname".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty hostname attribute. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "servertype".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty servertype attribute. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "object_id".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty object_id attribute. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
}); vec });
servertypes.insert("bond_interface".to_string(), { let mut vec = Vec::new(); vec.push(Attribute {
name: "bond_no".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "network_type".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(loopback|transit_(internal_igp|public_(hwlbs|uplink|igp))|public|internal|public_dmz|vpn_clients|internal_vpn_clients|cloud)\\Z".to_string(),
readonly: false,
hovertext: "Select what kind of connection you want to switch\r\nsingle: single interface without lacp or anything other special\r\nbond_ab: Bonding without LACP but 2 links with failover\r\nbond_lacp: bonding with lacp and 2 active links".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "project".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: true,
default: "ndco".to_string(),
});vec.push(Attribute {
name: "hostname".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty hostname attribute. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "servertype".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty servertype attribute. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "object_id".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty object_id attribute. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
}); vec });
servertypes.insert("floating_address".to_string(), { let mut vec = Vec::new(); vec.push(Attribute {
name: "environment".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(testing|staging|production)\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: true,
default: "production".to_string(),
});vec.push(Attribute {
name: "project_network".to_string(),
typ: Supernet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: true,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "no_monitoring".to_string(),
typ: Boolean,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(true|false)\\Z".to_string(),
readonly: false,
hovertext: "if set to true this server will be ignored by the nagios generation script".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "route_network".to_string(),
typ: Supernet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: true,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "function".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "Function of host".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "sshfp".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A[1-4] (1 [0-9a-f]{40}|2 [0-9a-f]{64})\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "monitoring_groups".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "subproject".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A[a-z][a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "state".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(retired|maintenance|cold_standby|deploy_offline|deploy_online|online_reserved|online)\\Z".to_string(),
readonly: false,
hovertext: "Retired: No DNS, no Nagios, lb downtimed\r\nMaintenance: DNS, Nagios downtimed, lb downtimed\r\nDeploy_offline: DNS, Nagios active, lb downtimed\r\nDeploy_online: DNS, Nagios downtimed, lb active\r\nOnline: DNS, Nagios active, lb active".to_string(),
required: true,
default: "online".to_string(),
});vec.push(Attribute {
name: "acl".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: true,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "allow_from".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: true,
hovertext: "List of Access lists from which access to this LB Pool is allowed. Connections from outside of allowed networks will be silently dropped.".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "primary_ip6".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A[^\\.]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "monitoring_checks".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A[a-z][a-z0-9_\\-@]*\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "ping".to_string(),
});vec.push(Attribute {
name: "responsible_admin".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A[a-z][a-z\\._]+[a-z]\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "datacenter".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "monitoring_queue".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "Set the gearman_queue for a route_network".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "project".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "service_groups".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "ipv4".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty IPv4 address. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "ipv6".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty IPv6 address. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "hostname".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty hostname attribute. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "servertype".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty servertype attribute. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "object_id".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty object_id attribute. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
}); vec });
servertypes.insert("vm_external".to_string(), { let mut vec = Vec::new(); vec.push(Attribute {
name: "loadbalancer".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "A list of LB Pools to which this Node belongs.".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "security_availability".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(L|M|H|C)\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "os".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(android|cumulus|centos64|wheezy|jessie|stretch|buster|bullseye|bookworm|bionic|freebsd_[0-9]+\\.[0-9]+|macos_.*|tvos|other|esx|ciscoios|ios12.2|iDRAC6|windows(2022_std|2019_std|2019_dc|2016_std|2016_dc|2012r2_std|2012r2_dc|2012_std|2012_dc|2008r2_std|2008r2_dc|7|8|10|11))\\Z".to_string(),
readonly: false,
hovertext: "Operation system, e.g. wheezy, jessie or freebsd100 or macos_*".to_string(),
required: false,
default: "buster".to_string(),
});vec.push(Attribute {
name: "puppet_ca".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "specifies the puppet ca server which the server should use".to_string(),
required: true,
default: "puppetca.innogames.de".to_string(),
});vec.push(Attribute {
name: "puppet_master".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "group_company".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A[a-zA-Z]+\\Z".to_string(),
readonly: false,
hovertext: "Company this host is serving e.g. Hutch, Congregate (Default: InnoGames)".to_string(),
required: false,
default: "InnoGames".to_string(),
});vec.push(Attribute {
name: "game_market".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A[a-z][a-z]|foe|war\\Z".to_string(),
readonly: false,
hovertext: "The 2 letter short code for the game market, see docu for allowed values.".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "sshfp".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A[1-4] (1 [0-9a-f]{40}|2 [0-9a-f]{64})\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "auto_update".to_string(),
typ: Boolean,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(true|false)\\Z".to_string(),
readonly: false,
hovertext: "This is used by automated scripts to decide if this server should be automatically updated in certain situations".to_string(),
required: false,
default: "True".to_string(),
});vec.push(Attribute {
name: "game_world".to_string(),
typ: Number,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(0|[1-9][0-9]*)\\Z".to_string(),
readonly: false,
hovertext: "The numerical part of the world id for games 0 is reserved for master or servers with market wide functionality".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "backup_disabled".to_string(),
typ: Boolean,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(true|false)\\Z".to_string(),
readonly: false,
hovertext: "Set to true if you do not want this server backed up.".to_string(),
required: false,
default: "False".to_string(),
});vec.push(Attribute {
name: "backup_canshrink".to_string(),
typ: Boolean,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(true|false)\\Z".to_string(),
readonly: false,
hovertext: "Monitoring system will warn in case last backup size of the host drops below 50% of its last 10 backups' median size. Set this to true if you expect a shrinking backup (e.g. on \"speed worlds\" which are reset from time to time) so it doesn't complain on suddenly shrinking backup size.".to_string(),
required: false,
default: "False".to_string(),
});vec.push(Attribute {
name: "backup_maxage".to_string(),
typ: Number,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(0|[1-9][0-9]*)\\Z".to_string(),
readonly: false,
hovertext: "The expected freshness of the latest registered backup of the host. Defaults to 24 hours. If you have multiple backup windows, you might consider lowering this. 0 is a special value, disables backup age checking completely.".to_string(),
required: false,
default: "36".to_string(),
});vec.push(Attribute {
name: "backup_prio".to_string(),
typ: Number,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(0|[1-9][0-9]*)\\Z".to_string(),
readonly: false,
hovertext: "Backup priority of the host. Defaults to 300. Backups will be eventually processed regardless of the value of this, but higher number means the dump and transfer of the backup will be prioritized.".to_string(),
required: false,
default: "300".to_string(),
});vec.push(Attribute {
name: "num_cpu".to_string(),
typ: Number,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(0|[1-9][0-9]*)\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: true,
default: "8".to_string(),
});vec.push(Attribute {
name: "no_monitoring".to_string(),
typ: Boolean,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(true|false)\\Z".to_string(),
readonly: false,
hovertext: "if set to true this server will be ignored by the nagios generation script".to_string(),
required: false,
default: "False".to_string(),
});vec.push(Attribute {
name: "memory".to_string(),
typ: Number,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(0|[1-9][0-9]*)\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "puppet_disabled".to_string(),
typ: Boolean,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(true|false)\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "False".to_string(),
});vec.push(Attribute {
name: "security_confidentiality".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(L|M|H|C)\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "primary_ip6".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A[^\\.]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "disk_size_gib".to_string(),
typ: Number,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(0|[1-9][0-9]*)\\Z".to_string(),
readonly: false,
hovertext: "This is the current disk size of the server in gibibytes.  It should be filled while the VM is created or maintained by other scripts.\r\n".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "graphite_graphs".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A[a-z][a-z0-9_\\.\\-]*\\Z".to_string(),
readonly: false,
hovertext: "a list of graphs to reference which graphs should be displayed on this server to be used in the collections (doku link).".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "security_integrity".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(L|M|H|C)\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "legal_personal_data".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(nothing|personal|highly_personal)\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "description".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "Intended to give an external auditor a very general idea of what the system is about.".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "function".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "Function of host".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "monitoring_queues".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "gearman_queues a worker should be responsible for".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "puppet_environment".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(?!(testing|staging|production))[a-z][a-z0-9_]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "backup_window".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A\\d+-\\d+\\Z".to_string(),
readonly: false,
hovertext: "This is a list of hour ranges (in CEST) when the backup should be scheduled on the server locally. E.g. \"3-4\" means create one backup between 03:00 and 04:00. The backups will be transferred in order of backup_prio attribute once the local script ran through and reported a success so not guaranteed to be within the backup_window (but mostly does fit in).".to_string(),
required: false,
default: "2-5".to_string(),
});vec.push(Attribute {
name: "repositories".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A\\w+:\\w+:\\w+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "int:innogames:stable".to_string(),
});vec.push(Attribute {
name: "domain".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A(\\*\\.)?([a-z0-9]+(\\.|-+))*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "list of all domains which are assigned to this server or load balancer".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "responsible_admin".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A[a-z][a-z\\._]+[a-z]\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "route_network".to_string(),
typ: Supernet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: true,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "acl".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: true,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "subproject".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A[a-z][a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "network_zones".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "A zone defining routing and firewall rules, a kind of sub-datacenter. It is a multi attribute for \"network_zone\"".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "state".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(retired|maintenance|cold_standby|deploy_offline|deploy_online|online_reserved|online)\\Z".to_string(),
readonly: false,
hovertext: "Retired: No DNS, no Nagios, lb downtimed\r\nMaintenance: DNS, Nagios downtimed, lb downtimed\r\nDeploy_offline: DNS, Nagios active, lb downtimed\r\nDeploy_online: DNS, Nagios downtimed, lb active\r\nOnline: DNS, Nagios active, lb active".to_string(),
required: true,
default: "online".to_string(),
});vec.push(Attribute {
name: "network_type".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(loopback|transit_(internal_igp|public_(hwlbs|uplink|igp))|public|internal|public_dmz|vpn_clients|internal_vpn_clients|cloud)\\Z".to_string(),
readonly: false,
hovertext: "Select what kind of connection you want to switch\r\nsingle: single interface without lacp or anything other special\r\nbond_ab: Bonding without LACP but 2 links with failover\r\nbond_lacp: bonding with lacp and 2 active links".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "monitoring_checks".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A[a-z][a-z0-9_\\-@]*\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "ping".to_string(),
});vec.push(Attribute {
name: "puppet_classes".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A[a-z][a-z0-9_]*(::[a-z][a-z0-9_]*)*\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "provider_network".to_string(),
typ: Supernet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: true,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "monitoring_satellite_of".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "icinga2".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "environment".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(testing|staging|production)\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: true,
default: "production".to_string(),
});vec.push(Attribute {
name: "datacenter".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "network_zone".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "A zone defining routing and firewall rules, a kind of sub-datacenter.".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "instance".to_string(),
typ: Number,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A[0-9]+\\Z".to_string(),
readonly: false,
hovertext: "used to index multiple instances of the same function".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "service_groups".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "monitoring_queue".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "Set the gearman_queue for a route_network".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "backup_storage".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "Storage server for the host's backups. This is normally auto-filled by the system while doing the first backup. Changes needs a puppet run on both the host and new storage.".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "project".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "vlan_networks".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "restic_keep_monthly".to_string(),
typ: Number,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A[1-9][0-9]?$|^100\\Z".to_string(),
readonly: false,
hovertext: "Monthly backup count to keep for this host, 1-100".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "restic_keep_yearly".to_string(),
typ: Number,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A[0-9]$|^10\\Z".to_string(),
readonly: false,
hovertext: "Yearly backup count to keep for this host, 1-10".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "datacenter_type".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(aws|digitalocean|hetzner|hyperv|kvm)\\.dct\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "restic_group".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A[a-z0-9][a-z0-9-]+[a-z0-9]\\Z".to_string(),
readonly: false,
hovertext: "Name of the backup bucket of this server, project (+ \"-subproject\" if defined) when empty by default".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "restic_keep_daily".to_string(),
typ: Number,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A[1-9][0-9]?$|^100\\Z".to_string(),
readonly: false,
hovertext: "Daily backup count to keep for this host, 1-100".to_string(),
required: false,
default: "3".to_string(),
});vec.push(Attribute {
name: "restic_keep_last".to_string(),
typ: Number,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A([1-3][0-9][0-9]|[0-9][0-9]|[0-9])\\Z".to_string(),
readonly: false,
hovertext: "Latest backup count to keep for this host, 1-399".to_string(),
required: false,
default: "3".to_string(),
});vec.push(Attribute {
name: "restic_keep_weekly".to_string(),
typ: Number,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A[1-9][0-9]?$|^100\\Z".to_string(),
readonly: false,
hovertext: "Weekly backup count to keep for this host, 1-100".to_string(),
required: false,
default: "2".to_string(),
});vec.push(Attribute {
name: "load_99".to_string(),
typ: Number,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(-?[0-9]+)\\Z".to_string(),
readonly: true,
hovertext: "".to_string(),
required: false,
default: "-1".to_string(),
});vec.push(Attribute {
name: "repositories_host".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A([a-z0-9]+(-[a-z0-9]+)*\\.)+[a-z]{2,}\\Z".to_string(),
readonly: false,
hovertext: "Repositories hostname to fetch packages from. On Debian this is where apt fetches debian packages from (aptly). For FreeBSD this is where pkg fetches FreeBSD packages from.\r\nIt is used to form the correct configs on the hosts.".to_string(),
required: false,
default: "aptly.innogames.de".to_string(),
});vec.push(Attribute {
name: "monitoring_groups".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
});vec.push(Attribute {
name: "ipv4".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty IPv4 address. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "ipv6".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty IPv6 address. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "hostname".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty hostname attribute. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "servertype".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty servertype attribute. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
});vec.push(Attribute {
name: "object_id".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty object_id attribute. Comes from somewhere, good luck finding it!".to_string(),
required: true,
default: "".to_string(),
}); vec });

        servertypes
}

pub  fn attributes() -> Vec<Attribute> {
    use AttributeType::*;

    vec![Attribute {
name: "restic_keep_yearly".to_string(),
typ: Number,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A[0-9]$|^10\\Z".to_string(),
readonly: false,
hovertext: "Yearly backup count to keep for this host, 1-10".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "cpu_util_pct_tmp".to_string(),
typ: Number,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(0|[1-9]{1,2})\\Z".to_string(),
readonly: true,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "hc_drain_codes".to_string(),
typ: Number,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A(0|[1-9][0-9]*)\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "gateway_type".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "assigned_to".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "max_nodes".to_string(),
typ: Number,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(0|[1-9][0-9]*)\\Z".to_string(),
readonly: false,
hovertext: "Maximum number of nodes serving pool's traffic even if more are alive.".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "protocol_ports_inbound".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A((esp|ah|ping)|((tcp|udp)([0-9]+)(-[0-9]+)?([lt]+)?))\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "restic_keep_last".to_string(),
typ: Number,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A([1-3][0-9][0-9]|[0-9][0-9]|[0-9])\\Z".to_string(),
readonly: false,
hovertext: "Latest backup count to keep for this host, 1-399".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "aws_placement".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A(ap|ca|eu|me|sa|us)\\-(central|east|north|northeast|south|west)\\-[1-9][a-z]\\Z".to_string(),
readonly: false,
hovertext: "AWS Placement".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "restic_keep_weekly".to_string(),
typ: Number,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A[1-9][0-9]?$|^100\\Z".to_string(),
readonly: false,
hovertext: "Weekly backup count to keep for this host, 1-100".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "dns_ns".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A(?:(?:[a-z01-9\\-]*[a-z01-9]\\.)+(?:[a-z]+))\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "firmware_versions".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A(BMC|BIOS|MIDPLANE)_[0-9a-z\\.]+\\Z".to_string(),
readonly: false,
hovertext: "Versions of various firmware components. See regexp for notation and supported attributes.".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "provider".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "backup_maxage".to_string(),
typ: Number,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(0|[1-9][0-9]*)\\Z".to_string(),
readonly: false,
hovertext: "The expected freshness of the latest registered backup of the host. Defaults to 24 hours. If you have multiple backup windows, you might consider lowering this. 0 is a special value, disables backup age checking completely.".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "hc_max_failed".to_string(),
typ: Number,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(0|[1-9][0-9]*)\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "loadbalancer".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "A list of LB Pools to which this Node belongs.".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "auto_update".to_string(),
typ: Boolean,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(true|false)\\Z".to_string(),
readonly: false,
hovertext: "This is used by automated scripts to decide if this server should be automatically updated in certain situations".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "game_type".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "Currently only used to mark Tribalwars Speed (s), Classic (c), and Casual (p) special worlds".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "allow_to_reverse".to_string(),
typ: Relation,
reversed_attribute_id: "allow_to".to_string(),
multi: true,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: true,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "serial_number".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A[a-zA-Z0-9\\-]+\\Z".to_string(),
readonly: false,
hovertext: "A unique string identifying hardware, like:\r\n- Dell ServiceTag\r\n- Memory module serial number\r\n- Disk WWN address".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "snat_lb_pool".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "LB Pool used for providing default SNAT for IPv4.".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "arch".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "security_availability".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(L|M|H|C)\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "aws_instance_type".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A[acimrptxz][0-9]+[a|d|n|e]?\\.[0-9]*x?(large|micro)\\Z".to_string(),
readonly: false,
hovertext: "AWS InstanceType".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "officetool_id".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\Ai?[0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "no_logging".to_string(),
typ: Boolean,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(true|false)\\Z".to_string(),
readonly: false,
hovertext: "To disable logging of traffic on firewall".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "allow_from".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: true,
hovertext: "List of Access lists from which access to this LB Pool is allowed. Connections from outside of allowed networks will be silently dropped.".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "load_99".to_string(),
typ: Number,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(-?[0-9]+)\\Z".to_string(),
readonly: true,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "object_id".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty object_id attribute. Comes from somewhere, good luck finding it!".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "network_link_type".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(kernel|ovs)_(single|bond_lacp|bond_ab)\\Z".to_string(),
readonly: false,
hovertext: "The type of physical connection between the hardware server and switches. The first part either 'ovs' if you use OpenVSwitch or 'kernel' if it's just plain Linux/Windows. Second part is either 'single' for devices with a single network cable or 'bond_ab' or 'bond_lacp' for servers with redundant network link.".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "legal_personal_data".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(nothing|personal|highly_personal)\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "hardware_model".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "protocol_ports_outbound".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A((esp|ah|ping)|((tcp|udp)([0-9]+)(-[0-9]+)?([lt]+)?))\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "igvm_locked".to_string(),
typ: Datetime,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "Set when object is being manipulated by igvm, e.g. being build or migrated.".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "monitoring_queue".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "Set the gearman_queue for a route_network".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "barman_host".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "installed_in".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A[a-z0-9\\-\\.]+\\Z".to_string(),
readonly: false,
hovertext: "A physical device in which the hardware component is installed.".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "dhcp_networks".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "security_confidentiality".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(L|M|H|C)\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "rack".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "dell_servicetag".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "The Service Tag for Dell to be used in the support interface and to identify the hardware".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "service_group_members".to_string(),
typ: Relation,
reversed_attribute_id: "service_groups".to_string(),
multi: true,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: true,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "aws_image_id".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(ami-)([a-z0-9]+)\\Z".to_string(),
readonly: false,
hovertext: "AWS ImageID".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "backup_window".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A\\d+-\\d+\\Z".to_string(),
readonly: false,
hovertext: "This is a list of hour ranges (in CEST) when the backup should be scheduled on the server locally. E.g. \"3-4\" means create one backup between 03:00 and 04:00. The backups will be transferred in order of backup_prio attribute once the local script ran through and reported a success so not guaranteed to be within the backup_window (but mostly does fit in).".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "igvm_action".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(build|migrate|migrate_offline|in_progress|successful|failed)\\Z".to_string(),
readonly: false,
hovertext: "Current state of VM for IGVM daemon".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "primary_ip6".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A[^\\.]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "order_date".to_string(),
typ: Date,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "When this server was ordered, for warranty and to find the order".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "memory_modules".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "bond_no".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "cpu_perffactor".to_string(),
typ: Number,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A[0-9]+\\Z".to_string(),
readonly: true,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "aws_instance_id".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\Ai-[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "AWS InstanceId".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "datacenter".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "internal_gateway".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "servertype".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty servertype attribute. Comes from somewhere, good luck finding it!".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "ipv4".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty IPv4 address. Comes from somewhere, good luck finding it!".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "provider_asn".to_string(),
typ: Number,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A[0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "game_world".to_string(),
typ: Number,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(0|[1-9][0-9]*)\\Z".to_string(),
readonly: false,
hovertext: "The numerical part of the world id for games 0 is reserved for master or servers with market wide functionality".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "iggop_locked".to_string(),
typ: Boolean,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(true|false)\\Z".to_string(),
readonly: false,
hovertext: "The object or a related object is worked on by iggop and should not be used.".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "sg_allow_to_reverse".to_string(),
typ: Relation,
reversed_attribute_id: "sg_allow_to".to_string(),
multi: true,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: true,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "num_cpu".to_string(),
typ: Number,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(0|[1-9][0-9]*)\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "letsencrypt_enabled".to_string(),
typ: Boolean,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(true|false)\\Z".to_string(),
readonly: false,
hovertext: "If you enable this for a public domain, our helper tool will try to generate a letsencrypt certificate for this domain and put it into puppet/acmekeys repo (& keep it up-to-date).".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "puppet_ca".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "specifies the puppet ca server which the server should use".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "dns_soa".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A([a-zA-Z0-9-.]+\\.)\\s+([a-zA-Z0-9-.]+\\.)\\s+(\\d+)\\s+(\\d+)\\s+(\\d+)\\s+(\\d+)\\s+(\\d+)\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "aws_subnet_id".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\Asubnet-[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "AWS SubnetId".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "routed_networks".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "hypervisor_standby".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "Hostname of the Hypervisor on which hot copy of this VM resides".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "vlan_interface_of".to_string(),
typ: Relation,
reversed_attribute_id: "vlan_interfaces".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: true,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "hardware_components".to_string(),
typ: Relation,
reversed_attribute_id: "installed_in".to_string(),
multi: true,
regexp: "\\A.*\\Z".to_string(),
readonly: true,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "lb_pool_nodes".to_string(),
typ: Relation,
reversed_attribute_id: "lb_pool".to_string(),
multi: true,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: true,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "vlan_networks".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "slack_channel".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(#[a-z0-9_-]+)\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "lb_multihoming".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(primary|secondary|noauto)\\Z".to_string(),
readonly: false,
hovertext: "Decide which datacenter is to be used as primary one for LB Pool of this LB Node.".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "vlan_interfaces".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "ipv4_only".to_string(),
typ: Boolean,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(true|false)\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "backup_canshrink".to_string(),
typ: Boolean,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(true|false)\\Z".to_string(),
readonly: false,
hovertext: "Monitoring system will warn in case last backup size of the host drops below 50% of its last 10 backups' median size. Set this to true if you expect a shrinking backup (e.g. on \"speed worlds\" which are reset from time to time) so it doesn't complain on suddenly shrinking backup size.".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "service_groups".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "rack_position".to_string(),
typ: Number,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A[0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "network_zones".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "A zone defining routing and firewall rules, a kind of sub-datacenter. It is a multi attribute for \"network_zone\"".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "min_nodes".to_string(),
typ: Number,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(0|[1-9][0-9]*)\\Z".to_string(),
readonly: false,
hovertext: "Minimum number of alive hosts required. If number goes below this value, min_nodes_action is checked".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "network_zone".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "A zone defining routing and firewall rules, a kind of sub-datacenter.".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "switchports".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "Switchports attribute is used by Foreman to configure switches. That includes creating proper LAGGs and assigning VLANs to them. ".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "jira_project".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A([A-Z]+)\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "network_autoconfiguration".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(none|dhcp_static|dhcp_dynamic)\\Z".to_string(),
readonly: false,
hovertext: "Configure automatic assignment of IP addresses of devices in this network:\r\n- none - no RAs, no DHCP\r\n- dhcp_static - no SLAAC due to RAs send without the \"auto\" flag, DHCPv4 and DHCPv6 only with entries from Serveradmin\r\n- dhcp_dynamic - SLAAC due to RAs send with the \"auto\" flag, DHCPv4 and DHCPv6 with entries from Serveradmin and from dynamic pool".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "io_weight".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(default|low)\\Z".to_string(),
readonly: false,
hovertext: "Defines the I/O weight for a VM, setting this to low means other VMs on the same Hypervisor will get preferred I/O.".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "protocol_port".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A((any|esp|ah)|((tcp|udp)([0-9]+([lt]+)?|[0-9]+(-[0-9]+)?(_[0-9]+)?))|((ping)|((tcp|udp)([0-9]+t?(-[0-9]+)?))))\\Z".to_string(),
readonly: true,
hovertext: "Ports and protocols to be forwarded to Nodes belonging to this LB Pool.".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "dns_txt".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "repositories".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A\\w+:\\w+:\\w+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "project_domain".to_string(),
typ: Domain,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: true,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "aws_vpc_id".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\Avpc-[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "Aws VpcId".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "backup_storage".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "Storage server for the host's backups. This is normally auto-filled by the system while doing the first backup. Changes needs a puppet run on both the host and new storage.".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "cpu_model".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "mx".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "storage_hardware".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A(raid|adaptec|threeware|perc700|perc710|perc730|perc310|perc330|perc200|sas1068|non-raid)\\Z".to_string(),
readonly: false,
hovertext: "The type of the raid controller if installed".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "graphite_graphs".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A[a-z][a-z0-9_\\.\\-]*\\Z".to_string(),
readonly: false,
hovertext: "a list of graphs to reference which graphs should be displayed on this server to be used in the collections (doku link).".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "provider_domain".to_string(),
typ: Domain,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: true,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "hc_port".to_string(),
typ: Number,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(0|[1-9][0-9]*)\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "hc_timeout".to_string(),
typ: Number,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(0|[1-9][0-9]*)\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "cpu_util_vm_pct".to_string(),
typ: Number,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(0|[1-9][0-9]*)\\Z".to_string(),
readonly: true,
hovertext: "Show the maximum vCPU usage of the last 7 days".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "restic_keep_daily".to_string(),
typ: Number,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A[1-9][0-9]?$|^100\\Z".to_string(),
readonly: false,
hovertext: "Daily backup count to keep for this host, 1-100".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "dns_srv".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A\\d+\\s\\d+\\s\\d+\\s[a-zA-Z0-9.-]+\\Z".to_string(),
readonly: false,
hovertext: "SRV DNS record".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "hardware_type".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "mandatory field on all objects such as amp, beamer, recorder etc.".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "rack_height".to_string(),
typ: Number,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A[0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "monitoring_workers".to_string(),
typ: Relation,
reversed_attribute_id: "monitoring_queues".to_string(),
multi: true,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: true,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "hc_query".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "provider_network".to_string(),
typ: Supernet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: true,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "libvirt_pool_total_gib".to_string(),
typ: Number,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(0|[1-9][0-9]*)\\Z".to_string(),
readonly: true,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "backup_pool".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "The name of another LB Pool to which the traffic for this one should be redirected once there is no active node left.".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "fixup_enabled".to_string(),
typ: Boolean,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(true|false)\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "lb_pool".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "A list of LB Pools to which this Node belongs.".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "hw_loadbalancer_group".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "group_company".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A[a-zA-Z]+\\Z".to_string(),
readonly: false,
hovertext: "Company this host is serving e.g. Hutch, Congregate (Default: InnoGames)".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "internal_gateway_vhid".to_string(),
typ: Number,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(0|[1-9][0-9]*)\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "health_checks".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "memory".to_string(),
typ: Number,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(0|[1-9][0-9]*)\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "rack_colo".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "description".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "Intended to give an external auditor a very general idea of what the system is about.".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "monitoring_checks".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A[a-z][a-z0-9_\\-@]*\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "sg_allow_from_reverse".to_string(),
typ: Relation,
reversed_attribute_id: "sg_allow_from".to_string(),
multi: true,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: true,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "datacenter_type".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(aws|digitalocean|hetzner|hyperv|kvm)\\.dct\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "aws_security_group_ids".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\Asg-[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "AWS SecurityGroupIds".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "symmetric_nat".to_string(),
typ: Boolean,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(true|false)\\Z".to_string(),
readonly: false,
hovertext: "This attribute will create a rule causing any Nodes belonging to this Pool to use its IP address for outgoing NAT.".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "hc_interval".to_string(),
typ: Number,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(0|[1-9][0-9]*)\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "acl".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: true,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "puppet_classes".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A[a-z][a-z0-9_]*(::[a-z][a-z0-9_]*)*\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "libvirt_memory_used_gib".to_string(),
typ: Number,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(0|[1-9][0-9]*)\\Z".to_string(),
readonly: true,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "served_game".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "Like game_market used to group (backend) servers for one/multiple game(s)".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "libvirt_memory_total_gib".to_string(),
typ: Number,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(0|[1-9][0-9]*)\\Z".to_string(),
readonly: true,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "support_code".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A([a-z0-9]+)\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "allow_to".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: true,
hovertext: "If set on any server in a segment it will block all access from servers in this segment to the internet unless they are explicitly given a value here.".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "ipmi_of".to_string(),
typ: Relation,
reversed_attribute_id: "ipmi".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: true,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "hidden_primary".to_string(),
typ: Boolean,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(true|false)\\Z".to_string(),
readonly: false,
hovertext: "If true the domain is served by one of our external DNS providers (e.g. INWX) to the world and our Nameserver act as hidden master.".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "backup_pool_of".to_string(),
typ: Relation,
reversed_attribute_id: "backup_pool".to_string(),
multi: true,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: true,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "project".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "libvirt_memory_free_gib".to_string(),
typ: Number,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(0|[1-9][0-9]*)\\Z".to_string(),
readonly: true,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "function".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "Function of host".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "hypervisor".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "Hostname of the Hypervisor on which this VM resides".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "security_integrity".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(L|M|H|C)\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "default_gateway".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "monitoring_groups".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "memory_usage_gib".to_string(),
typ: Number,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(0|[1-9][0-9]*)\\Z".to_string(),
readonly: true,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "nessus_scan_id".to_string(),
typ: Number,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A[0-9]+\\Z".to_string(),
readonly: false,
hovertext: "The id of the scan in nessus".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "tunnel_interfaces".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "powa_enabled".to_string(),
typ: Boolean,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(true|false)\\Z".to_string(),
readonly: false,
hovertext: "True if Powa is enabled on the host. This attribute is managed by puppet, don't set it manually.".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "dns_spf".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A.+\\Z".to_string(),
readonly: false,
hovertext: "SPF DNS record:\r\nprio, weight, port, target\r\n\r\nExample:\r\n10 0 5269 jabber.innogames.de".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "subproject".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A[a-z][a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "domain".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A(\\*\\.)?([a-z0-9]+(\\.|-+))*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "list of all domains which are assigned to this server or load balancer".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "restic_keep_monthly".to_string(),
typ: Number,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A[1-9][0-9]?$|^100\\Z".to_string(),
readonly: false,
hovertext: "Monthly backup count to keep for this host, 1-100".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "lb_nodes".to_string(),
typ: Relation,
reversed_attribute_id: "loadbalancer".to_string(),
multi: true,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: true,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "capacity".to_string(),
typ: Number,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A[0-9]+\\Z".to_string(),
readonly: false,
hovertext: "Capacity of a memory or a storage module. Measured in B.".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "sshfp".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A[1-4] (1 [0-9a-f]{40}|2 [0-9a-f]{64})\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "os".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(android|cumulus|centos64|wheezy|jessie|stretch|buster|bullseye|bookworm|bionic|freebsd_[0-9]+\\.[0-9]+|macos_.*|tvos|other|esx|ciscoios|ios12.2|iDRAC6|windows(2022_std|2019_std|2019_dc|2016_std|2016_dc|2012r2_std|2012r2_dc|2012_std|2012_dc|2008r2_std|2008r2_dc|7|8|10|11))\\Z".to_string(),
readonly: false,
hovertext: "Operation system, e.g. wheezy, jessie or freebsd100 or macos_*".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "loadbalancer_nodes".to_string(),
typ: Relation,
reversed_attribute_id: "domain".to_string(),
multi: true,
regexp: "\\A.*\\Z".to_string(),
readonly: true,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "vrrp_id".to_string(),
typ: Number,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(0|[1-9][0-9]*)\\Z".to_string(),
readonly: false,
hovertext: "This is VHID for CARP or VRID for VRRP.".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "harddisk_type".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(ssd|hdd|unknown)\\Z".to_string(),
readonly: false,
hovertext: "this is either hdd or ssd to give an indication on the performance and io scheduler to be used".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "repositories_host".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A([a-z0-9]+(-[a-z0-9]+)*\\.)+[a-z]{2,}\\Z".to_string(),
readonly: false,
hovertext: "Repositories hostname to fetch packages from. On Debian this is where apt fetches debian packages from (aptly). For FreeBSD this is where pkg fetches FreeBSD packages from.\r\nIt is used to form the correct configs on the hosts.".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "acl_members".to_string(),
typ: Relation,
reversed_attribute_id: "acl".to_string(),
multi: true,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: true,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "state_limit".to_string(),
typ: Number,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(0|[1-9][0-9]*)\\Z".to_string(),
readonly: false,
hovertext: "Maximum amount of states which can be created by traffic to this LB Pool. Any new traffic above this number will be silently dropped.".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "ospf_active".to_string(),
typ: Boolean,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(true|false)\\Z".to_string(),
readonly: false,
hovertext: "Make routers having vlan_interfaces in this network active in OSPF.".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "iops_avg".to_string(),
typ: Number,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(0|[1-9][0-9]*)\\Z".to_string(),
readonly: true,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "libvirt_pool_free_gib".to_string(),
typ: Number,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(0|[1-9][0-9]*)\\Z".to_string(),
readonly: true,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "dns_cname".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(?:(?:[a-zA-Z01-9\\-]*[a-zA-Z01-9]\\.)+(?:[a-z]+))\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "puppet_disabled".to_string(),
typ: Boolean,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(true|false)\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "vms".to_string(),
typ: Relation,
reversed_attribute_id: "hypervisor".to_string(),
multi: true,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: true,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "aws_key_name".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A[a-zA-Z0-9@-]+\\Z".to_string(),
readonly: false,
hovertext: "AWS KeyName".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "allow_from_reverse".to_string(),
typ: Relation,
reversed_attribute_id: "allow_from".to_string(),
multi: true,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: true,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "route_network".to_string(),
typ: Supernet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: true,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "cpu_clock_mhz".to_string(),
typ: Number,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(0|[1-9][0-9]*)\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "ipv6".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty IPv6 address. Comes from somewhere, good luck finding it!".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "rack_row".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "event_tracking_code".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A([a-z0-9]+)\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "public_networks".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "puppet_environment".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(?!(testing|staging|production))[a-z][a-z0-9_]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "state".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(retired|maintenance|cold_standby|deploy_offline|deploy_online|online_reserved|online)\\Z".to_string(),
readonly: false,
hovertext: "Retired: No DNS, no Nagios, lb downtimed\r\nMaintenance: DNS, Nagios downtimed, lb downtimed\r\nDeploy_offline: DNS, Nagios active, lb downtimed\r\nDeploy_online: DNS, Nagios downtimed, lb active\r\nOnline: DNS, Nagios active, lb active".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "disk_size_gib".to_string(),
typ: Number,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(0|[1-9][0-9]*)\\Z".to_string(),
readonly: false,
hovertext: "This is the current disk size of the server in gibibytes.  It should be filled while the VM is created or maintained by other scripts.\r\n".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "hc_host".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "responsible_internal".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "business owner".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "internal".to_string(),
typ: Boolean,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "project_network".to_string(),
typ: Supernet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: true,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "load_avg".to_string(),
typ: Number,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(0|[1-9][0-9]*)\\Z".to_string(),
readonly: true,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "bladecenter_slot".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "backup_prio".to_string(),
typ: Number,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(0|[1-9][0-9]*)\\Z".to_string(),
readonly: false,
hovertext: "Backup priority of the host. Defaults to 300. Backups will be eventually processed regardless of the value of this, but higher number means the dump and transfer of the backup will be prioritized.".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "backup_disabled".to_string(),
typ: Boolean,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(true|false)\\Z".to_string(),
readonly: false,
hovertext: "Set to true if you do not want this server backed up.".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "os_product_license".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "powerports".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "default_snat".to_string(),
typ: Boolean,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(true|false)\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "monitoring_networks".to_string(),
typ: Relation,
reversed_attribute_id: "monitoring_queue".to_string(),
multi: true,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: true,
hovertext: "shows the route_networks for a gearman_queue".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "backup_timestamp".to_string(),
typ: Datetime,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "Last action time of backup system (UTC). Should be set automatically by backup manager.\r\nWe're utilizing this like a cache to quickly check the freshness of the state (real source of truth is the client itself, but not always).".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "hostname".to_string(),
typ: Inet,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "".to_string(),
readonly: false,
hovertext: "The almighty hostname attribute. Comes from somewhere, good luck finding it!".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "served_markets".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "Like game_market used to group (backend) servers for multiple market(s)".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "hc_ok_codes".to_string(),
typ: Number,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A(0|[1-9][0-9]*)\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "project_english_name".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A[a-zA-Z0-9 ]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "table_priority".to_string(),
typ: Number,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(0|[1-9][0-9]*)\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "sg_allow_from".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "responsible_admin".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A[a-z][a-z\\._]+[a-z]\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "sentry_team".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "Sentry team name at sentry.io".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "cpu_util_pct".to_string(),
typ: Number,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(0|[1-9]{1,2})\\Z".to_string(),
readonly: true,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "sg_allow_to".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "hc_user".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "ipmi".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "min_nodes_action".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(force_down|force_up|backup_pool)\\Z".to_string(),
readonly: false,
hovertext: "force_down: switch to backup_pool if active nodes drop below min_nodes\r\nforce_up: always send traffic to last min_nodes hosts, even if they are considered offline".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "routing_table".to_string(),
typ: Number,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(0|[1-9][0-9]*)\\Z".to_string(),
readonly: false,
hovertext: "Number of routing table used for routing to and from this subnet".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "network_type".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(loopback|transit_(internal_igp|public_(hwlbs|uplink|igp))|public|internal|public_dmz|vpn_clients|internal_vpn_clients|cloud)\\Z".to_string(),
readonly: false,
hovertext: "Select what kind of connection you want to switch\r\nsingle: single interface without lacp or anything other special\r\nbond_ab: Bonding without LACP but 2 links with failover\r\nbond_lacp: bonding with lacp and 2 active links".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "game_market".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A[a-z][a-z]|foe|war\\Z".to_string(),
readonly: false,
hovertext: "The 2 letter short code for the game market, see docu for allowed values.".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "dns_mx".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A.+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "no_monitoring".to_string(),
typ: Boolean,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(true|false)\\Z".to_string(),
readonly: false,
hovertext: "if set to true this server will be ignored by the nagios generation script".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "hc_type".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(dns|http|https|ping|postgres|tcp)\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "disk_free_gib".to_string(),
typ: Number,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(0|[1-9][0-9]*)\\Z".to_string(),
readonly: true,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "nessus_folder_name".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(.*)\\Z".to_string(),
readonly: false,
hovertext: "The name of the nessus scan that includes this project_network".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "libvirt_pool_used_gib".to_string(),
typ: Number,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(0|[1-9][0-9]*)\\Z".to_string(),
readonly: true,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "tunnel_interface_of".to_string(),
typ: Relation,
reversed_attribute_id: "tunnel_interfaces".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: true,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "igvm_migration_log".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A[0-9]{10}\\s[-|+][0-9]{1,3}\\Z".to_string(),
readonly: false,
hovertext: "Log for IGVM migrations to properly calculate the destination load of a hypervisor.".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "bladecenter".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "monitoring_satellite_of".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "icinga2".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "monitoring_queues".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "gearman_queues a worker should be responsible for".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "puppet_master".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+[\\.\\-])*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "order_id".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "some reference to the ticked, bill or order for this one".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "backup_state".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(idle|local_backup_running|local_backup_done|local_backup_failed|restic_running|restic_failed|waiting_local_backup_start|please_backup_now|dead)\\Z".to_string(),
readonly: false,
hovertext: "Current state of the backup. Used by backup system, might not be always 100% accurate (by design).".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "internal_domain".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(\\*\\.)?([a-z0-9]+(\\.|-+))*[a-z0-9]+\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "instance".to_string(),
typ: Number,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A[0-9]+\\Z".to_string(),
readonly: false,
hovertext: "used to index multiple instances of the same function".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "hc_dbname".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "datacenter_colo".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "provisioning_step".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A([a-z0-9]+[_])*[a-z0-9]+\\|([a-z0-9]+[\\.\\-])*[a-z0-9]+\\|\\d{10}\\|\\d{10}\\.\\d{6}\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "parent_bond".to_string(),
typ: Relation,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "access_uri".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A(ssh://|ipv4://|ipv6:|https://|http://).*\\Z".to_string(),
readonly: false,
hovertext: "URI to access the device".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "apt_repository_url".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A([a-z0-9]+(-[a-z0-9]+)*\\.)+[a-z]{2,}\\Z".to_string(),
readonly: false,
hovertext: "Debian repository URL for apt e.g. update.innogames.de which will be used to form the entries in /etc/apt/sources.list together with repositories attribute.".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "vlan_tag".to_string(),
typ: Number,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(0|[1-9][0-9]*)\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "mac".to_string(),
typ: Macaddr,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "environment".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(testing|staging|production)\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "restic_group".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A[a-z0-9][a-z0-9-]+[a-z0-9]\\Z".to_string(),
readonly: false,
hovertext: "Name of the backup bucket of this server, project (+ \"-subproject\" if defined) when empty by default".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "short_code".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A([a-z]+)\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "rack_number".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "is_game".to_string(),
typ: Boolean,
reversed_attribute_id: "".to_string(),
multi: false,
regexp: "\\A(true|false)\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "monitoring_group_members".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "".to_string(),
required: false,
default: "".to_string(),
},Attribute {
name: "responsible_external".to_string(),
typ: String,
reversed_attribute_id: "".to_string(),
multi: true,
regexp: "\\A.*\\Z".to_string(),
readonly: false,
hovertext: "extern company who monitory/maintains device or system".to_string(),
required: false,
default: "".to_string(),
}]
}