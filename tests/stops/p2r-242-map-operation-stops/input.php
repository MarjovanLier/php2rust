<?php

declare(strict_types=1);

final class Box
{
    /**
     * @param array<string, int> $m
     */
    public function __construct(public array $m)
    {
    }

    public function put(): void
    {
        $this->m['k'] = 1;
    }

    public function drop(): void
    {
        unset($this->m['k']);
    }

    public function push(): void
    {
        $this->m[] = 1;
    }
}

/**
 * @param array<int, int> $m
 */
function append(array $m): int
{
    $m[] = 5;
    return count($m);
}

/**
 * @param array<string, int> $m
 */
function textKeys(array $m): int
{
    $n = 0;
    foreach ($m as $k => $v) {
        $n += strlen($k) + $v;
    }
    return $n;
}

/**
 * @param array<int, int> $m
 */
function changeInLoop(array $m): int
{
    foreach ($m as $v) {
        $m[$v] = 1;
    }
    return count($m);
}

/**
 * @param array<string, int> $m
 */
function compound(array $m): int
{
    $m['a'] += 1;
    $m['b']++;
    $m['c'] ??= 3;
    return count($m);
}

function unsetOthers(int $x): int
{
    unset($x);
    return 1;
}

/**
 * @param array<int, int> $m
 */
function stringKey(array $m, string $s): int
{
    $m[$s] = 1;
    return count($m);
}

/**
 * @param array<string, int> $m
 * @param array<string, int> $n
 */
function compare(array $m, array $n): bool
{
    return $m === $n;
}

/**
 * @param array<string, int> $m
 */
function show(array $m): void
{
    echo $m;
}

/**
 * @param ?array<string, int> $m
 */
function maybe(?array $m): int
{
    return $m['a'];
}

/**
 * @param ?array<string, int> $m
 */
function maybeWrite(?array $m): void
{
    $m['a'] = 1;
}

/**
 * @param array<string, int> $m
 * @param list<int> $l
 */
function checks(array $m, array $l, int $x): bool
{
    return isset($m['a'], $m['b']) || isset($x) || isset($l[0]) || array_key_exists(0, $l);
}

/**
 * @param array<string, array<string, int>> $m
 */
function nestedCoalesce(array $m): int
{
    return $m['a']['b'] ?? 0;
}
