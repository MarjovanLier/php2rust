<?php

declare(strict_types=1);

function one(): int
{
    return 1;
}

$max = 9223372036854775807;
echo one(), "\n";
$next = $max + 1;
echo $next, "\n";
$min = -$max - 1;
echo -$min, "\n";
