/// The configuration parameters for the application.
///
/// These can either be passed on the command line, or pulled from environment variables.
/// The latter is preferred as environment variables are one of the recommended ways to
/// get configuration from Kubernetes Secrets in deployment.
///
/// This is a pretty simple configuration struct as far as backend APIs go. You could imagine
/// a bunch of other parameters going here, like API keys for external services
/// or flags enabling or disabling certain features or test modes of the API.
///
/// For development convenience, these can also be read from a `.env` file in the working
/// directory where the application is started.
#[derive(clap::Parser)]
pub struct Config {
    /// The connection URL for the Postgres database this application should use.
    #[clap(long, env)]
    pub database_url: String,

    /// The crank_authority account address we are validating incoming requests with (where applicable).
    ///
    /// See endpoint logic for where this is used
    #[clap(long, env)]
    pub crank_authority: String,

    /// MySQL max connections config
    #[clap(long, env)]
    pub max_connections: u32,
}
