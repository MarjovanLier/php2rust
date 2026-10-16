<?php

declare(strict_types=1);

namespace App;

interface Shape
{
    public function area(): int;
}

final readonly class Square implements Shape
{
    public function __construct(public int $side)
    {
    }

    public function area(): int
    {
        return $this->side * $this->side;
    }
}

// A value of the interface's type needs dyn or a generic, a later slice.
function measured(Shape $shape): int
{
    return $shape->area();
}

function made(): Shape
{
    return new Square(2);
}

function tested(Square $square): bool
{
    return $square instanceof Shape;
}

function named(): string
{
    return Shape::class;
}
