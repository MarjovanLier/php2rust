<?php

declare(strict_types=1);

function isEven(int $n): bool
{
    return $n % 2 === 0;
}

function pick(bool $first, int $a, int $b): int
{
    if ($first) {
        return $a;
    }
    return $b;
}

$found = false;
if (isEven(4)) {
    $found = true;
}
$odd = !isEven(7);
if ($found && $odd) {
    echo pick($found, 1, 2), ' ', pick(false, 1, 2), "\n";
}
