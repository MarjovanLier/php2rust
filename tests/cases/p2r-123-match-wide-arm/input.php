<?php

declare(strict_types=1);

function sentence(int $n): string
{
    return match ($n) {
        1 => 'a string of seventy-one characters that keeps this arm at one hundred c',
        default => 'short',
    };
}

function joined(int $n, string $first, string $second): string
{
    return match ($n) {
        1 => $first . ' and ' . $second . ' and ' . $first . ' and ' . $second . ' and more',
        default => 'short',
    };
}

echo sentence(1), ' ', sentence(2), "\n";
echo joined(1, 'x', 'y'), ' ', joined(2, 'x', 'y'), "\n";
