<?php

declare(strict_types=1);

function flag(int $n): bool
{
    return $n % 2 === 0;
}

function grid(int $size): int
{
    $total = 0;
    for ($row = 0; $row < $size; $row++) {
        if ($row === 1) {
            continue;
        }
        for ($col = 0; $col < $size; $col++) {
            if ($col === $row) {
                continue;
            }
            $cell = $row * 10 + $col;
            $total += $cell;
        }
        if (flag($row)) {
            continue;
        }
        $total += 1000;
    }
    return $total;
}

function search(int $limit): int
{
    $i = 0;
    while (true) {
        $i++;
        if ($i > $limit) {
            $i = -1;
            break;
        }
        if ($i * $i === 49) {
            break;
        }
    }
    return $i;
}

function nested(int $n): int
{
    $outer = 0;
    $count = 0;
    while ($outer < $n) {
        $outer++;
        $inner = 0;
        while ($inner < $outer) {
            $inner++;
            if ($inner === 2) {
                $last = $inner;
            } else {
                $last = 0;
            }
            $count += $last;
        }
    }
    return $count;
}

$steps = 0;
for ($a = 0, $b = 3; $a < 5; $a++, $steps++) {
    if ($a === $b) {
        continue;
    }
    if ($a > 3) {
        break;
    }
}
echo grid(4), ' ', search(10), ' ', search(3), ' ', nested(4), ' ', $steps, "\n";
