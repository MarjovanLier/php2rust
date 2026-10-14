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
