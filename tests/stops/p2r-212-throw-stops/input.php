<?php

declare(strict_types=1);

function error(int $n): int
{
    if ($n < 0) {
        throw new Error('negative');
    }
    return $n;
}

function previous(int $n): int
{
    if ($n < 0) {
        throw new RuntimeException('negative', 1, new LogicException('first'));
    }
    return $n;
}

function text(string $s): int
{
    if ($s === '') {
        throw $s;
    }
    return 1;
}

function made(): int
{
    $e = new RuntimeException('kept');
    echo $e->getMessage();
    return 1;
}
