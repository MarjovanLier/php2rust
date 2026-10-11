<?php

declare(strict_types=1);

function zero(int $n): void
{
    $divisor = 0;
    if ($n > 0) {
        $divisor = 0;
    }
    echo 10 % $divisor, "\n";
    $other = 1;
    if ($n > $other) {
        $other = 0;
    } else {
        return;
    }
    echo 10 % $other, "\n";
}
