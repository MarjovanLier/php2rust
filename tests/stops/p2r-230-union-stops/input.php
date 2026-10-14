<?php

declare(strict_types=1);

/** @param int|string $x */
function nativeFloat(float|string $x): string
{
    return gettype($x);
}

/** @param float $x */
function docFloat(int|float $x): string
{
    return gettype($x);
}

/** @param float $x */
function mixedFloat(mixed $x): string
{
    return gettype($x);
}

function twice(int|string|INT $x): string
{
    return gettype($x);
}

function falseBool(bool|false|string $x): string
{
    return gettype($x);
}

/** @param list<int>|list<string> $x */
function lists(array $x): int
{
    return 1;
}

/** @param list<int|string> $x */
function listOfUnion(array $x): int
{
    return 1;
}

final class Counter
{
    public int $n = 0;

    public function bump(): int
    {
        $this->n += 1;
        return $this->n;
    }
}

function changing(Counter|int $c): int
{
    return 1;
}

final readonly class Node
{
    public function __construct(public Node|int $next) {}
}

final readonly class BoolOrInt
{
    public function __construct(public int $n) {}
}

function named(bool|int $v): int
{
    return 1;
}
