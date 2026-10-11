<?php

declare(strict_types=1);

$first = total(3, 4);

function total(int $price, int $count): int
{
    $first = $price * $count;
    $shipping = 5;
    $sum = $first + $shipping;
    return $sum;
}

$second = $first - 2;
$type = 7;
$first = $second * $type;
echo $first, ' ', $second, "\n";
