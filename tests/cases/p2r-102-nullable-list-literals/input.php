<?php

declare(strict_types=1);

/** @param list<?int> $xs */
function size(array $xs): int
{
    return count($xs);
}

/** @param list<?string> $xs */
function names(array $xs): int
{
    return count($xs);
}

/** @return ?list<int> */
function nothing(bool $none): ?array
{
    if ($none) {
        return null;
    }
    return [];
}

/** @param ?list<int> $xs */
function known(?array $xs): string
{
    if ($xs === null) {
        return "no list\n";
    }
    return "list\n";
}

foreach ([null, 2147483647] as $y) {
    if ($y !== null) {
        echo $y + 1, "\n";
    }
}
echo count([null, 1]), "\n";
echo size([1, 2]), ' ', names([null, null]), "\n";
echo known(nothing(false)), known(nothing(true)), known([]);
