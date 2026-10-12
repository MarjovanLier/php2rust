<?php

declare(strict_types=1);

function code(int $n): ?int
{
    if ($n > 0) {
        return $n * 10;
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

function twice(?int $n): int
{
    if ($n === null) {
        return 0;
    }
    return $n * 2;
}

function shout(?string $s): string
{
    if (null === $s) {
        return "silence\n";
    }
    return $s . "!\n";
}

function size(?string $s): int
{
    if ($s === null) {
        return -1;
    }
    return 1;
}

echo twice(code(2)), ' ', twice(null), "\n";
echo shout(name(1)), shout(name(0));
echo size(name(1)), ' ', size(null), "\n";
$sum = 0;
foreach ([1, 0, 3] as $i) {
    $c = code($i);
    if ($c === null) {
        continue;
    }
    $sum += $c;
}
echo $sum, "\n";
$n = 3;
while (true) {
    $s = name($n);
    if ($s === null) {
        break;
    }
    echo $s, strlen($s), "\n";
    $n = $n - 1;
}
