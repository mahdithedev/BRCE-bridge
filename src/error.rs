
use core::fmt;

#[derive(PartialEq , Eq , Copy, Clone , Debug)]
pub enum NetworkErrorCode {
    TargetOffline,
    InvalidProfileId,
    ChannelIsnotCreated,
    AccessDenied,
    Unknown,
}

impl From<NetworkErrorCode> for u8 {
    fn from(error_code: NetworkErrorCode) -> Self {
        match error_code {
            NetworkErrorCode::TargetOffline => 0,
            NetworkErrorCode::InvalidProfileId => 1,
            NetworkErrorCode::ChannelIsnotCreated => 2,
            NetworkErrorCode::AccessDenied => 3,
            NetworkErrorCode::Unknown => 99,
        }
    }
}

impl From<u8> for NetworkErrorCode {
    fn from(error_code: u8) -> Self {
        match error_code {
            0 => NetworkErrorCode::TargetOffline,
            1 => NetworkErrorCode::InvalidProfileId,
            2 => NetworkErrorCode::ChannelIsnotCreated,
            3 => NetworkErrorCode::AccessDenied,
            _n => NetworkErrorCode::Unknown,
        }
    }
}

impl fmt::Display for NetworkErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NetworkErrorCode::TargetOffline => write!(f , "The target is ofline"),
            NetworkErrorCode::InvalidProfileId => write!(f , "Profile ID doesn't exist"),
            NetworkErrorCode::ChannelIsnotCreated => write!(f , "The channel is not created"),
            NetworkErrorCode::AccessDenied => write!(f, " You do not have access to this action"),
            _ => write!(f , "unhandeled"),
        }
    }
}

#[derive(Eq , PartialEq , Clone , Copy , Debug)]
pub struct NetworkError(pub NetworkErrorCode);

impl fmt::Display for NetworkError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f , "{}" , self.0)
    }   
}

#[derive(Eq , PartialEq , Clone , Debug)]
pub enum ApplicationErrorCode {
BYTESNOTENOUGH,
}

impl From<ApplicationErrorCode> for u8 {
    fn from(error_code: ApplicationErrorCode) -> Self {
        match error_code {
            ApplicationErrorCode::BYTESNOTENOUGH => 0,
        }
    }
}

impl From<u8> for ApplicationErrorCode {
    fn from(error_code: u8) -> Self {
        match error_code {
            0 => ApplicationErrorCode::BYTESNOTENOUGH,
            _n => unreachable!(),
        }
    }
}

impl fmt::Display for ApplicationErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ApplicationErrorCode::BYTESNOTENOUGH => write!(f , "bytes not enough")
        }
    }
}

#[derive(Eq , PartialEq , Clone , Debug)]
pub struct ApplicationError(pub ApplicationErrorCode);

pub type NeErCode = NetworkErrorCode;
pub type ApErCode = ApplicationErrorCode;


impl fmt::Display for ApplicationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f , "{}" , self.0)
    }   
}