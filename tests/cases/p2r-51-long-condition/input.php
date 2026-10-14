<?php

declare(strict_types=1);

function wide(int $firstValue, int $secondValue, int $thirdValue): void
{
    if ($firstValue + $secondValue > 10 && $firstValue * $secondValue > 100 && $thirdValue > 12) {
        echo 'first';
    } elseif ($firstValue - $secondValue - $thirdValue > 10 && $secondValue * $thirdValue > 1000000) {
        echo 'second';
    }
    while ($firstValue + $secondValue + $thirdValue > 10 && $secondValue * $thirdValue > 10000000000) {
        echo 'loop';
        $firstValue = 0;
        $thirdValue = 0;
    }
}

function letFirst(?int $maybeAValueFromTheCaller, int $firstValue, int $secondValue): string
{
    if ($maybeAValueFromTheCaller !== null && $firstValue + $secondValue > 10 && $firstValue * $secondValue > 100) {
        return 'value ' . $maybeAValueFromTheCaller;
    }
    return 'none';
}

wide(20, 30, 13);
wide(100, 1, 2);
echo ' ', letFirst(5, 20, 30), ' ', letFirst(null, 20, 30), ' ', letFirst(5, 1, 1), "\n";
echo in_array(3333333333, [1111111111, 2222222222, 3333333333, 4444444444, 5555555555, 6666666666], true) ? 'found' : 'missing', "\n";
