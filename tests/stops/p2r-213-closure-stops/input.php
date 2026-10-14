<?php

declare(strict_types=1);

function risky(int $n): int
{
    if ($n < 0) {
        throw new RangeException('negative');
    }
    return $n;
}

function label(int $n): string
{
    if ($n < 0) {
        throw new RangeException('negative');
    }
    return 'n';
}

function fallback(?int $x): int
{
    return $x ?? risky(5);
}

function needed(?int $x): int
{
    return $x ?? throw new LengthException(label(-1));
}
