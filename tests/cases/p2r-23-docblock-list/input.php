<?php

declare(strict_types=1);

/**
 * @param list<int> $values
 * @return list<int>
 */
function withExtra(array $values, int $extra): array
{
    $copy = $values;
    $copy[] = $extra;
    return $copy;
}

/**
 * @param list<int> $values
 * @return list<int>
 */
function same(array $values): array
{
    return $values;
}

$numbers = withExtra([1, 2], 9);
$again = same($numbers);
echo count($numbers), ' ', $again[2], "\n";
