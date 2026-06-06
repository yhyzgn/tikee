export interface FixedRateFields {
  fixedRateValue?: number;
  fixedRateUnit: string;
  fixedRateJitterValue?: number;
  fixedRateJitterUnit: string;
}

const DURATION_PATTERN = /^(\d+(?:\.\d+)?)(ns|us|ms|s|m|h|d|w|month|year)$/;

export function durationExpr(value?: number | null, unit?: string | null): string | null {
  return value ? `${value}${unit || 's'}` : null;
}

export function parseFixedRate(expr?: string | null): FixedRateFields {
  const [interval = '', ...options] = String(expr ?? '').trim().split(';').map((item) => item.trim()).filter(Boolean);
  const match = interval.match(DURATION_PATTERN);
  const jitterOption = options.find((item) => item.startsWith('jitter='))?.slice('jitter='.length) ?? '';
  const jitterMatch = jitterOption.match(DURATION_PATTERN);
  return {
    fixedRateValue: match ? Number(match[1]) : undefined,
    fixedRateUnit: match?.[2] ?? 's',
    fixedRateJitterValue: jitterMatch ? Number(jitterMatch[1]) : undefined,
    fixedRateJitterUnit: jitterMatch?.[2] ?? 's',
  };
}
