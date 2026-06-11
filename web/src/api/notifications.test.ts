import { afterEach, describe, expect, mock, test } from 'bun:test';

import { createNotificationChannel, renderNotificationTemplate, updateNotificationChannel } from './notifications';

const originalFetch = globalThis.fetch;

afterEach(() => {
  globalThis.fetch = originalFetch;
});

describe('notification api client', () => {
  test('renders templates through slash render route so dotted template keys are path-safe', async () => {
    let calledUrl = '';
    globalThis.fetch = mock(async (url: string | URL | Request) => {
      calledUrl = String(url);
      return new Response(JSON.stringify({ code: 0, message: 'success', data: { provider: 'slack', messageType: 'text', rendered: { text: 'ok' } } }));
    }) as unknown as typeof fetch;

    await expect(renderNotificationTemplate('ops.slack.failure', { sample: { subject: 'ok' } })).resolves.toMatchObject({ rendered: { text: 'ok' } });

    expect(calledUrl).toBe('/api/v1/notification-templates/ops.slack.failure/render');
    expect(calledUrl).not.toContain(':render');
  });

  test('metadata-only channel updates omit config and secretRefs so stored values are preserved server-side', async () => {
    let payload: Record<string, unknown> | null = null;
    globalThis.fetch = mock(async (_url: string | URL | Request, init?: RequestInit) => {
      payload = init?.body ? JSON.parse(String(init.body)) : null;
      return new Response(JSON.stringify({ code: 0, message: 'success', data: { id: 'channel-1', scopeType: 'global', namespace: null, app: null, workerPool: null, name: 'renamed', provider: 'webhook', enabled: true, configJson: '{}', targetRedacted: 'webhook:secret-ref', safetyPolicyJson: null, targetConfigured: true, secretConfigured: true, createdBy: null, updatedBy: null, createdAt: 'now', updatedAt: 'now' } }));
    }) as unknown as typeof fetch;

    await updateNotificationChannel('channel-1', { name: 'renamed', enabled: true });

    expect(payload as unknown).toEqual({ name: 'renamed', enabled: true });
  });

  test('channel create can send config and secretRefs separately without exposing a secretRefsJson field', async () => {
    let payload: Record<string, unknown> | null = null;
    globalThis.fetch = mock(async (_url: string | URL | Request, init?: RequestInit) => {
      payload = init?.body ? JSON.parse(String(init.body)) : null;
      return new Response(JSON.stringify({ code: 0, message: 'success', data: { id: 'channel-1', scopeType: 'global', namespace: null, app: null, workerPool: null, name: 'webhook', provider: 'webhook', enabled: true, configJson: '{}', targetRedacted: 'webhook:secret-ref', safetyPolicyJson: null, targetConfigured: true, secretConfigured: true, createdBy: null, updatedBy: null, createdAt: 'now', updatedAt: 'now' } }));
    }) as unknown as typeof fetch;

    await createNotificationChannel({
      scopeType: 'global',
      name: 'webhook',
      provider: 'webhook',
      config: { messageType: 'json' },
      secretRefs: { url: 'env:TIKEO_NOTIFICATION_WEBHOOK_URL' },
    });

    expect(payload as unknown).toMatchObject({ config: { messageType: 'json' }, secretRefs: { url: 'env:TIKEO_NOTIFICATION_WEBHOOK_URL' } });
    expect(payload as unknown).not.toHaveProperty('secretRefsJson');
  });
});
