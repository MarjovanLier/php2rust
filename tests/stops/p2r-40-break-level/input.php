<?php

declare(strict_types=1);

function nested(int $n): void
{
    $i = 0;
    while ($i < $n) {
        $i++;
        $j = 0;
        while ($j < $n) {
            $j++;
            if ($j === 2) {
                continue 2;
            }
            if ($j === 3) {
                break 2;
            }
            if ($j === 4) {
                break 1;
            }
        }
    }
}
