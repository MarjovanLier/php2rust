<?php

declare(strict_types=1);

// PHP 8.6 also trims a form feed by default, where 8.3 to 8.5 keep it, so the default
// characters stop for 8.6, as a call and as a callback.
function edges(string $s): string
{
    return trim($s) . ltrim($s) . rtrim($s);
}

/**
 * @param list<string> $words
 * @return list<string>
 */
function cleaned(array $words): array
{
    return array_map(trim(...), $words);
}

// An argument that stops for every version still prints beside the 8.6 stop.
function wrong(): string
{
    return trim(5);
}

// With the characters given, every version removes the same bytes.
function named(string $s): string
{
    return trim($s, " \n");
}

echo edges(' x '), implode(',', cleaned([' a', 'b '])), named(" y\n"), "\n";
