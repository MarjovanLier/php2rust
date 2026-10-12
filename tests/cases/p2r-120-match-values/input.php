<?php

declare(strict_types=1);

enum Suit
{
    case Hearts;
    case Spades;
}

function colour(Suit $s): string
{
    return match ($s) {
        Suit::Hearts => 'red',
        Suit::Spades => 'black',
    };
}

function word(int $n): string
{
    return match ($n) {
        1 => 'one',
        -1 => 'minus one',
        default => 'many',
    };
}

function sign(bool $b): int
{
    return match ($b) {
        true => 1,
        false => -1,
    };
}

function code(string $t): int
{
    $r = match ($t) {
        'a' => 10,
        default => 20,
    };
    return $r;
}

function literal(): int
{
    return match (3000000000) {
        default => 1,
    };
}

echo colour(Suit::Spades), ' ', word(1), ' ', word(-1), ' ', word(7), ' ', literal(), "\n";
echo sign(true) + sign(false), ' ', code('a'), ' ', code('b'), "\n";
