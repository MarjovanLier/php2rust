<?php

declare(strict_types=1);

final readonly class Tag
{
    public function __construct(public string $text)
    {
    }

    public function shout(): string
    {
        return strtoupper($this->text);
    }
}

// PHP 8.3 fails to parse a call or a property read on new without parentheses around it;
// PHP 8.4 added both, and Mago parses them for every version.
echo new Tag('a')->shout(), "\n";
echo new Tag('b')->text, "\n";
// With parentheses every version parses it.
echo (new Tag('c'))->shout(), "\n";
