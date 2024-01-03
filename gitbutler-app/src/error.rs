mod new_types;
#[cfg(feature = "sentry")]
mod sentry;

pub use legacy::*;

//#[deprecated(
//    note = "the types in the error::legacy::* module are deprecated; use error::gb::Error and error::gb::Result instead"
//)]
mod legacy {
    use core::fmt;

    use serde::{ser::SerializeMap, Serialize};

    #[derive(Debug)]
    pub enum Code {
        Unknown,
        Validation,
        Projects,
        Branches,
        ProjectGitAuth,
        ProjectGitRemote,
        ProjectConflict,
        ProjectHead,
        Menu,
        PreCommitHook,
        CommitMsgHook,
    }

    impl fmt::Display for Code {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Code::Menu => write!(f, "errors.menu"),
                Code::Unknown => write!(f, "errors.unknown"),
                Code::Validation => write!(f, "errors.validation"),
                Code::Projects => write!(f, "errors.projects"),
                Code::Branches => write!(f, "errors.branches"),
                Code::ProjectGitAuth => write!(f, "errors.projects.git.auth"),
                Code::ProjectGitRemote => write!(f, "errors.projects.git.remote"),
                Code::ProjectHead => write!(f, "errors.projects.head"),
                Code::ProjectConflict => write!(f, "errors.projects.conflict"),
                //TODO: rename js side to be more precise what kind of hook error this is
                Code::PreCommitHook => write!(f, "errors.hook"),
                Code::CommitMsgHook => write!(f, "errors.hooks.commit.msg"),
            }
        }
    }

    #[derive(Debug, thiserror::Error)]
    pub enum UserError {
        #[error("[{code}]: {message}")]
        User { code: Code, message: String },
        #[error("[errors.unknown]: Something went wrong")]
        Unknown,
    }

    impl Serialize for UserError {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            let (code, message) = match self {
                UserError::User { code, message } => (code.to_string(), message.to_string()),
                UserError::Unknown => (
                    Code::Unknown.to_string(),
                    "Something went wrong".to_string(),
                ),
            };

            let mut map = serializer.serialize_map(Some(2))?;
            map.serialize_entry("code", &code)?;
            map.serialize_entry("message", &message)?;
            map.end()
        }
    }

    impl From<anyhow::Error> for UserError {
        fn from(error: anyhow::Error) -> Self {
            tracing::error!(?error);
            UserError::Unknown
        }
    }
}