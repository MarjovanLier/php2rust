<?php

declare(strict_types=1);

namespace App;

// A union as a list element type still stops (P2R-230).
/**
 * @param list<int|string> $values
 */
function count2(array $values): int
{
    return count($values);
}

// A union value is no string to echo, and no value to compare (P2R-232).
/**
 * @param array{key: string, value: int|string} $pair
 */
function shown(array $pair): void
{
    echo $pair['value'];
}

/**
 * @param array{key: string, value: int|string} $pair
 */
function same(array $pair): bool
{
    return $pair['value'] === 'x';
}

// Two declared shapes of one name with other field types (P2R-200).
/**
 * @param array{key: string, value: bool|string} $pair
 */
function other(array $pair): string
{
    return $pair['key'];
}

final readonly class Point
{
    public function __construct(public int $x)
    {
    }
}

// A union field with a class member holds a class (P2R-200).
/**
 * @param array{at: Point|int} $place
 */
function placed(array $place): int
{
    return 1;
}

// A field that is a map with union values stops (P2R-250).
/**
 * @param array{opts: array<string, int|string>} $config
 */
function configured(array $config): int
{
    return 1;
}

// A shape with a union field whose keys no declared shape has keeps P2R-22.
function local(bool $flag): string
{
    $pair = ['name' => 'k', 'value' => $flag ? 1 : 'x'];
    return $pair['name'];
}
