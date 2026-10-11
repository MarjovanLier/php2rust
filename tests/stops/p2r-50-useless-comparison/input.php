<?php

declare(strict_types=1);

function limits(int $n): void
{
    if ($n <= 9223372036854775807) {
        echo 'always';
    }
    if (9223372036854775807 < $n) {
        echo 'never';
    }
    if ($n < 9223372036854775807) {
        echo 'fine';
    }
}
