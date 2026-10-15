<?php

declare(strict_types=1);

function writeAfterGuard(int|string $v): string
{
    if (!is_string($v)) {
        return '';
    }
    $v = 'changed';
    return $v;
}

function writeInBody(int|string $v): string
{
    if (is_string($v)) {
        $v = 5;
    }
    return 'x';
}

function reference(int|string $v): int
{
    $r = &$v;
    return 1;
}

/**
 * The step also runs at each `continue`, where the binding may not exist yet.
 */
function forStep(int|string $v): int
{
    $n = 0;
    for ($i = 0; $i < 3; $i++, $v = $i) {
        if (!is_int($v)) {
            continue;
        }
        $n += $v;
    }
    return $n;
}

/**
 * A return narrows nothing: `$v` is still the union in the list and the append.
 *
 * @return list<string>
 */
function listed(int|string $v): array
{
    if (is_int($v)) {
        return [];
    }
    $xs = [];
    $xs[] = $v;
    return [$v];
}

/**
 * An or-chain binds nothing, so the member use after it stops.
 */
function orChain(int|string $v, bool $b): int
{
    if (!is_string($v) || $b) {
        return 0;
    }
    return strlen($v);
}
