<?php

declare(strict_types=1);

final readonly class Text
{
    public function __construct(public string $php_offset) {}
}

function php_int(): int
{
    return 1;
}

function size(string $ctype_digit): int
{
    return strlen($ctype_digit);
}

/**
 * @param list<string> $words
 */
function longest(array $words): int
{
    $max = 0;
    foreach ($words as $phpSubstr) {
        $max = strlen($phpSubstr);
    }
    return $max;
}

function local(): int
{
    $php_offset = 1;
    return $php_offset;
}
