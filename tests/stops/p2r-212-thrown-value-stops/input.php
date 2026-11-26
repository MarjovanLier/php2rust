<?php

declare(strict_types=1);

function after(): void
{
    try {
        throw new RuntimeException('first');
    } catch (RuntimeException $e) {
        echo "caught\n";
    }
    throw $e;
}

function chosen(int $n): void
{
    throw $n < 0 ? new RangeException('negative') : new LengthException('zero');
}

try {
    after();
} catch (RuntimeException $e) {
    echo $e->getMessage(), "\n";
}
try {
    chosen(0);
} catch (LogicException $e) {
    echo $e->getMessage(), "\n";
}
