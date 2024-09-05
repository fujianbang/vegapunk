export enum hostAuthMethod {
  Password,
  PublicKey,
}

export enum hostOs {
  Ubuntu,
  Fedora,
  openSUSE,
  CentOS,
  ArchLinux,
  Debian,
  RedHat,
  Kali,
}

export interface HostInfo {
  name: string; /// aaa
  address: string;
  port: number;
  authMethod: hostAuthMethod;
  os: hostOs;
  comment: string;
}
