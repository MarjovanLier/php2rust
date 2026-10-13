<?php

declare(strict_types=1);

readonly class Point
{
    public function __construct(public int $x)
    {
    }
}

function compare(Point $a, Point $b): bool
{
    return $a == $b;
}

function show(Point $a): void
{
    echo $a, "\n";
}

function duplicate(Point $a): Point
{
    return clone $a;
}

function kind(Point $a): bool
{
    return $a instanceof Point;
}

function truth(Point $a): int
{
    if ($a) {
        return 1;
    }
    return 0;
}
