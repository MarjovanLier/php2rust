<?php

declare(strict_types=1);

// A comment that makes Mago drop a finding stops, because the finding may be one PHP
// enforces (P2R-501): a line comment, a docblock and a block comment.
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

echo first(1), second(2), third(3), "\n";
