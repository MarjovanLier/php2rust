<?php

declare(strict_types=1);

function half(float $x): float
{
    return $x * 0.5;
}

function report(float $value, float $limit): void
{
    if ($value > $limit) {
        echo 'above';
    } else {
        echo 'not above';
    }
    echo "\n";
}

$price = 19.99;
$total = half($price);
report($total, 9.99);
report(half(2.5), $total);
