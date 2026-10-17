<?php

declare(strict_types=1);

// A comment that makes Mago drop a finding stops, because the finding may be one PHP
// enforces (P2R-501): a line comment, a docblock, a block comment and a # comment.
function first(int $value): int
{
    // @mago-expect analysis:redundant-cast
    return (int) $value;
}

/**
 * @mago-ignore analysis:redundant-cast
 */
function second(int $value): int
{
    return (int) $value;
}

function third(int $value): int
{
    /* @mago-expect analysis:redundant-cast */
    return $value + 1;
}

function fourth(int $value): int
{
    # @mago-ignore all
    return $value + 2;
}

function twice(int $value): int
{
    return $value * 2;
}

// Main converted this to twice("1"), which rustc rejects; PHP throws a TypeError.
// @mago-expect analysis:invalid-argument
echo first(1), second(2), third(3), fourth(4), twice("1"), "\n";
