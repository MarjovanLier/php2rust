<?php

declare(strict_types=1);

function code(int $n): ?int
{
    if ($n > 0) {
        return $n;
    }
    return null;
}

$xs = [1, 2];
$xs[] = null;
echo count($xs), "\n";
$ys = [3, 4];
echo $ys[code(1)], "\n";
