import {hostAuthMethod, osType, type HostInfo} from "$lib/types/host";


export function load() {
    let osIcon = osSvgIconMapping;
    let hosts: HostInfo[] = [
        {
            name: "Windows Server Server Server",
            address: "192.168.1.1",
            port: 22,
            auth_method: hostAuthMethod.Password,
            os: osType.Windows,
            comment: "This is a comment",
        },
        {
            name: "Virtual Machine",
            address: "192.168.1.1",
            port: 60022,
            auth_method: hostAuthMethod.Password,
            os: osType.Ubuntu,
            comment: "This is a comment",
        },
        {
            name: "Local QA",
            address: "192.168.1.1",
            port: 22,
            auth_method: hostAuthMethod.Password,
            os: osType.Fedora,
            comment: "This is a comment",
        },
        {
            name: "QA",
            address: "192.168.1.1",
            port: 22,
            auth_method: hostAuthMethod.Password,
            os: osType.Debian,
            comment: "This is a comment",
        },
        {
            name: "Outside Server",
            address: "192.168.1.1",
            port: 22,
            auth_method: hostAuthMethod.Password,
            os: osType.CentOS,
            comment: "This is a comment",
        },
        {
            name: "10.0.0.1",
            address: "192.168.1.1",
            port: 22,
            auth_method: hostAuthMethod.Password,
            os: osType.ArchLinux,
            comment: "This is a comment",
        },
        {
            name: "10.0.0.1",
            address: "192.168.1.1",
            port: 22,
            auth_method: hostAuthMethod.Password,
            os: osType.Kali,
            comment: "This is a comment",
        },
        {
            name: "10.0.0.1",
            address: "192.168.1.1",
            port: 22,
            auth_method: hostAuthMethod.Password,
            os: osType.openSUSE,
            comment: "This is a comment",
        },
        {
            name: "10.0.0.1",
            address: "192.168.1.1",
            port: 22,
            auth_method: hostAuthMethod.Password,
            os: osType.Android,
            comment: "This is a comment",
        },
        {
            name: "10.0.0.1",
            address: "192.168.1.1",
            port: 22,
            auth_method: hostAuthMethod.Password,
            os: osType.Apple,
            comment: "This is a comment",
        },
        {
            name: "10.0.0.1",
            address: "192.168.1.1",
            port: 22,
            auth_method: hostAuthMethod.Password,
            os: osType.RockyLinux,
            comment: "This is a comment",
        },
    ];

    return {
        hostInfo: hosts,
        osIcon: osIcon,
    };
}
