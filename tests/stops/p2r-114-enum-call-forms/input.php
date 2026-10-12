<?php

declare(strict_types=1);

enum Suit: string
{
    case Hearts = 'H';
}

enum Nothing: string
{
}

if (Suit::from(value: 'H') === Suit::Hearts) {
    echo "named\n";
}
if (Nothing::from('a') === Nothing::from('b')) {
    echo "never\n";
}
