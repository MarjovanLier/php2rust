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

function code(int $n): ?string
{
    if ($n > 0) {
        return 'H';
    }
    return null;
}

if (Status::from('Active') === Status::Active) {
    echo "a\n";
}
if (Suit::from(code(1)) === Suit::Hearts) {
    echo "b\n";
}
echo count(Suit::values()), "\n";
