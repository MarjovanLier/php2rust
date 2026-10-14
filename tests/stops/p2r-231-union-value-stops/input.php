<?php

declare(strict_types=1);

function wide(bool|int|string $y): string
{
    return gettype($y);
}

function narrow(int|string $x): string
{
    return wide($x);
}

function floats(float|string $x): string
{
    return gettype($x);
}

function plain(int|string $x): int
{
    return strlen($x);
}

echo floats(5), wide([1]), "\n";
