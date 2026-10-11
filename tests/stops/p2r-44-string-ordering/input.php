<?php

declare(strict_types=1);

function before(string $a, string $b): void
{
    if ($a < $b) {
        echo 'before';
    }
    if ($a >= 'm') {
        echo 'late';
    }
}
