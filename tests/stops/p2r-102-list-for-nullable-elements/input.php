<?php

declare(strict_types=1);

/** @param list<?int> $xs */
function size(array $xs): int
{
    return count($xs);
}

$ys = [1, 2];
echo size($ys), "\n";
