<?php

declare(strict_types=1);

final class Tools
{
    public static function up(string $s): string
    {
        return strtoupper($s);
    }
}

function pair(string $a, string $b): string
{
    return $a . $b;
}

function risky(string $s): string
{
    if ($s === '') {
        throw new InvalidArgumentException('empty');
    }
    return $s;
}

function nothing(string $s): void
{
    echo $s;
}

/**
 * @param list<?int> $maybe
 * @param list<string> $words
 */
function others(array $maybe, array $words): int
{
    $a = array_map(fn(?int $x): int => $x ?? 0, $maybe);
    $b = array_map(risky(...), $words);
    $c = array_map(nothing(...), $words);
    $d = array_map(...[trim(...), $words]);
    return count($a) + count($b) + count($c) + count($d);
}

/**
 * @param list<int> $xs
 * @return list<int>
 */
function widened(array $xs): array
{
    return array_map(fn(int $x): float => $x, $xs);
}

/**
 * @param list<int> $xs
 * @return list<int>
 */
function optional(array $xs): array
{
    return array_map(fn(int $x): ?int => $x, $xs);
}

/**
 * @param array<string, int> $map
 * @param list<string> $words
 * @param list<list<int>> $nested
 * @param list<int> $numbers
 */
function forms(array $map, array $words, array $nested, array $numbers): int
{
    $a = array_map(fn(int $x): int => $x, $map);
    $b = array_map(fn(string $x): string => $x, $words, $words);
    $c = array_map(null, $words);
    $d = array_map('trim', $words);
    $e = array_map(Tools::up(...), $words);
    $f = array_map(fn(string $x): string => $x, $numbers);
    $g = array_map(fn(array $x): int => count($x), $nested);
    $h = array_map(pair(...), $words);
    $i = array_map(strlen(...), $words);
    $j = array_map(array: $words, callback: trim(...));
    $u = ucwords(...);
    return count($a) + count($b) + count($c) + count($d) + count($e) + count($f) + count($g)
        + count($h) + count($i) + count($j) + strlen($u('a'));
}
