<?php

declare(strict_types=1);

function mixed_arms(int $n): void
{
    $v = match ($n) {
        1 => 1,
        default => 'one',
    };
    echo $v, "\n";
}

function null_arm(int $n): int
{
    return match ($n) {
        1 => null,
        default => 2,
    };
}
