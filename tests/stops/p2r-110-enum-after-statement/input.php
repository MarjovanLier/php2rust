<?php

declare(strict_types=1);

function make(): void
{
    enum Inner
    {
        case One;
    }
}

echo "start\n";

enum Suit
{
    case Hearts;
}
