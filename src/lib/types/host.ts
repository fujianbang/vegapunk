export enum HostAuthMethod {
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
    auth_method: HostAuthMethod;
    os: osType;
    comment: string;
}

export interface AddNewHostRequest {
    name: string;
    address: string;
    port: number;
    username: string;
    password: string;
}