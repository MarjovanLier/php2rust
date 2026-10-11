<?php

declare(strict_types=1);

function factorial(int $n): int
{
    if ($n <= 1) {
        return 1;
    }
    return $n * factorial($n - 1);
}

function fib(int $n): int
{
    if ($n < 2) {
        return $n;
    }
    return fib($n - 1) + fib($n - 2);
}

function gcd(int $a, int $b): int
{
    while ($b !== 0) {
        $t = $b;
        $b = $a % $b;
        $a = $t;
    }
    return $a;
}

for ($i = 0; $i <= 10; $i++) {
    echo fib($i), ' ';
}
echo "\n", factorial(10), ' ', gcd(48, 18), "\n";
echo -factorial(3) % 4, ' ', (5 - 9) * -gcd(12, 8), ' ', -(2 - 7) % 3, "\n";
