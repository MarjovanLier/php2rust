<?php

declare(strict_types=1);

function sum(int $myValue): int
{
    $my_value = $myValue + 1;
    return $my_value;
}

echo sum(1), "\n";
