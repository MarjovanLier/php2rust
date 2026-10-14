<?php

declare(strict_types=1);

final readonly class Box
{
    public function __construct(public int $n) {}

    public static function make(int $n): Box
    {
        return new Box($n);
    }

    public function size(): int
    {
        return $this->n;
    }

    public function twice(): int
    {
        return $this->make(2)->n + self::size();
    }
}

function outside(): int
{
    return self::make(1)->n;
}

function viaName(string $class): int
{
    return $class::make(1)->n;
}

echo Box::size(), "\n";
echo Box::make(3)->twice(), "\n";
