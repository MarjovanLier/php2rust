<?php

declare(strict_types=1);

enum Suit
{
    case Hearts;
    case Spades;
}

enum Nothing
{
}

function plain(int $n): int
{
    return $n;
}

function shapes(int $n, ?int $m, float $f, int $other): void
{
    echo match ($n) {
        1 => 'one',
        default => 'many',
    };
    $a = plain(match ($n) {
        1 => 1,
        default => 2,
    });
    $b = match ($m) {
        1 => 1,
        default => 2,
    };
    $c = match ($f) {
        1.5 => 1,
        default => 2,
    };
    $d = match ($n) {
        $other => 1,
        default => 2,
    };
    echo $a + $b + $c + $d, "\n";
}

function defaults(Suit $s): int
{
    return match ($s) {
        default => 1,
        Suit::Hearts => 2,
        Suit::Spades => 3,
        default => 4,
    };
}

function empty_enum(Nothing $e): int
{
    return match ($e) {
        default => 5,
    };
}

function narrowed(Suit $s): int
{
    if ($s === Suit::Hearts) {
        return 0;
    }
    return match ($s) {
        Suit::Spades => 1,
        default => 2,
    };
}
