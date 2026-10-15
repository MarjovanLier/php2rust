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
 * An or-chain binds nothing, so the member use after it stops.
 */
function orChain(int|string $v, bool $b): int
{
    if (!is_string($v) || $b) {
        return 0;
    }
    return strlen($v);
}
