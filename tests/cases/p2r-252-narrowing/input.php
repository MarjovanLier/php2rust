<?php

declare(strict_types=1);

enum Tone: string
{
    case Warm = 'warm';
    case Cool = 'cool';
}

function ownedText(int|string $v): string
{
    $w = $v;
    if (!is_string($w)) {
        return 'not text';
    }
    return $w . '!' . strlen($w);
}

function borrowedText(int|string $v, string $b): bool
{
    if (!is_string($v)) {
        return false;
    }
    return $v === $b;
}

function ownedInt(int|string $v): int
{
    $w = $v;
    if (!is_int($w)) {
        return -1;
    }
    return $w * 2;
}

function borrowedInt(int|string $v): int
{
    if (is_int($v)) {
        return $v + 1;
    }
    return 0;
}

function nullableText(int|string|null $v): string
{
    if (!is_string($v)) {
        return 'none';
    }
    return $v;
}

function nullableInt(int|string|null $v): int
{
    if (is_int($v) && $v > 2) {
        return $v;
    }
    return 0;
}

function chained(int|string $v, string $b): bool
{
    if (is_string($v) && strlen($v) > 1) {
        return $b === $v;
    }
    return false;
}

function matched(int|string $v): int
{
    if (!is_string($v)) {
        return 0;
    }
    return match ($v) {
        'x' => 1,
        default => 2,
    };
}

/**
 * @param array<string, int> $m
 */
function keyed(int|string $v, array $m): int
{
    if (!is_string($v)) {
        return -1;
    }
    $m[$v] = 9;
    return ($m[$v] ?? 0) + count($m);
}

function tone(Tone|int $v): string
{
    if (!is_int($v)) {
        return 'tone';
    }
    return 'int' . $v;
}

function afterNullCheck(int|string|null $v): int
{
    if ($v !== null) {
        if (is_int($v)) {
            return $v * 10;
        }
        return 1;
    }
    return 0;
}

function boolOrFloat(bool|float $v): string
{
    if (is_bool($v)) {
        return $v ? 'T' : 'F';
    }
    // Only a binding narrows: the return above leaves `$v` a union here.
    if (!is_float($v)) {
        return 'never';
    }
    return $v > 1.5 ? 'big' : 'small';
}

echo ownedText('ab'), ' ', ownedText(3), "\n";
echo borrowedText('cd', 'cd') ? 'y' : 'n', borrowedText(4, 'x') ? 'y' : 'n', "\n";
echo ownedInt(5), ' ', ownedInt('e'), ' ', borrowedInt(6), ' ', borrowedInt('f'), "\n";
echo nullableText('g'), ' ', nullableText(null), ' ', nullableText(7), "\n";
echo nullableInt(8), ' ', nullableInt(1), ' ', nullableInt(null), ' ', nullableInt('h'), "\n";
echo chained('ab', 'ab') ? 'y' : 'n', matched('x'), matched('y'), matched(3), "\n";
echo keyed('k', ['a' => 1]), keyed(2, []), "\n";
echo tone(Tone::Warm), tone(3), afterNullCheck(4), afterNullCheck('s'), afterNullCheck(null), "\n";
echo boolOrFloat(true), boolOrFloat(false), boolOrFloat(2.5), boolOrFloat(0.5), "\n";
