<?php

declare(strict_types=1);

// PHP throws a TypeError in both functions, because strict types do not let an int pass
// as a string.
function pieces(int $n): int
{
    return count(explode(',', $n));
}

/**
 * @param list<string> $words
 */
function joined(int $n, array $words): string
{
    return implode($n, $words);
}
