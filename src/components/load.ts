import {hostAuthMethod, hostOs, type HostInfo} from "$lib/types/host";

function osSvgIconMapping(os: hostOs) {
    switch (os) {
        case hostOs.Windows:
            return "devicon:windows8";
        case hostOs.Apple:
            return "devicon:apple";
        case hostOs.Android:
            return "devicon:android";
        case hostOs.Debian:
            return "devicon:debian";
        case hostOs.Ubuntu:
            return "logos:ubuntu";
        case hostOs.Fedora:
            return "devicon:fedora";
        case hostOs.openSUSE:
            return "devicon:opensuse";
        case hostOs.CentOS:
            return "devicon:centos";
        case hostOs.ArchLinux:
            return "devicon:archlinux";
        case hostOs.RedHat:
            return "devicon:kalilinux";
        case hostOs.Kali:
            return "devicon:kalilinux";
        case hostOs.RockyLinux:
            return "devicon:rockylinux";
        default:
            return "devicon:linux";
    }
}
export function load() {
    let osIcon = osSvgIconMapping;
    let hosts: HostInfo[] = [
        {
            name: "Windows Server Server Server",
            address: "192.168.1.1",
            port: 22,
            authMethod: hostAuthMethod.Password,
            os: hostOs.Windows,
            comment: "This is a comment",
        },
        {
            name: "Virtual Machine",
            address: "192.168.1.1",
            port: 60022,
            authMethod: hostAuthMethod.Password,
            os: hostOs.Ubuntu,
            comment: "This is a comment",
        },
        {
            name: "Local QA",
            address: "192.168.1.1",
            port: 22,
            authMethod: hostAuthMethod.Password,
            os: hostOs.Fedora,
            comment: "This is a comment",
        },
        {
            name: "QA",
            address: "192.168.1.1",
            port: 22,
            authMethod: hostAuthMethod.Password,
            os: hostOs.Debian,
            comment: "This is a comment",
        },
        {
            name: "Outside Server",
            address: "192.168.1.1",
            port: 22,
            authMethod: hostAuthMethod.Password,
            os: hostOs.CentOS,
            comment: "This is a comment",
        },
        {
            name: "10.0.0.1",
            address: "192.168.1.1",
            port: 22,
            authMethod: hostAuthMethod.Password,
            os: hostOs.ArchLinux,
            comment: "This is a comment",
        },
        {
            name: "10.0.0.1",
            address: "192.168.1.1",
            port: 22,
            authMethod: hostAuthMethod.Password,
            os: hostOs.Kali,
            comment: "This is a comment",
        },
        {
            name: "10.0.0.1",
            address: "192.168.1.1",
            port: 22,
            authMethod: hostAuthMethod.Password,
            os: hostOs.openSUSE,
            comment: "This is a comment",
        },
        {
            name: "10.0.0.1",
            address: "192.168.1.1",
            port: 22,
            authMethod: hostAuthMethod.Password,
            os: hostOs.Android,
            comment: "This is a comment",
        },
        {
            name: "10.0.0.1",
            address: "192.168.1.1",
            port: 22,
            authMethod: hostAuthMethod.Password,
            os: hostOs.Apple,
            comment: "This is a comment",
        },
        {
            name: "10.0.0.1",
            address: "192.168.1.1",
            port: 22,
            authMethod: hostAuthMethod.Password,
            os: hostOs.RockyLinux,
            comment: "This is a comment",
        },
    ];

    return {
        hostInfo: hosts,
        osIcon: osIcon,
    };
}
