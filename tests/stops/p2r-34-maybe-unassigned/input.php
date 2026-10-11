<?php

declare(strict_types=1);

function pick(int $n): int
{
    if ($n > 0) {
        $result = 1;
    } elseif ($n < 0) {
        $result = 2;
    }
    return $result;
}
