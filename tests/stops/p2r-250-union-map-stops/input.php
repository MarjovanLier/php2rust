<?php

declare(strict_types=1);

/**
 * A union stays out of lists and shape fields.
 *
 * @param list<int|string> $l
 */
function unionList(array $l): int
{
    return count($l);
}

/**
 * @param array{a: int|string} $s
 */
function unionField(array $s): int
{
    return 1;
}

/**
 * A map is no member of a union yet.
 *
 * @param array<string, string>|bool $m
 */
function mapMember(array|bool $m): int
{
    return 1;
}

/**
 * @param array<string, int|string> $m
 */
function wrongDefault(array $m): int|string
{
    return $m['k'] ?? 0;
}

/**
 * @param array<string, int|string> $m
 */
function memberUse(array $m): int
{
    return $m['k'] + 1;
}

/**
 * @param array<string, int|string> $m
 * @return array<string, int|string|null>
 */
function otherValues(array $m): array
{
    return $m;
}

/**
 * @param array<string, float|string> $m
 */
function intForFloat(array $m): int
{
    $m['k'] = 5;
    return count($m);
}
