<?php

declare(strict_types=1);

function loop(int $match, int $gen): int
{
    return $match + $gen;
}

echo loop(2, 3), "\n";
