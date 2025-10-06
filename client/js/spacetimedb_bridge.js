import {Client} from "@spacetimedb/browser-sdk";

let client;

export async function connect(uri, module, token) {
    client = await Client.connect({uri, module, token});
}

export async function callReducer(name, payload) {
    return client.callReducer(name, payload);
}

export function subscribe(query, onMessage) {
    return client.subscribe(query, onMessage);
}