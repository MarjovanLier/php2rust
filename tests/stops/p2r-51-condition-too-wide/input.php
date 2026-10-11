<?php

declare(strict_types=1);

function wide(int $firstValue, int $secondValue, int $thirdValue): void
{
    if ($firstValue + $secondValue > 10 && $firstValue * $secondValue > 100 && $thirdValue > 12) {
        echo 'fits';
    } elseif ($firstValue - $secondValue - $thirdValue > 10 && $secondValue * $thirdValue > 1000000) {
        echo 'too wide';
    }
    while ($firstValue + $secondValue + $thirdValue > 10 && $secondValue * $thirdValue > 10000000000) {
        echo 'too wide';
    }
}
