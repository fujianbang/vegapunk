import {invoke} from "@tauri-apps/api/core";
import type {HostInfo} from "$lib/types/host";

export function invoke_host_list(): Promise<HostInfo[]> {
    return invoke("get_hosts");
}