<?php

declare(strict_types=1);

function last(int $n): int
{
    $i = 0;
    while ($i < $n) {
        $result = $i;
        $i++;
    }
    return $result;
}
