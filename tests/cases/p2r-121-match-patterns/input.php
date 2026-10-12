<?php

declare(strict_types=1);

enum Suit
{
    case Hearts;
    case Spades;
    case Clubs;
}

function size(int $n): string
{
    return match ($n) {
        default => 'large',
        1, 2, 0x3 => 'small',
        4 => 'medium',
    };
}

function colour(Suit $s): string
{
    return match ($s) {
        Suit::Hearts => 'red',
        Suit::Spades, Suit::Clubs => 'black',
        default => throw new LogicException('never'),
    };
}

function yes(bool $b): string
{
    return match ($b) {
        true => 'yes',
        false => 'no',
        default => 'never',
    };
}

function vowel(string $c): bool
{
    return match ($c) {
        'a', 'e', 'i', 'o', 'u' => true,
        default => false,
    };
}

echo size(2), ' ', size(3), ' ', size(4), ' ', size(9), "\n";
echo colour(Suit::Clubs), ' ', colour(Suit::Hearts), ' ', yes(false), "\n";
if (vowel('e')) {
    echo "vowel\n";
}
