<?php

declare(strict_types=1);

// PHP throws an Error: a string has no methods.
function shout(string $text): string
{
    return $text->upper();
}

echo shout('a'), "\n";
