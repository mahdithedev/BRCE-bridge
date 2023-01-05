
    use core::fmt;

    #[derive(PartialEq , Eq , Copy, Clone , Debug)]
    pub enum ErrorCode {
        TargetOffline,
        InvalidProfileId,
        ChannelIsnotCreated,
        AccessDenied,
        Unknown,
    }

    impl From<ErrorCode> for u8 {
        fn from(error_code: ErrorCode) -> Self {
            match error_code {
                ErrorCode::TargetOffline => 0,
                ErrorCode::InvalidProfileId => 1,
                ErrorCode::ChannelIsnotCreated => 2,
                ErrorCode::AccessDenied => 3,
                ErrorCode::Unknown => 99,
            }
        }
    }

    impl From<u8> for ErrorCode {
        fn from(error_code: u8) -> Self {
            match error_code {
                0 => ErrorCode::TargetOffline,
                1 => ErrorCode::InvalidProfileId,
                2 => ErrorCode::ChannelIsnotCreated,
                3 => ErrorCode::AccessDenied,
                _n => ErrorCode::Unknown,
            }
        }
    }

    impl fmt::Display for ErrorCode {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                ErrorCode::TargetOffline => write!(f , "The target is ofline"),
                ErrorCode::InvalidProfileId => write!(f , "Profile ID doesn't exist"),
                ErrorCode::ChannelIsnotCreated => write!(f , "The channel is not created"),
                ErrorCode::AccessDenied => write!(f, " You do not have access to this action"),
                _ => write!(f , "unhandeled"),
            }
        }
    }

    #[derive(Eq , PartialEq , Clone , Copy , Debug)]
    pub struct NetworkErr(ErrorCode);

    impl fmt::Display for NetworkErr {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f , "{}" , self.0)
        }   
    }
