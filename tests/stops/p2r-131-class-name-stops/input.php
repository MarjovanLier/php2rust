<?php

declare(strict_types=1);

readonly class Cloner
{
    public function __construct(public int $x)
    {
    }

    public function clone(): int
    {
        return $this->x;
    }

    public function new(): int
    {
        return 1;
    }

    public function size(): int
    {
        return 1;
    }

    public function SIZE(): int
    {
        return 2;
    }
}

readonly class Countable
{
}

readonly class Clone
{
}

enum Shade
{
    case Dark;
}

readonly class shade_
{
}

readonly class Twin
{
    public function __construct(public int $first_name, public int $firstName)
    {
    }
}
