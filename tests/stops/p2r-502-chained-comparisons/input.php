<?php

declare(strict_types=1);

// PHP fails to parse a comparison chained to another of its level, which Mago reads
// (P2R-502). Main converted $a == $b == $c to a == (b == c).
function same(bool $a, bool $b, bool $c): bool
{
    return $a == $b == $c;
}

function strict(bool $a, bool $b, bool $c): bool
{
    return $a === $b !== $c;
}

function ordered(int $a, int $b): bool
{
    return $a < $b < 3;
}

function bounded(int $a, int $b, int $c): bool
{
    return $a >= $b <= $c;
}

// A comparison of the other level parses: PHP reads ($a < $b) == $c and $c == ($a < $b).
function mixedLeft(int $a, int $b, bool $c): bool
{
    return $a < $b == $c;
}

function mixedRight(bool $c, int $a, int $b): bool
{
    return $c == $a < $b;
}

echo mixedLeft(1, 2, true) ? 'y' : 'n', mixedRight(true, 1, 2) ? 'y' : 'n', "\n";
