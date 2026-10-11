<?php

declare(strict_types=1);

function next_value(int $n): int
{
    return $n + 1;
}

function scan(int $n): void
{
    $i = 0;
    $seen = 0;
    while ($i < $n) {
        $seen = next_value($i);
        $i++;
    }
}
