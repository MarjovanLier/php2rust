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

/** @param array{x: int}|bool $v */
function unionFirst(array|bool $v): int
{
    return 1;
}

/** @param array{bool_or_x: int} $s */
function shapeSecond(array $s): int
{
    return 1;
}

/** @param array{int_or_y: int} $s */
function shapeFirst(array $s): int
{
    return 1;
}

/** @param array{y: int}|int $v */
function unionSecond(array|int $v): int
{
    return 1;
}

final readonly class Cat {}

final readonly class Dog {}

final readonly class CatOrDog {}

function threeMembers(Cat|Dog|int $v): int
{
    return 1;
}

function twoMembers(CatOrDog|int $v): int
{
    return 1;
}

function trueFalse(true|false $v): int
{
    return 1;
}

/** @param numeric|bool $v */
function numericMember(int|float|string|bool $v): int
{
    return 1;
}

function objectMember(object|int $v): int
{
    return 1;
}

function sameClass(Cat|\Cat $v): int
{
    return 1;
}

final readonly class Card
{
    public function __construct(public int $n) {}

    public function same(self|Card $v): int
    {
        return 1;
    }
}

final readonly class Item {}

/** @param list<?Item>|string $v */
function nullableList(array|string $v): int
{
    return 1;
}
