<?php

declare(strict_types=1);

function shifts(int $n, float $f, ?int $maybe): void
{
    echo 1 << 64;
    echo 1 << $n;
    echo (1 << 63) - 1;
    echo $n << 2.5;
    echo $f >> 1;
    echo $maybe << 1;
    echo $n << (1 + 1);
    $i = 1 << 62;
    echo $i * 2;
    $k = $n;
    $k <<= $n;
    $k >>= 64;
    echo $k;
}
