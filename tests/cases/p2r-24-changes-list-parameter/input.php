<?php

declare(strict_types=1);

/** @param list<int> $values */
function grow(array $values): int
{
    $values[] = 1;
    return count($values);
}

/** @param list<int> $values */
function replace(array $values): int
{
    $first = $values[0];
    $values = [$first];
    return count($values);
}

echo grow([5]), replace([6]), "\n";
