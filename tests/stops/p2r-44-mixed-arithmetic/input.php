<?php

declare(strict_types=1);

function sum(int $i, float $f): float
{
    return $i + $f;
}

function scaled(float $f): float
{
    return $f * 2;
}

function grow(float $f): float
{
    $f += 1;
    return $f;
}

function lower(int $i, float $f): bool
{
    return $i < $f;
}
