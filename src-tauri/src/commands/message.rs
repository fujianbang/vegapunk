use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
pub enum AuthMethod {
    Password = 0,
    PublicKey = 1,
}

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
pub enum HostOS {
    Windows = 0,
    Apple = 1,
    Android = 2,
    Ubuntu = 3,
    Fedor = 4,
    OpenSUSE = 5,
    CentOS = 6,
    ArchLinux = 7,
    Debian = 8,
    RedHat = 9,
    Kali = 10,
    RockyLinux = 11,
    Unknown = 99,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Host {
    pub(crate) id: String,
    pub(crate) name: String,
    pub(crate) address: String,
    pub(crate) port: u16,
    pub(crate) auth_method: AuthMethod,
    pub(crate) os: HostOS,
    pub(crate) comment: String,
}
