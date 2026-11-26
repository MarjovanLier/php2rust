<?php

declare(strict_types=1);

/**
 * @param list<array<string, int>> $rows
 */
function mark(array $rows): int
{
    $rows[0]['seen'] = 1;
    return count($rows);
}

// PHP throws an ArgumentCountError: array_key_exists() takes exactly two arguments.
/**
 * @param array<string, int> $counts
 */
function known(array $counts): bool
{
    return array_key_exists('a');
}
