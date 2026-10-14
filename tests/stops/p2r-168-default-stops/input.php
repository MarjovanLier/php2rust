<?php

declare(strict_types=1);

enum Kind
{
    case A;
    case B;

    const FIRST = 1;

    public static function pick(Kind $k = static::A): int
    {
        return 1;
    }

    public static function first(int $n = self::FIRST): int
    {
        return $n;
    }
}

final readonly class Point
{
    public function __construct(public int $x = null) {}
}

function sum(int $x = 1 + 2): int
{
    return $x;
}

function implicit(int $x = null): int
{
    return 1;
}

function early(int $x = 1, int $y): int
{
    return $x + $y;
}

function joined(string $s = "a{$x}"): string
{
    return $s;
}

function made(Kind $k = new Kind()): int
{
    return 1;
}

/**
 * @param list<int> $xs
 */
function listed(array $xs = [1], float $f = '1'): int
{
    return 1;
}

function byReference(int &$n = 1, int ...$rest): int
{
    return $n;
}
