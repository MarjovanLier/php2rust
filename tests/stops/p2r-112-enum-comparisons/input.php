<?php

declare(strict_types=1);

enum Suit: string
{
    case Hearts = 'H';
    case Spades = 'S';
}

function first(Suit $a, Suit $b): string
{
    if ($a < $b) {
        return "a\n";
    }
    if ($a === 'H') {
        return "h\n";
    }
    return "b\n";
}
