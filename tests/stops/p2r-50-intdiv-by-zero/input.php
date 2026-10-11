<?php

declare(strict_types=1);

function broken(int $n): int
{
    return intdiv($n, 0);
}

$zero = 0;
echo intdiv(5, $zero), "\n";
echo intdiv(5, 3 - 3), "\n";
