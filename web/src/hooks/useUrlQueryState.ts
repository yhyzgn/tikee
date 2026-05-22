import { useCallback, useMemo } from 'react';
import { useSearchParams } from 'react-router-dom';

export type QueryValue = string | number | boolean | null | undefined;

export function useUrlQueryState<T extends Record<string, QueryValue>>(defaults: T) {
  const [searchParams, setSearchParams] = useSearchParams();

  const query = useMemo(() => {
    const next: Record<string, QueryValue> = { ...defaults };
    for (const key of Object.keys(defaults)) {
      const value = searchParams.get(key);
      if (value === null) continue;
      const defaultValue = defaults[key];
      if (typeof defaultValue === 'number') {
        const parsed = Number(value);
        next[key] = Number.isFinite(parsed) ? parsed : defaultValue;
      } else if (typeof defaultValue === 'boolean') {
        next[key] = value === 'true';
      } else {
        next[key] = value;
      }
    }
    return next as T;
  }, [defaults, searchParams]);

  const setQuery = useCallback((patch: Partial<T>, options: { replace?: boolean } = { replace: true }) => {
    setSearchParams((current) => {
      const next = new URLSearchParams(current);
      for (const [key, value] of Object.entries(patch)) {
        const defaultValue = defaults[key];
        if (value === undefined || value === null || value === '' || value === defaultValue) {
          next.delete(key);
        } else {
          next.set(key, String(value));
        }
      }
      return next;
    }, options);
  }, [defaults, setSearchParams]);

  const resetQuery = useCallback(() => setSearchParams(new URLSearchParams(), { replace: true }), [setSearchParams]);

  return { query, setQuery, resetQuery };
}
