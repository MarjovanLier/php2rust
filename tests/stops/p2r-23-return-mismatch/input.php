<?php

declare(strict_types=1);

// PHP rejects both returns at compile time; Mago parses them, so this folder pins
// php2rust's own check.
function nothing(int $n): void
{
    if ($n > 0) {
        return $n;
    }
}

function missing(int $n): int
{
    if ($n > 0) {
        return;
    }
    return $n;
}
