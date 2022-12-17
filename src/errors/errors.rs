#[derive(Debug)]
pub enum Error {
    // Critical errors
    FailedToStartFetcher,
    FailedToStartSubscriber,
}

impl From<Error> for crate::Error {
    fn from(err: Error) -> Self {
        match err {
            Error::FailedToStartFetcher => {
                crate::Error::Internal("Failed to start fetcher".to_string())
            }
            Error::FailedToStartSubscriber => {
                crate::Error::Internal("Failed to start subscriber".to_string())
            }
        }
    }
}
