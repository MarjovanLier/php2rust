<?php

declare(strict_types=1);

/**
 * @param array<string, int>|string $value
 */
function asText(array|string $value): string
{
    return (string) $value;
}

/**
 * @param array<string, int>|string $value
 */
function truthy(array|string $value): bool
{
    if ($value) {
        return true;
    }
    return false;
}

/**
 * @param array<string, int>|string $value
 */
function isArray(array|string $value): bool
{
    return is_array($value);
}

/**
 * @param array<string, int>|string $value
 */
function size(array|string $value): int
{
    return count($value);
}

/**
 * @param array<string, int>|array<int, string> $value
 */
function twoMaps(array $value): int
{
    return 1;
}

/**
 * @param array<string, int>|string $value
 */
function total(array|string $value): int
{
    if (is_string($value)) {
        return 0;
    }
    $sum = 0;
    foreach ($value as $v) {
        $sum += $v;
    }
    return $sum;
}

/**
 * @param array<string, string>|bool $first
 */
function textMap(array|bool $first): int
{
    return 1;
}

/**
 * @param array<string, int>|bool $second
 */
function intMap(array|bool $second): int
{
    return 2;
}

/**
 * @return array<string, int>|string
 */
function literal(): array|string
{
    return ['a' => 1];
}
