import { BaseDirectory, create, readTextFile, exists } from "@tauri-apps/plugin-fs";

export async function createAccount(username: string) {
    const trimmed = username.trim();
    if (!trimmed) throw new Error("empty username");

    let accounts: string[] = [];
    const fileExists = await exists('account.json', { baseDir: BaseDirectory.AppData });
    if (fileExists) {
        const raw = await readTextFile('account.json', { baseDir: BaseDirectory.AppData });
        const parsed = JSON.parse(raw);
        accounts = Array.isArray(parsed) ? parsed : [];
    }

    if (accounts.includes(trimmed)) throw new Error("username already exists");

    accounts.push(trimmed);

    const file = await create('account.json', { baseDir: BaseDirectory.AppData });
    await file.write(new TextEncoder().encode(JSON.stringify(accounts)));
    await file.close();
}