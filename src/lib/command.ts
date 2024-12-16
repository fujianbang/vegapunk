import {invoke} from "@tauri-apps/api/core";
import type {AddNewHostRequest, HostInfo} from "$lib/types/host";

export function invoke_host_list(): Promise<HostInfo[]> {
    return invoke("get_hosts");
}

export function invoke_host_add(host: AddNewHostRequest): Promise<void> {
    return invoke("add_new_host", {
        "name": host.name,
        "address": host.address,
        "port": host.port,
        "username": host.username,
        "password": host.password,
    });
}