<?php

declare(strict_types=1);

/**
 * @return array<string, string>
 */
function orders(bool $flip, ?string $x): array
{
    // The same shape built in two key orders: PHP keeps each literal's order.
    $pair = ['a' => 'A', 'b' => 'B'];
    $other = $x === null ? $pair : ['b' => $x, 'a' => 'A'];
    return $flip ? $other : $pair;
}

/**
 * @return array<string, array{c: string, d: string}>
 */
function viaMapValue(): array
{
    return ['first' => ['c' => '1', 'd' => '2'], 'second' => ['d' => '3', 'c' => '4']];
}

/**
 * @param array{g: string, h: string}|null $maybe
 *
 * @return array<string, string>
 */
function viaDefault(?array $maybe): array
{
    $two = ['g' => '1', 'h' => '2'];
    echo $two['g'];
    return $maybe ?? ['h' => 'y', 'g' => 'x'];
}

/**
 * @return array<string, string>
 */
function intField(): array
{
    $mixed = ['n' => 1, 's' => 'one'];
    return $mixed;
}

/**
 * @return array<int, string>
 */
function intKeys(): array
{
    $shape = ['p' => 'P', 'q' => 'Q'];
    return $shape;
}

/**
 * @param array{x: string, y: string} $given
 *
 * @return array<string, string>
 */
function noLiteral(array $given): array
{
    return $given;
}
