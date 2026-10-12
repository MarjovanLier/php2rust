<?php

declare(strict_types=1);

enum Level: string
{
    case Low = 'low';
    case High = 'high';
}

function scale(int $n): float
{
    return match ($n) {
        0 => 0.5,
        default => $n,
    };
}

function label(string $t): ?string
{
    return match ($t) {
        'x' => null,
        default => $t,
    };
}

function weight(Level $level): int
{
    return match ($level->value) {
        'low' => 1,
        default => 2,
    };
}

function show(string $name): void
{
    $big = match ($name) {
        'a' => 3000000000,
        default => 4000000000,
    };
    echo $big * 3, "\n";
    $maybe = match ($name) {
        'b' => 5,
        default => null,
    };
    if ($maybe !== null) {
        echo $maybe * 1000000000, "\n";
    }
    $steps = match ($name) {
        'b' => [1000000, 2000000],
        default => [],
    };
    echo count($steps), "\n";
    $owned = $name . '!';
    $copy = match ($owned) {
        'b!' => $owned,
        default => 'none',
    };
    echo $copy, ' ', $owned, "\n";
}

show('a');
show('b');
echo label('y') ?? 'null', ' ', label('x') ?? 'null', ' ', weight(Level::High), "\n";
if (scale(0) < scale(1)) {
    echo "scaled\n";
}
