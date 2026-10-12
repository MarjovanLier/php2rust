<?php

declare(strict_types=1);

enum Suit: string
{
    case Hearts = 'H';

    public function color(): string
    {
        return 'red';
    }
}

enum Level: int
{
    case Low = 1;
    case High = 1 + 1;
}
