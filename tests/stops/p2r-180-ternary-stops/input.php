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

function same(?int $x): int
{
    return $x !== null ? $x : 0;
}

function maybe(): ?string
{
    return null;
}

final class Pair
{
    public function __construct(public string $n) {}
}

/** @return list<string> */
function names(): array
{
    return ['x'];
}

function either(bool $c, Pair $a, Pair $b): string
{
    $p = $c ? $a : $b;
    return $p->n;
}

function changes(bool $c, int $n, string $s): void
{
    $j = $c ? $n++ : 3;
    $k = $c ? 'a' . $s : 'b';
    echo $j, $k, "\n";
}

function temporaries(bool $c): void
{
    $a = $c ? (maybe() ?? '') : '';
    $b = $c ? (new Pair('a'))->n : '';
    $d = $c ? names()[0] : '';
    echo $a, $b, $d, "\n";
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
