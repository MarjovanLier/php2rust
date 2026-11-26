<?php

declare(strict_types=1);

/**
 * @param array<string, int> $counts
 */
function store(array $counts, ?int $n): int
{
    $counts['a'] = $n;
    return count($counts);
}

/**
 * @param array<string, array{a: int}> $rows
 */
function storeRow(array $rows, ?int $n): int
{
    $rows['k'] = ['a' => $n];
    return count($rows);
}

/**
 * @param array<string, int> $counts
 */
function storePicked(array $counts, bool $b, ?int $n): int
{
    $counts['a'] = $b ? $n : 0;
    return count($counts);
}
