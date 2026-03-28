import { describe, it, expect, vi, beforeEach } from 'vitest';

vi.mock('@tauri-apps/plugin-fs', () => ({
    BaseDirectory: { AppData: 'AppData' },
    mkdir: vi.fn().mockResolvedValue(undefined),
    create: vi.fn().mockResolvedValue({
        write: vi.fn().mockResolvedValue(undefined),
        close: vi.fn().mockResolvedValue(undefined),
    }),
}));

import { createAccount } from './account';
import { mkdir, create } from '@tauri-apps/plugin-fs';

beforeEach(() => vi.clearAllMocks());

describe('createAccount', () => {
    it('throws on empty username', async () => {
        await expect(createAccount('')).rejects.toThrow('empty username');
    });

    it('throws on whitespace username', async () => {
        await expect(createAccount('   ')).rejects.toThrow('empty username');
    });

    it('calls mkdir and create on valid username', async () => {
        await createAccount('Fynr1x');
        expect(create).toHaveBeenCalledWith('account.json', expect.anything());
    });

});