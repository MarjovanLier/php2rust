<?php

declare(strict_types=1);

function bump(?int $n): int
{
    if ($n === null) {
        return 0;
    }
    $n = $n + 1;
    return $n;
}
