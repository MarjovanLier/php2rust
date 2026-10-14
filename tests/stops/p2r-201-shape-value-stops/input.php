<?php

declare(strict_types=1);

/**
 * @param array{a: int} $s
 * @param array{a: int} $t
 */
function compare(array $s, array $t): bool
{
    return $s === $t;
}

/**
 * @param array{a: int} $s
 */
function condition(array $s): int
{
    if ($s) {
        return 1;
    }
    return 0;
}

/**
 * @param array{a: int} $s
 */
function loop(array $s): int
{
    $total = 0;
    foreach ($s as $v) {
        $total += $v;
    }
    return $total;
}

/**
 * @param array{a: int} $s
 */
function counted(array $s): int
{
    return count($s);
}

/**
 * @return array{a: int}
 */
function written(): array
{
    $s = ['a' => 1];
    $s['a'] = 2;
    return $s;
}

/**
 * @return array{a: int}
 */
function appended(): array
{
    $s = ['a' => 1];
    $s[] = 2;
    return $s;
}

/**
 * @param array{a: int} $s
 */
function checked(array $s): bool
{
    return isset($s['a']);
}

/**
 * @param array{a: int} $s
 */
function coalesced(array $s): int
{
    return $s['a'] ?? 0;
}

/**
 * @param array{a: int} $s
 */
function assigned(array $s): int
{
    $s = ['a' => 2];
    return $s['a'];
}
