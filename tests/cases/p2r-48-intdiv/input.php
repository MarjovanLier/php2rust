<?php

declare(strict_types=1);

function per(int $total, int $size): int
{
    return intdiv($total, $size);
}

$n = 17;
echo intdiv(7, 2), ' ', intdiv(-7, 2), ' ', intdiv(7, -2), ' ', INTDIV(-7, -2), "\n";
echo per(100, 7) * 2, ' ', 3 * intdiv(10, 4), ' ', intdiv($n, 5) + 1, "\n";
$q = intdiv($n, 3);
echo $q, ' ', 10 % ($q - 4), "\n";
