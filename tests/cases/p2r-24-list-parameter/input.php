<?php

declare(strict_types=1);

/** @param list<int> $values */
function total(array $values): int
{
    $sum = $values[0] + $values[1];
    return $sum + count($values);
}

/** @param list<int> $values */
function twice(array $values): int
{
    return total($values) * 2;
}

/** @param list<list<int>> $rows */
function width(array $rows, int $row): int
{
    return count($rows[$row]) + total($rows[$row]);
}

$scores = [4, 5];
echo total($scores), ' ', twice([1, 2, 3]), ' ';
echo width([[1, 2], [3, 4, 5]], 1), ' ', count($scores), "\n";
