<?php

declare(strict_types=1);

/**
 * PHP stores an int key and a decimal string key the same way.
 *
 * @return array<array-key, int>
 */
function keyed(string $a, string $b): array
{
    return [$a => 1, $b => 2, -0 => 3, 0x10 => 4, 7 => 5];
}

/**
 * @return array<int, string>
 */
function names(int $n): array
{
    return [3 => 'c', 1 => 'a', $n => 'n'];
}

/**
 * @return array<string, ?float>
 */
function prices(bool $known): array
{
    return $known ? ['tea' => 1.5, 'cake' => null] : [];
}

/**
 * @return array<string, int>
 */
function pick(int $n): array
{
    return match ($n) {
        1 => ['one' => 1],
        default => [],
    };
}

/**
 * @param array<array-key, int> $m
 */
function show(array $m): string
{
    $out = '';
    foreach ($m as $v) {
        $out .= $v . ';';
    }
    return $out . count($m);
}

/**
 * @param array<int, string> $m
 */
function showInts(array $m): string
{
    $out = '';
    foreach ($m as $k => $v) {
        $out .= $k . '=' . $v . ';';
    }
    return $out;
}

// A key that repeats at run time keeps its first place and its last value.
echo show(keyed('x', 'x')), "\n";
echo show(keyed('x', '16')), "\n";
echo showInts(names(3)), "\n";
echo showInts(names(9)), "\n";
echo count(prices(true)), ' ', count(prices(false)), "\n";
echo count(pick(1)), ' ', count(pick(2)), "\n";
