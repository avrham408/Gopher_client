#[derive(Clone, Copy, Debug)]
pub(crate) struct Config {}

impl Config {
    #[allow(dead_code)]
    pub(crate) fn from_env() -> Self {
        env_logger::init();
        return Self {};
    }
}
