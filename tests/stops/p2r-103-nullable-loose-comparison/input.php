<?php

declare(strict_types=1);

function code(int $n): ?int
{
    if ($n > 0) {
        return $n;
    }
    return null;
}

function same(int $n, int $m): bool
{
    return code($n) == code($m);
}

function less(int $n, int $m): bool
{
    return code($n) < code($m);
}
