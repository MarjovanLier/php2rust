<?php

declare(strict_types=1);

final readonly class Point
{
    public function __construct(
        public int $x,
    ) {}

    public function twice(): int
    {
        return $this->x * 2;
    }
}

/**
 * @param list<?Point> $points
 */
function first(array $points): int
{
    return count($points);
}

function read(?Point $p): ?int
{
    return $p->x;
}

function call(?Point $p): int
{
    return $p->twice();
}

function safe(?Point $p): ?int
{
    return $p?->x;
}

function same(?Point $a, ?Point $b): bool
{
    return $a === $b;
}

function cleared(?Point $p): bool
{
    $p = null;
    return $p === null;
}

final class Cell
{
    public int $n = 0;
}

function cell(?Cell $c): bool
{
    return $c === null;
}
