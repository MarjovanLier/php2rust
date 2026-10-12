<?php

declare(strict_types=1);

function code(int $n): ?int
{
    if ($n > 0) {
        return $n;
    }
    return null;
}

function name(int $n): ?string
{
    if ($n > 0) {
        return 'ann';
    }
    return null;
}

$c = code(0);
if ($c === null) {
    $c = 4;
}
$s = name(0);
if ($s === null) {
    $s = 'x';
}
echo count([$c, 7]), "\n";
echo strlen($s), "\n";
echo "$s!\n";
echo intdiv($c, 2), "\n";
$k = [1];
$k[] = $c;
