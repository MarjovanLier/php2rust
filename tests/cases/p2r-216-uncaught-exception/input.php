<?php

declare(strict_types=1);

function check(int $n): int
{
    if ($n > 2) {
        throw new RuntimeException("too big: $n");
    }
    return $n * 10;
}

echo 'start ';
echo check(1), ' ', check(2), ' ';
echo check(3), "\n";
echo "never\n";
