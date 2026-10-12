<?php

declare(strict_types=1);

function code(int $n): ?int
{
    if ($n > 0) {
        return $n;
    }
    return null;
}

function show(?int $n): string
{
    if ($n === null) {
        return "none\n";
    }
    return "some\n";
}

/** @param list<?int> $xs */
function nulls(array $xs): int
{
    $count = 0;
    foreach ($xs as $x) {
        if ($x === null) {
            $count += 1;
        }
    }
    return $count;
}

function later(): ?float
{
    return 2;
}

$v = null;
if (code(1) !== null) {
    $v = 5;
}
echo show($v);
$w = 0;
echo show($w);
$w = code(0);
echo show($w);
$xs = [1, null, 3];
echo nulls($xs), "\n";
$ys = [1, null];
$ys[] = 4;
$ys[] = null;
echo nulls($ys), "\n";
if (later() !== null) {
    echo "later\n";
}
