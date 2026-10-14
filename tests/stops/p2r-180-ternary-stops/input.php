<?php

declare(strict_types=1);

/** @return list<int> */
function lists(bool $c): array
{
    return $c ? [1] : [2];
}

function nullable(bool $c): ?string
{
    return $c ? 'a' : null;
}

function plus(?int $x): int
{
    return $x !== null ? $x + 1 : 0;
}

function kind(int $n): string
{
    return match ($n) {
        1 => $n > 0 ? 'a' : 'b',
        default => 'c',
    };
}

function run(int $n, string $s, bool $on): void
{
    $a = $n ?: 3;
    $b = $on ? ($n > 2 ? 1 : 2) : 3;
    $c = ($on ? true : false) ? 1 : 2;
    $d = $on ? 1 : 'one';
    $e = $on ? 'abcdefghijklmn' : 'nopqrstuvwxyz';
    $f = $on ? strtolower($s) : $s;
    $g = $n ? 1 : 2;
    $h = $on ? $n = 2 : 3;
    $i = $on ? 5 : null;
    echo $a, $b, $c, $d, $e, $f, $g, $h, $i, "\n";
}
