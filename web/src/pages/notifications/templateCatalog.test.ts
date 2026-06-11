import { describe, expect, test } from 'bun:test';

import type { NotificationChannelSummary, NotificationTemplateSummary } from '../../api/notifications';
import { notificationTemplateOptions, selectedPolicyProviders } from './templateCatalog';

const baseChannel: NotificationChannelSummary = {
  id: 'channel-slack',
  scopeType: 'global',
  namespace: null,
  app: null,
  workerPool: null,
  name: 'Slack',
  provider: 'slack',
  enabled: true,
  configJson: '{}',
  targetRedacted: 'slack:secret-ref',
  safetyPolicyJson: null,
  targetConfigured: true,
  secretConfigured: true,
  createdBy: null,
  updatedBy: null,
  createdAt: 'now',
  updatedAt: 'now',
};

const templates: NotificationTemplateSummary[] = [
  { id: 'tpl-1', templateKey: 'ops.slack.failure', name: 'Slack failure', provider: 'slack', messageType: 'blockKit', enabled: true, description: null, bodyJson: '{}', variablesJson: '{}', createdBy: null, updatedBy: null, createdAt: 'now', updatedAt: 'now' },
  { id: 'tpl-2', templateKey: 'ops.feishu.failure', name: 'Feishu failure', provider: 'feishu', messageType: 'text', enabled: true, description: null, bodyJson: '{}', variablesJson: '{}', createdBy: null, updatedBy: null, createdAt: 'now', updatedAt: 'now' },
  { id: 'tpl-3', templateKey: 'ops.slack.disabled', name: 'Disabled', provider: 'slack', messageType: 'text', enabled: false, description: null, bodyJson: '{}', variablesJson: '{}', createdBy: null, updatedBy: null, createdAt: 'now', updatedAt: 'now' },
];

describe('notification template catalog helpers', () => {
  test('selects distinct providers from currently selected policy channels', () => {
    const providers = selectedPolicyProviders([
      baseChannel,
      { ...baseChannel, id: 'channel-feishu', provider: 'feishu' },
      { ...baseChannel, id: 'channel-slack-2', provider: 'slack' },
    ], ['channel-feishu', 'channel-slack-2']);

    expect(providers.sort()).toEqual(['feishu', 'slack']);
  });

  test('template options include only enabled templates matching selected providers', () => {
    expect(notificationTemplateOptions(templates, ['slack'])).toEqual([
      expect.objectContaining({ value: 'ops.slack.failure', provider: 'slack', messageType: 'blockKit' }),
    ]);
  });

  test('template options show all enabled templates before a channel is selected', () => {
    const options = notificationTemplateOptions(templates, []);

    expect(options.map((item) => item.value).sort()).toEqual(['ops.feishu.failure', 'ops.slack.failure']);
    expect(options.some((item) => item.value === 'ops.slack.disabled')).toBe(false);
  });
});
