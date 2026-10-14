<?php

declare(strict_types=1);

namespace App\B;

use App\A\Counter;
use App\A\Tally;

enum Suit: string
{
    case Hearts = 'h';
    case Spades = 's';

    public function score(Tally $tally): void
    {
        if ($this === Suit::Hearts) {
            $tally->total += 10;
        } else {
            $tally->total += 1;
        }
    }

    public function shout(): string
    {
        return $this->value . '!';
    }
}

enum Color
{
    case Red;
    case Blue;
}

function bump(Counter $c): void
{
    $c->inc();
}
