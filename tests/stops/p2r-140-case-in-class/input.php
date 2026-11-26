<?php

declare(strict_types=1);

// PHP rejects a case in a class when it compiles the file, and Mago parses it, so this
// folder pins php2rust's own check.
final class Suit
{
    case Hearts;

    public function name(): string
    {
        return 'hearts';
    }
}

echo (new Suit())->name(), "\n";
