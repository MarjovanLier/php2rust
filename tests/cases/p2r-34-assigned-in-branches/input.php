<?php

declare(strict_types=1);

function label(int $n): int
{
    if ($n > 5) {
        $x = $n * 2;
        echo $x, ' ';
    } else {
        $x = 1;
        echo $x, ' ';
    }
    if ($n > 7) {
        $only = 3;
        echo $only, ' ';
    }
    $divisor = 7;
    if ($n > 8) {
        $divisor = 4;
    }
    echo 10 % $divisor, ' ';
    return $x;
}

echo label(9), "\n";
echo label(6), "\n";
echo label(2), "\n";
