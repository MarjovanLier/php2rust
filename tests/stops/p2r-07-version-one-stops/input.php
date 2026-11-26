<?php

declare(strict_types=1);

final readonly class Point
{
    public function __construct(public int $x) {}
}

/**
 * @param list<int> $a
 * @param list<int> $b
 */
function sameList(array $a, array $b): bool
{
    return $a === $b;
}

function flags(int $a, int $b): int
{
    return $a & $b;
}

// PHP throws an ArgumentCountError: the constructor needs one argument.
$p = new Point;
echo $p->x, "\n";
echo sameList([1], [1]) ? "same\n" : "different\n";
echo flags(6, 3), "\n";
return;
