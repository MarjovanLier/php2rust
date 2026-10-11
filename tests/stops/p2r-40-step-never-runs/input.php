<?php

declare(strict_types=1);

function first(int $n): int
{
    for ($i = 0; $i < $n; $i++) {
        return $i;
    }
    return -1;
}
