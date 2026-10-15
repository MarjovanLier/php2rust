<?php

declare(strict_types=1);

function tests(int|string|null $v): string
{
    return (is_int($v) ? 'i' : '-') . (is_string($v) ? 's' : '-') . (!is_int($v) ? 'n' : '-');
}

function floats(bool|float $v): string
{
    return (is_bool($v) ? 'b' : '-') . (\is_float($v) ? 'f' : '-');
}

/**
 * A test whose body does not always leave binds nothing.
 */
function noLeave(int|string $v): string
{
    $out = 'x';
    if (!is_string($v)) {
        $out = 'y';
    }
    return $out;
}

/**
 * @param array<string, int|string> $m
 */
function element(array $m): string
{
    return is_string($m['k']) ? 'text' : 'number';
}

echo tests(1), ' ', tests('i'), ' ', tests(null), "\n";
echo floats(true), ' ', floats(1.5), "\n";
echo noLeave('j'), noLeave(9), "\n";
echo element(['k' => 'a']), ' ', element(['k' => 2]), "\n";
