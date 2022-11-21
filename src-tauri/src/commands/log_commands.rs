use std::{error::Error, fmt};

use log::{debug, info, warn, trace, error};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct UnknownLogSeverityInvocation {
    severity: String,
}
  
impl Error for UnknownLogSeverityInvocation {}

impl fmt::Display for UnknownLogSeverityInvocation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} is not a known severity", self.severity)
    }
}

#[tauri::command]
pub async fn backend_log(message: &str, severity: &str) -> Result<(), UnknownLogSeverityInvocation> {
    let res = match severity {
        "info" => Ok(info!("{}", message)),
        "debug" => Ok(debug!("{}", message)),
        "warn" => Ok(warn!("{}", message)),
        "trace" => Ok(trace!("{}", message)),
        "error" => Ok(error!("{}", message)),
        _ => Err(UnknownLogSeverityInvocation {
            severity: severity.clone().to_owned(),
        }),
    };
    res
}
