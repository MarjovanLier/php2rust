<?php

declare(strict_types=1);

enum Suit
{
    case Hearts;
    case Spades;
}

function int_without_default(int $n): string
{
    return match ($n) {
        1 => 'one',
    };
}

function parity(int $n): string
{
    return match ($n % 2) {
        0 => 'even',
        1, -1 => 'odd',
    };
}

function bool_half(bool $b): string
{
    return match ($b) {
        true => 'yes',
    };
}

function enum_half(Suit $s): string
{
    return match ($s) {
        Suit::Hearts => 'red',
    };
}

function same_value(int $n, string $t): string
{
    $x = match ($n) {
        16, 0x10 => 'sixteen',
        default => 'other',
    };
    $y = match ($t) {
        'a', "a" => 'a',
        default => 'b',
    };
    return $x . $y;
}
