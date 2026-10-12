<?php

declare(strict_types=1);

function step(int $total, bool $big): int
{
    if ($big) {
        $total = match ($total) {
            1 => $total + 100,
            default => $total + 1000,
        };
    }
    return $total;
}

function sentence(int $n): string
{
    return match ($n) {
        1 => 'a string of seventy characters that keeps this arm at one hundred cols',
        default => 'short',
    };
}

echo step(1, true), ' ', step(2, true), ' ', step(3, false), "\n";
echo sentence(1), ' ', sentence(2), "\n";
