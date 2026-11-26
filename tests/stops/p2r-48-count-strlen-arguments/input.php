<?php

declare(strict_types=1);

/**
 * @param list<int> $list
 */
function total(array $list): int
{
    return count($list, COUNT_RECURSIVE);
}

// PHP throws a TypeError here, because strict types do not let an int pass as a string.
function digits(int $n): int
{
    return strlen($n);
}

// PHP throws an ArgumentCountError here: strlen() takes one argument.
function width(string $s): int
{
    return strlen($s, 2);
}
