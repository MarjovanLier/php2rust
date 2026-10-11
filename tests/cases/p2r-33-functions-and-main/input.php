<?php

declare(strict_types=1);

echo 'The answer is ', answer(6, 7), "\n";
greet('Ann');
echo square(-4) - 2 * (3 + 1), "\n";

function answer(int $sixTimes, int $seven): int
{
    return $sixTimes * $seven;
}

function greet(string $name): void
{
    echo 'Hello, ', $name, "\n";
}

function square(int $n): int
{
    return $n * $n;
}
