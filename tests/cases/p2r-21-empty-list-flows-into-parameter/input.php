<?php

declare(strict_types=1);

/** @param list<int> $values */
function size(array $values): int
{
    return count($values);
}

/** @return list<int> */
function none(): array
{
    $empty = [];
    return $empty;
}

$a = [];
echo size($a), ' ', count(none()), "\n";
