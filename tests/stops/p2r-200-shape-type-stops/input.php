<?php

declare(strict_types=1);

final readonly class Point
{
    public function __construct(
        public int $x,
    ) {}
}

enum DayShape
{
    case One;
}

/**
 * @param array{a?: int} $s
 */
function optional(array $s): int
{
    return 1;
}

/**
 * @param array{0: int, 1: string} $s
 */
function numbered(array $s): int
{
    return 1;
}

/**
 * @param array{a: int, ...<string, int>} $s
 */
function open(array $s): int
{
    return 1;
}

/**
 * @param array{'2x': int} $s
 */
function digits(array $s): int
{
    return 1;
}

/**
 * @param array{fooBar: int, foo_bar: int} $s
 */
function same(array $s): int
{
    return 1;
}

/**
 * @param array{p: Point} $s
 */
function holds(array $s): int
{
    return 1;
}

/**
 * @param array{day: int} $s
 */
function named(array $s): int
{
    return 1;
}

/**
 * @param array{a_b: int} $s
 */
function first(array $s): int
{
    return 1;
}

/**
 * @param array{a: int, b: int} $s
 */
function second(array $s): int
{
    return 1;
}

/**
 * @param array{n: mixed} $s
 */
function loose(array $s): int
{
    return 1;
}
