/**
 * Simple memoization utility for expensive computations
 */
export function memoize<Args extends unknown[], Result>(
  fn: (...args: Args) => Result,
  getKey?: (...args: Args) => string
): (...args: Args) => Result {
  const cache = new Map<string, Result>();

  return (...args: Args): Result => {
    const key = getKey ? getKey(...args) : JSON.stringify(args);

    if (cache.has(key)) {
      return cache.get(key)!;
    }

    const result = fn(...args);
    cache.set(key, result);

    // Limit cache size to prevent memory leaks
    if (cache.size > 100) {
      const firstKey = cache.keys().next().value;
      if (firstKey !== undefined) {
        cache.delete(firstKey);
      }
    }

    return result;
  };
}

/**
 * Memoize with TTL (time-to-live)
 */
export function memoizeWithTTL<Args extends unknown[], Result>(
  fn: (...args: Args) => Result,
  ttlMs: number,
  getKey?: (...args: Args) => string
): (...args: Args) => Result {
  const cache = new Map<string, { value: Result; expiry: number }>();

  return (...args: Args): Result => {
    const key = getKey ? getKey(...args) : JSON.stringify(args);
    const now = Date.now();

    const cached = cache.get(key);
    if (cached && cached.expiry > now) {
      return cached.value;
    }

    const result = fn(...args);
    cache.set(key, { value: result, expiry: now + ttlMs });

    // Clean expired entries
    for (const [k, v] of cache.entries()) {
      if (v.expiry <= now) {
        cache.delete(k);
      }
    }

    return result;
  };
}