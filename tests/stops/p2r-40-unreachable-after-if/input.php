<?php

declare(strict_types=1);

function choose(int $n): int
{
    if ($n > 0) {
        return 1;
    } elseif ($n < 0) {
        return -1;
    } else {
        return 0;
    }
    echo 'never';
}
