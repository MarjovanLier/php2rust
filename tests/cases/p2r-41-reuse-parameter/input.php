<?php

declare(strict_types=1);

function addAll(int $n): int
{
    $sum = $n;
    foreach ([1, 2] as $n) {
        $sum += $n;
    }
    return $sum;
}

/** @param list<int> $keys */
function keyTotal(int $i, array $keys): int
{
    $total = $i * 100;
    foreach ($keys as $i => $key) {
        $total += $i + $key;
    }
    return $total;
}

echo addAll(10), ' ', keyTotal(3, [5, 6]), "\n";
