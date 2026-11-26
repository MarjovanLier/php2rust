<?php

declare(strict_types=1);

// PHP rejects a property in an enum when it compiles the file, and Mago parses it, so this
// folder pins php2rust's own check.
enum Suit: string
{
    case Hearts = 'H';

    public int $rank;
}

echo Suit::Hearts->value, "\n";
