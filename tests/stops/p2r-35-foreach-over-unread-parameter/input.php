<?php

declare(strict_types=1);

/** @param list<int> $values */
function total(int $value, array $values): int
{
    $sum = 0;
    foreach ($values as $value) {
        $sum += $value;
    }
    return $sum;
}

/** @param list<int> $values */
function scaled(int $factor, array $values): int
{
    $sum = 0;
    echo $factor, ' ';
    foreach ($values as $factor) {
        $sum += $factor;
    }
    return $sum;
}

echo total(1, [2]), scaled(3, [4]), "\n";
