<?php

declare(strict_types=1);

function spin(int $n): void
{
    while ($n > 0) {
        $n--;
        if ($n === 3) {
            break;
            echo 'after break';
        }
        continue;
        echo 'after continue';
    }
    while (true) {
        echo 'forever';
    }
    echo 'after loop';
}
