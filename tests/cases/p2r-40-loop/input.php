<?php

declare(strict_types=1);

function firstSquareAbove(int $limit): int
{
    $n = 0;
    while (true) {
        $n++;
        if ($n * $n > $limit) {
            return $n;
        }
    }
}

$count = 0;
for (;;) {
    $count++;
    if ($count === 3) {
        break;
    }
}
echo firstSquareAbove(50), ' ', $count, "\n";

while (true) {
    $found = 7;
    break;
}
echo $found, "\n";
