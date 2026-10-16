<?php

declare(strict_types=1);

/**
 * A union stays out of lists (section 44 lets it into shape fields).
 *
 * @param list<int|string> $l
 */
function unionList(array $l): int
{
    return count($l);
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
