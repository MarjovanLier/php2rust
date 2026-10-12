<?php

declare(strict_types=1);

function half(?float $f): ?float
{
    return $f;
}

/** @param ?list<int> $xs */
function known(?array $xs): string
{
    if ($xs === null) {
        return "no list\n";
    }
    return "list\n";
}

function show(?int $n): string
{
    if ($n === null) {
        return "none\n";
    }
    return "some\n";
}

/** @param list<?int> $xs */
function size(array $xs): int
{
    return count($xs);
}

echo show(5), show(null);
echo known([1, 2, 3]), known(null);
if (half(4) !== null) {
    echo "half\n";
}
echo size([1, null, 3]), "\n";
