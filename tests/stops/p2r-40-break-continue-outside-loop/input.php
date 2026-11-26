<?php

declare(strict_types=1);

// PHP rejects this file at compile time: break and continue need a loop or a switch.
// Mago's parser accepts it, so this folder pins php2rust's own check.
function bounded(int $n): int
{
    if ($n > 10) {
        break;
    }
    if ($n < 0) {
        continue;
    }
    return $n;
}
