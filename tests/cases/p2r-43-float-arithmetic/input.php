<?php

declare(strict_types=1);

function blend(float $a, float $b): float
{
    $sum = $a + $b;
    $sum -= 0.25;
    $sum *= 2.0;
    $sum += -$a;
    $sum++;
    $sum--;
    ++$sum;
    return $sum - $b * 0.5;
}

function check(float $value, float $expected): void
{
    if ($value === $expected) {
        echo 'same';
    } else {
        echo 'different';
    }
    echo "\n";
}

check(blend(1.5, 2.0), 5.0);
check(-(-0.5) * 4.0 - 1.0, 1.0);
check(0.1 * 3.0, 0.3);
$steps = 0;
for ($x = 0.0; $x < 1.0; $x += 0.25) {
    $steps++;
}
--$x;
check($x, 0.0);
echo $steps, "\n";
