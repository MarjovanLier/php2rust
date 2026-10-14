<?php

declare(strict_types=1);

function compare(int|string $a, int|string $b): bool
{
    return $a === $b;
}

function printed(int|string $a): void
{
    echo $a, "\n";
}

function narrowed(int|string $v): int
{
    if (is_string($v)) {
        return strlen($v);
    }
    return 0;
}

function joined(int|string $v): string
{
    return 'v=' . $v;
}

function condition(int|string|null $v): bool
{
    if ($v) {
        return true;
    }
    return false;
}

function counted(int|string $v): int
{
    return $v + 1;
}

function subject(int|string $v): int
{
    return match ($v) {
        1 => 1,
        default => 0,
    };
}

function cast(int|string $v): int
{
    return (int) $v;
}
