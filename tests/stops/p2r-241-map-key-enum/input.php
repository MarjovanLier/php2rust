<?php

declare(strict_types=1);

enum Kind: string
{
    case A = 'a';
}

// PHP throws a TypeError: an enum case cannot be an array offset.
/**
 * @param array<string, int> $counts
 */
function count_of(array $counts): int
{
    return $counts[Kind::A];
}
