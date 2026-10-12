<?php

declare(strict_types=1);

function fact(int $n): int
{
    return match ($n) {
        0 => 1,
        default => $n * fact($n - 1),
    };
}

function pick(string $s, int $n): string
{
    $first = $s . '1';
    $second = $s . '2';
    $picked = match ($n) {
        1 => $first,
        2 => $second,
        default => $first . $second,
    };
    return $picked . ' ' . $first;
}

echo fact(5), "\n";
echo pick('a', 1), ' ', pick('b', 2), ' ', pick('c', 3), "\n";
