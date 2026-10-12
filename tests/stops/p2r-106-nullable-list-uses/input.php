<?php

declare(strict_types=1);

/** @return ?list<int> */
function digits(int $n): ?array
{
    if ($n > 0) {
        return [$n];
    }
    return null;
}

function code(int $n): ?int
{
    if ($n > 0) {
        return $n;
    }
    return null;
}

function flag(int $n): ?bool
{
    if ($n > 0) {
        return true;
    }
    return null;
}

$d = digits(1);
foreach ($d as $x) {
    echo $x, "\n";
}
echo count($d), "\n";
echo $d[0], "\n";
$d[] = 3;
$m = code(1);
echo -$m, "\n";
if (flag(1)) {
    echo "set\n";
}
