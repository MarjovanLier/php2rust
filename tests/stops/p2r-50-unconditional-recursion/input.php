<?php

declare(strict_types=1);

function countUp(int $n): void
{
    echo $n;
    countUp($n + 1);
}

function bothWays(int $n): int
{
    if ($n > 0) {
        return bothWays($n - 1);
    } elseif ($n < 0) {
        return 1 + bothWays($n + 1);
    } else {
        echo 'zero ', bothWays($n), "\n";
        return 0;
    }
}

function spin(int $n): void
{
    while (true) {
        spin($n);
    }
}

function stepper(int $n): void
{
    for (;; stepper($n)) {
        echo $n;
    }
}

function asked(int $n): bool
{
    if (asked($n - 1)) {
        return true;
    }
    return false;
}
