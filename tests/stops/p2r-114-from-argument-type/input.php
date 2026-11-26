<?php

declare(strict_types=1);

enum Suit: string
{
    case Hearts = 'H';
}

// Under strict_types PHP throws a TypeError: Suit is backed by string.
function parsed(int $code): ?Suit
{
    return Suit::tryFrom($code);
}

echo parsed(1) === null ? "none\n" : "some\n";
