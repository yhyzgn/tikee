import { describe, expect, test } from 'bun:test';

import { parseFixedRate } from '../jobs/jobSchedule';

describe('job schedule helpers', () => {
  test('parses empty API schedule expression without crashing edit drawer replay', () => {
    expect(parseFixedRate(null)).toEqual({
      fixedRateValue: undefined,
      fixedRateUnit: 's',
      fixedRateJitterValue: undefined,
      fixedRateJitterUnit: 's',
    });
    expect(parseFixedRate(undefined).fixedRateUnit).toBe('s');
  });

  test('parses fixed rate interval and optional jitter', () => {
    expect(parseFixedRate('30s;jitter=5s')).toEqual({
      fixedRateValue: 30,
      fixedRateUnit: 's',
      fixedRateJitterValue: 5,
      fixedRateJitterUnit: 's',
    });
  });
});
