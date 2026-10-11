<?php

declare(strict_types=1);

function same(string $a, string $b): void
{
    if ($a == $b) {
        echo 'equal';
    }
    if ($a != 'x') {
        echo 'not x';
    }
}
