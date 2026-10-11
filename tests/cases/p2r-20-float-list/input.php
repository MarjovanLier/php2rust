<?php

declare(strict_types=1);

/**
 * @param list<float> $values
 */
function total(array $values): float
{
    $sum = 0.0;
    $i = 0;
    while ($i < count($values)) {
        $sum += $values[$i];
        $i++;
    }
    return $sum;
}

/**
 * @return list<float>
 */
function halves(): array
{
    return [0.5, 1.5];
}

function check(bool $ok): void
{
    if ($ok) {
        echo 'y';
    } else {
        echo 'n';
    }
}

$prices = [1.25, 2.5];
check($prices[1] > 2.0);
$prices[] = 0.25;
$copy = $prices;
$copy[] = 1.0;
check(total($prices) === 4.0);
check(total($copy) === 5.0);
check(total(halves()) === 2.0);
check(total([]) === 0.0);
check(total([0.1, 0.2]) === 0.3);
echo "\n";
