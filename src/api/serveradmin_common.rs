pub enum PredefinedColumn<'a> {
    Always(&'a str),
    Detailed(&'a str),
}

impl<'a> PredefinedColumn<'a> {
    pub fn get_name(&self) -> &'a str {
        match self {
            PredefinedColumn::Always(name) => name,
            PredefinedColumn::Detailed(name) => name,
        }
    }
}

pub const DEFAULT_ATTRIBUTES: &[PredefinedColumn] = &[
    PredefinedColumn::Always("hostname"),
    PredefinedColumn::Always("servertype"),
    PredefinedColumn::Always("state"),
    PredefinedColumn::Detailed("intern_ip"),
    PredefinedColumn::Detailed("primary_ip6"),
];

pub const VISIBLE_ATTRIBUTES: &[(&str, &[PredefinedColumn])] = &[
    (
        "VM",
        &[
            PredefinedColumn::Always("hostname"),
            PredefinedColumn::Always("state"),
            PredefinedColumn::Always("ipv6"),
            PredefinedColumn::Always("ipv4"),
            PredefinedColumn::Detailed("hypervisor"),
            PredefinedColumn::Detailed("service_groups"),
            PredefinedColumn::Detailed("loadbalancer"),
            PredefinedColumn::Detailed("route_network"),
            PredefinedColumn::Detailed("project_network"),
        ],
    ),
    (
        "ServiceGroup",
        &[
            PredefinedColumn::Always("hostname"),
            PredefinedColumn::Always("state"),
            PredefinedColumn::Always("protocol_ports_inbound"),
            PredefinedColumn::Always("protocol_ports_outbound"),
            PredefinedColumn::Always("service_group_members"),
        ],
    ),
    (
        "Loadbalancer",
        &[
            PredefinedColumn::Always("hostname"),
            PredefinedColumn::Always("state"),
            PredefinedColumn::Always("ipv6"),
            PredefinedColumn::Always("ipv4"),
            PredefinedColumn::Detailed("lb_nodes"),
            PredefinedColumn::Detailed("route_network"),
        ],
    ),
];
