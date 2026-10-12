<?php

declare(strict_types=1);

function code(int $n): ?int
{
    if ($n > 0) {
        return $n;
    }
    return null;
}

/** @return ?list<int> */
function digits(int $n): ?array
{
    if ($n > 0) {
        return [$n];
    }
    return null;
}

$a = code(1);
echo count(digits(0) ?? []), "\n";
$b = $a ?? code(2);
if ($b !== null) {
    echo "set\n";
}
