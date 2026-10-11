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

function cheap(float $value): bool
{
    return $value < 10.0;
}

$price = 19.99;
$total = half($price);
report($total, 9.99);
report(half(2.5), $total);
if (cheap($total)) {
    $discount = 0.5;
} else {
    $discount = 1.5;
}
report($discount, 1.0);
$i = 0;
while ($i < 2) {
    $rate = 0.25;
    $rate *= 2.0;
    $i++;
    report($rate, 0.4);
}
