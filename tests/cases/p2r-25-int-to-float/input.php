<?php

declare(strict_types=1);

function scale(float $value, float $factor): float
{
    return $value * $factor;
}

function whole(int $n): float
{
    return $n;
}

function four(): float
{
    return 4;
}

function check(float $value, float $expected): void
{
    if ($value === $expected) {
        echo 'same';
    } else {
        echo 'different';
    }
    echo "\n";
}

$count = 3;
check(scale(2, 1.5), 3.0);
check(scale($count, 0.5), 1.5);
check(scale(-2, $count + 1), -8.0);
check(whole(7), 7.5);
check(four(), 4.0);
check(whole(9007199254740993), 9007199254740992.0);
check(scale(9007199254740993, 1.0), 9007199254740992.0);
