<?php

declare(strict_types=1);

/**
 * @return array{a: int, b: int}
 */
function twice(): array
{
    return ['a' => 1, 'a' => 2, 'b' => 3];
}

/**
 * @return array{a: int}
 */
function mixed_keys(): array
{
    $k = 'a';
    return [$k => 1];
}

/**
 * @return array{a: int}
 */
function plain(int $n): array
{
    return ['a' => $n, 5];
}

/**
 * @param array{a: int} $s
 * @return array{a: int}
 */
function spread(array $s): array
{
    return ['a' => 1, ...$s];
}

/**
 * @return array{x: float}
 */
function widened(): array
{
    return ['x' => 1];
}
