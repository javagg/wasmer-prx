
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    DatagramWarn(anyhow::Error),
    #[error(transparent)]
    DatagramFatal(anyhow::Error),
}

unsafe impl Sync for Error {}
unsafe impl Send for Error {}
