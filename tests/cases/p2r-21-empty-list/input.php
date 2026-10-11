<?php

declare(strict_types=1);

/** @return list<int> */
function none(): array
{
    return [];
}

/** @param list<int> $values */
function size(array $values): int
{
    return count($values);
}

$found = [];
$found[] = 4;
$found[] = 5;
/** @var list<int> $later */
$later = [];
echo count($found), ' ', $found[1], ' ', count($later), ' ';
echo count(none()), ' ', size([]), "\n";
