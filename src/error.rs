use snafu::Snafu;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub))]
pub enum Error {
    #[snafu(display("SSH connection to {host} failed: {source}"))]
    SshConnect {
        host: String,
        source: openssh::Error,
    },

    #[snafu(display("SSH command on {host} failed: {source}"))]
    SshCommand {
        host: String,
        source: openssh::Error,
    },
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
