import type { NotificationChannelSummary, NotificationTemplateSummary } from '../../api/notifications';

export interface TemplateOption {
  value: string;
  label: string;
  provider: string;
  messageType: string;
  disabled?: boolean;
}

export function selectedPolicyProviders(channels: NotificationChannelSummary[], selectedChannelIds: string[] | undefined): string[] {
  const selected = new Set(selectedChannelIds ?? []);
  if (selected.size === 0) return [];
  return Array.from(new Set(channels.filter((channel) => selected.has(channel.id)).map((channel) => channel.provider)));
}

export function notificationTemplateOptions(
  templates: NotificationTemplateSummary[],
  providerFilter: string[] = [],
): TemplateOption[] {
  const allowed = new Set(providerFilter.filter(Boolean));
  return templates.flatMap((template) => {
    if (!template.enabled) return [];
    if (allowed.size > 0 && !allowed.has(template.provider)) return [];
    return [{
      value: template.templateKey,
      label: `${template.name} · ${template.provider}/${template.messageType} · ${template.templateKey}`,
      provider: template.provider,
      messageType: template.messageType,
    }];
  });
}
