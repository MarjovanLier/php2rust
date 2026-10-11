<?php

declare(strict_types=1);

function grow(int $n): int
{
    $n = $n + 1;
    $n *= 3;
    return $n;
}

$count = 10;
$count += grow(2);
$count -= 4;
$count *= 2;
$count++;
++$count;
$count--;
--$count;
$step = 1;
$step++;
echo $count, ' ', $step, "\n";
