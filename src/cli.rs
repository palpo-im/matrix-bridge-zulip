use clap::Parser;

#[derive(Parser, Debug, Clone)]
#[command(
    name = "matrix-bridge-zulip",
    author,
    version,
    about = "A puppeting Matrix - Zulip appservice bridge",
    long_about = None
)]
pub struct CliArgs {
    #[arg(
        short = 'c',
        long = "config",
        value_name = "FILE",
        help = "Configuration file path",
        default_value = "config.yaml"
    )]
    pub config: String,

    #[arg(
        short = 'g',
        long = "generate",
        help = "Generate registration YAML for Matrix homeserver (Synapse)"
    )]
    pub generate: bool,

    #[arg(
        long = "generate-compat",
        help = "Generate registration YAML for Matrix homeserver (Dendrite and Conduit)"
    )]
    pub generate_compat: bool,

    #[arg(
        short = 'l',
        long = "listen-address",
        help = "Bridge listen address",
        env = "BRIDGE_LISTEN_ADDRESS"
    )]
    pub listen_address: Option<String>,

    #[arg(
        short = 'p',
        long = "listen-port",
        help = "Bridge listen port",
        env = "BRIDGE_LISTEN_PORT"
    )]
    pub listen_port: Option<u16>,

    #[arg(
        long = "homeserver",
        help = "URL of Matrix homeserver",
        default_value = "http://localhost:8008",
        env = "HOMESERVER_URL"
    )]
    pub homeserver: String,

    #[arg(short = 'v', long = "verbose", help = "Log debug messages")]
    pub verbose: bool,

    #[arg(
        long = "unsafe-mode",
        help = "Allow appservice to leave rooms on error"
    )]
    pub unsafe_mode: bool,

    #[arg(
        short = 'o',
        long = "owner",
        help = "Set owner MXID (eg: @user:homeserver)",
        env = "BRIDGE_OWNER"
    )]
    pub owner: Option<String>,

    #[arg(
        long = "reset",
        help = "Reset ALL bridge configuration from homeserver and exit"
    )]
    pub reset: bool,
}

impl CliArgs {
    pub fn parse_args() -> Self {
        Self::parse()
    }
}
