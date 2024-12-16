export enum hostAuthMethod {
    Password,
    PublicKey,
}

export enum osType {
    Windows,
    Apple,
    Android,
    Ubuntu,
    Fedora,
    OpenSUSE,
    CentOS,
    ArchLinux,
    Debian,
    RedHat,
    Kali,
    RockyLinux,
    Unknown = 99,
}

export interface HostInfo {
    id: string,
    name: string;
    address: string;
    port: number;
    auth_method: hostAuthMethod;
    os: osType;
    comment: string;
}
