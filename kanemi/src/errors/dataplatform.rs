use std::fmt;

/// Errors that can occur within the API
#[derive(Debug)]
pub enum ApiError {
    /// An error occurred during the HTTP request.
    FetchError(String),

    /// An error occurred during parsing the response.
    UrlResponseParseError(String),

    /// An error occurred during parsing the files response.
    FilesResponseParseError(String),

    /// An error occurred during parsing the notification response.
    NotificationResponseParseError(String),

    /// Missing or invalid configuration.
    ConfigurationError(String),

    /// An error occurred during saving the file.
    SaveFileError(String),
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ApiError::FetchError(s) => write!(f, "Fetch error: {}", s),
            ApiError::UrlResponseParseError(s) => write!(f, "Url response parse error: {}", s),
            ApiError::FilesResponseParseError(s) => write!(f, "Files response parse error: {}", s),
            ApiError::NotificationResponseParseError(s) => {
                write!(f, "Notification response parse error: {}", s)
            }
            ApiError::ConfigurationError(s) => write!(f, "Configuration error: {}", s),
            ApiError::SaveFileError(s) => write!(f, "Error saving file: {}", s),
        }
    }
}

#[derive(Debug)]
pub enum NotificationError {
    /// Connection was lost
    ConnectionError(String),

    /// Subscription error
    SubscriptionError(String),
}

impl fmt::Display for NotificationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            NotificationError::ConnectionError(s) => write!(f, "Connection error: {}", s),
            NotificationError::SubscriptionError(s) => write!(f, "Subscription error: {}", s),
        }
    }
}
