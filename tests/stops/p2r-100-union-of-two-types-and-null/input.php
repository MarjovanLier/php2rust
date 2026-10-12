<?php

declare(strict_types=1);

function f(int|string|null $x): int
{
    if ($x === null) {
        return 0;
    }
    return 1;
}
