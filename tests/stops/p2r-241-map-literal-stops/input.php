<?php

declare(strict_types=1);

/**
 * @return array<string, int>
 */
function noKey(): array
{
    return ['a' => 1, 2];
}

/**
 * @param array<string, int> $more
 * @return array<string, int>
 */
function spread(array $more): array
{
    return ['a' => 1, ...$more];
}

/**
 * @return array<array-key, int>
 */
function oddKeys(float $f, bool $b): array
{
    return [$f => 1, $b => 2, null => 3];
}

/**
 * @return array<array-key, int>
 */
function unionKey(int|string $k): array
{
    return [$k => 1];
}

/**
 * @return array<string, float>
 */
function intForFloat(): array
{
    return ['a' => 1];
}

/**
 * @return array<string, int>
 */
function twice(): array
{
    return ['a' => 1, 'a' => 2];
}
