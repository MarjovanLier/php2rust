<?php

declare(strict_types=1);

function code(int $n): ?int
{
    if ($n > 0) {
        return $n;
    }
    return null;
}

$n = 2;
$v = code($n);
if ($n > 1 && $v !== null && $v > 3) {
    echo "big\n";
}
