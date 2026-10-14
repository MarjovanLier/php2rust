<?php

declare(strict_types=1);

final readonly class Point
{
    public function __construct(public int $x) {}

    public function above(parent $other): bool
    {
        return true;
    }

    public function same(static $other): bool
    {
        return $this->x === $other->x;
    }

    public function either(int|static $other): bool
    {
        return true;
    }
}

function origin(): self
{
    return new Point(0);
}

echo (new Point(1))->x, "\n";
