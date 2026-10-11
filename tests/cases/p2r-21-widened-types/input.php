<?php

declare(strict_types=1);

function rest(int $n): int
{
    $r = $n % 4;
    $r = $r + 10;
    return $r;
}

$n = 5;
$n = $n * 2 + 7;
echo $n, ' ', rest($n), "\n";
