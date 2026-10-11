<?php

declare(strict_types=1);

function band(int $score): int
{
    if ($score >= 90) {
        $result = 1;
    } elseif ($score >= 75) {
        $result = 2;
    } else if ($score >= 50) {
        $result = 3;
    } else {
        $result = 4;
    }
    return $result;
}

function sign(int $n): int
{
    if ($n < 0) {
        return -1;
    } elseif ($n === 0) {
        return 0;
    } else {
        return 1;
    }
}

function describe(int $n): void
{
    if ($n > 100) echo "big ";
    if ($n % 2 === 0):
        echo "even\n";
    elseif ($n % 3 === 0):
        echo "three\n";
    else:
        $half = $n - 1;
        echo $half, "\n";
    endif;
}

echo band(95), band(80), band(60), band(10), "\n";
echo sign(-5), ' ', sign(0), ' ', sign(7), "\n";
describe(200);
describe(9);
describe(7);
