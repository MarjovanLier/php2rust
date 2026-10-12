<?php

declare(strict_types=1);

enum Suit
{
    case Hearts;
    case Spades;
}

function colour(Suit $s): string
{
    $fallback = 'never';
    return match ($s) {
        Suit::Hearts => 'red',
        Suit::Spades => 'black',
        default => $fallback,
    };
}

function maybe_set(int $n): string
{
    if ($n > 0) {
        $y = 'set';
    }
    return match ($n) {
        1 => $y,
        default => 'none',
    };
}
