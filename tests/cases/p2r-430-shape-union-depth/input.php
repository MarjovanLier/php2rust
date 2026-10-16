<?php

declare(strict_types=1);

namespace App;

enum Suit: string
{
    case Hearts = 'h';
    case Spades = 's';
}

// Union-field shapes at depth: a list field and a map field of them, an Option<U> field
// lent to a parameter, a list element read and its copy, and gettype() and is_scalar() on a
// variable that holds a field read.
/**
 * @param array{rows: list<array{id: int, v: int|string|null}>, meta: array<string, array{w: bool|float}>} $doc
 */
function walk(array $doc): string
{
    $out = '';
    foreach ($doc['rows'] as $row) {
        $out .= $row['id'] . ':' . kind($row['v']) . ' ';
        $v = $row['v'];
        $out .= gettype($v) . ' ';
    }
    $first = $doc['rows'][0];
    $copy = $first;
    $out .= kind($copy['v']) . ' ';
    foreach ($doc['meta'] as $m) {
        $w = $m['w'];
        $out .= gettype($w) . (is_scalar($w) ? 'S' : 'N') . ' ';
    }
    return $out;
}

function kind(int|string|null $v): string
{
    if ($v === null) {
        return 'null';
    }
    return is_int($v) ? 'int' : 'str';
}

// An enum as a member of a union field.
/**
 * @param array{s: Suit|int} $c
 */
function card(array $c): string
{
    $s = $c['s'];
    if (is_int($s)) {
        return 'n' . $s;
    }
    return gettype($s);
}

// A union-field shape as a member of a declared union.
/**
 * @param array{n: int|string}|bool $x
 */
function either(array|bool $x): string
{
    if (is_bool($x)) {
        return 'bool';
    }
    return gettype($x);
}

/**
 * @return array{n: int|string}
 */
function made(): array
{
    return ['n' => 'q'];
}

// A shape with union fields where a map of that union is needed (P2R-271).
/**
 * @param array{a: int|string, b: int|string} $s
 * @return array<string, int|string>
 */
function asMap(array $s): array
{
    return $s;
}

/**
 * @param array<string, int|string> $m
 */
function letters(array $m): string
{
    $out = '';
    foreach ($m as $value) {
        $out .= is_int($value) ? 'i' : 's';
    }
    return $out . count($m);
}

// A map with union values as the map member of a union field (P2R-270).
/**
 * @param array{opts: array<string, int|string>|string} $config
 */
function optsKind(array $config): string
{
    $o = $config['opts'];
    return gettype($o);
}

/**
 * @param array<string, int|string> $m
 */
function wrapped(array $m): string
{
    return optsKind(['opts' => $m]);
}

// A literal in a local variable, met before the declared shape it maps to.
function early(): string
{
    $pair = ['key' => 'k', 'value' => 7];
    return pairText($pair);
}

/**
 * @param array{key: string, value: int|bool} $pair
 */
function pairText(array $pair): string
{
    $value = $pair['value'];
    if (is_int($value)) {
        return $pair['key'] . '=' . $value;
    }
    return $pair['key'] . '=bool';
}

echo walk([
    'rows' => [['id' => 1, 'v' => 3], ['id' => 2, 'v' => null], ['id' => 3, 'v' => 'q']],
    'meta' => ['x' => ['w' => true], 'y' => ['w' => 2.5]],
]), "\n";
echo card(['s' => Suit::Hearts]), ' ', card(['s' => 4]), "\n";
echo either(true), ' ', either(made()), "\n";
$ab = ['b' => 'x', 'a' => 1];
echo letters(asMap($ab)), "\n";
echo optsKind(['opts' => 'plain']), ' ', wrapped(['a' => 1, 'b' => 'x']), "\n";
echo early(), "\n";
$late = ['key' => 'z', 'value' => true];
echo pairText($late), "\n";
