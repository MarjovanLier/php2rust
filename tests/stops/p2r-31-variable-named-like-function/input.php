<?php

declare(strict_types=1);

function total(int $n): int
{
    return $n * 2;
}

$total = total(4);
echo $total, "\n";
