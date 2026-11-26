<?php

declare(strict_types=1);

function half(int $n): int
{
    if ($n % 2 !== 0) {
        throw new Exception('odd');
    }
    return intdiv($n, 2);
}

function safe(int $n): string
{
    try {
        return (string) half($n);
    } catch (Exception $_2) {
        return 'none';
    }
}

echo safe(4), "\n";
