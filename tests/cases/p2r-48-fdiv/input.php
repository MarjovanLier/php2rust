<?php

declare(strict_types=1);

function rate(float $part, float $whole): float
{
    return fdiv($part, $whole);
}

function check(bool $ok): void
{
    if ($ok) {
        echo 'y';
    } else {
        echo 'n';
    }
}

$n = 3;
check(fdiv(7, 2) === 3.5);
check(rate(1.0, 4.0) === 0.25);
check(fdiv($n, 2) === 1.5);
check(fdiv(1, $n) < 0.5);
check(2.0 * fdiv(1.0, 4.0) === 0.5);
check(fdiv(1, 0) > 1e308);
check(fdiv(-1, 0) < -1e308);
check(fdiv(1, -0) > 0.0);
check(fdiv(0, 0) !== fdiv(0, 0));
check(fdiv(0, 0) === fdiv(0, 0));
echo "\n";
