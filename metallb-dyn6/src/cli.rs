use clap::Parser;
use clap::ValueEnum;
use std::net::Ipv6Addr;
use std::time::Duration;
use types::ranges::V6HostRange;

macro_rules! env_prefix {
    () => {
        "METALLB_DYN6_"
    };
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Name of the IPAddressPool resource to manage
    #[arg(
        env = concat!(env_prefix!(), "POOL"),
    )]
    pub pool: String,

    /// Source of the IPv4 address to set in all A records
    #[arg(
        value_enum,
        long,
        required = true,
        env = concat!(env_prefix!(), "SOURCE"),
        default_value_t = AddressSource::MyIp
    )]
    pub source: AddressSource,

    /// Use this field manager identity when patching the IPAddressPool
    /// using server-side apply
    #[arg(
        long,
        env = concat!(env_prefix!(), "FIELD_MANAGER"),
        default_value = "metallb-dyn6"
    )]
    pub field_manager: String,

    /// Override a portion of the prefix (usually the subnet). This value must be a valid IPv6 address.
    /// For example, to set the subnet to :beef: with a /48 dynamic prefix, use: 0:0:0:beef::
    #[arg(
        long,
        env = concat!(env_prefix!(), "SUBNET_OVERRIDE"),
        requires = "prefix_length"
    )]
    pub subnet_override: Option<Ipv6Addr>,

    /// Length of the original network prefix that should be preserved when overriding the subnet with --subnet-override.
    /// For example, if you have a /48 prefix and are overriding the subnet with :beef:, set this to 48.
    #[arg(
        long,
        env = concat!(env_prefix!(), "PREFIX_LENGTH"),
        requires = "subnet_override",
        value_parser = clap::value_parser!(u8).range(1..64)
    )]
    pub prefix_length: Option<u8>,

    /// Range of host addresses that MetalLB can use for allocating services.
    /// Must be passed as a range of Ipv6-Host-parts, such as ::1000-::1999
    #[arg(
        env = concat!(env_prefix!(), "HOST_RANGE"),
    )]
    pub host_range: V6HostRange,

    /// Time between attempts to refresh the dynamic Prefix and updating the IPAddressPool in seconds
    #[arg(
        long,
        env = concat!(env_prefix!(), "UPDATE_INTERVAL"),
        default_value_t = 60
    )]
    pub update_interval: u64,

    /// Only show the changes that would be made, but do not update the IPAddresspool.
    /// Useful for testing.
    #[arg(
        long,
        env = concat!(env_prefix!(), "DRY_RUN"),
        default_value_t = false
    )]
    pub dy_run: bool,

    /// The namespace the MetalLB controller and speakers reside in.
    /// Needed to force-reload MetalLB after an address has changed.
    #[arg(
        long,
        env = concat!(env_prefix!(), "METALLB_NAMESPACE"),
        default_value = "metallb-system"
    )]
    pub metallb_namespace: String,
}

/// Which source to use for our Ipv4 address
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, ValueEnum)]
pub enum AddressSource {
    //Interface,
    MyIp,
}
