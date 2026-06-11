export function formatJson(raw: string | null | undefined, fallback = '{}'): string {
  if (!raw || raw.trim() === '') return fallback;
  try {
    return JSON.stringify(JSON.parse(raw), null, 2);
  } catch {
    return raw;
  }
}

export function blankToNull(value: string | undefined): string | null {
  const trimmed = value?.trim();
  return trimmed ? trimmed : null;
}

export function parseJsonObject(raw: string | undefined, fieldLabel: string, fallback: Record<string, unknown> | null): Record<string, unknown> | null {
  const trimmed = raw?.trim();
  if (!trimmed) return fallback;
  const parsed = JSON.parse(trimmed) as unknown;
  if (parsed === null) return null;
  if (typeof parsed !== 'object' || Array.isArray(parsed)) {
    throw new Error(`${fieldLabel} 必须是 JSON object`);
  }
  return parsed as Record<string, unknown>;
}

export function assertNoRedactedMarker(raw: string | undefined, fieldLabel: string) {
  if (raw?.includes('***redacted***')) {
    throw new Error(`${fieldLabel} 包含脱敏占位符；请填写完整新值，或保持字段不变以保留原配置。`);
  }
}

export function parseMaybeJson(raw: string | undefined): unknown {
  const trimmed = raw?.trim();
  if (!trimmed) return undefined;
  try {
    return JSON.parse(trimmed);
  } catch {
    return raw;
  }
}

export function compactObject(value: Record<string, unknown>): Record<string, unknown> {
  return Object.fromEntries(
    Object.entries(value).filter(([, item]) => {
      if (item === undefined || item === null) return false;
      if (typeof item === 'string') return item.trim() !== '';
      if (Array.isArray(item)) return item.length > 0;
      return true;
    }),
  );
}
