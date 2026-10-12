<?php

declare(strict_types=1);

function code(int $n): ?int
{
    if ($n > 0) {
        return $n;
    }
    return null;
}

function twice(int $n): int
{
    return $n * 2;
}

$a = code(1);
echo twice($a), "\n";
