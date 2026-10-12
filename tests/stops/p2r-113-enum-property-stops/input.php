<?php

declare(strict_types=1);

enum Suit: string
{
    case Hearts = 'H';
}

enum Status
{
    case Active;
}

function pick(int $n): ?Suit
{
    if ($n > 0) {
        return Suit::Hearts;
    }
    return null;
}

$s = Suit::Hearts;
echo Status::Active->value, "\n";
echo "{$s->name}\n";
echo pick(1)->name, "\n";
