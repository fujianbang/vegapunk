export enum hostAuthMethod {
  Password,
  PublicKey,
}

export enum hostOs {
  Windows,
  Apple,
  Android,
  Ubuntu,
  Fedora,
  openSUSE,
  CentOS,
  ArchLinux,
  Debian,
  RedHat,
  Kali,
  RockyLinux,
}

export interface HostInfo {
  name: string; /// aaa
  address: string;
  port: number;
  authMethod: hostAuthMethod;
  os: hostOs;
  comment: string;
}
