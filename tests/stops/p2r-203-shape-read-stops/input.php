<?php

declare(strict_types=1);

/**
 * @param array{a: int} $s
 */
function keyed(array $s): int
{
    $k = 'a';
    return $s[$k];
}

/**
 * @param ?array{a: int} $s
 */
function maybe(?array $s): int
{
    return $s['a'];
}

/**
 * @param list<array{k: ?string}> $ps
 */
function first(array $ps): string
{
    return $ps[0]['k'] ?? 'none';
}

/**
 * @param ?array{a: int} $s
 */
function fallback(?array $s): int
{
    return $s['a'] ?? 0;
}

function literal(): int
{
    if (['a' => 1]['a'] > 0) {
        return 1;
    }
    return 0;
}
