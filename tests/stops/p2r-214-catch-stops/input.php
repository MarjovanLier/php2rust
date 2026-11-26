<?php

declare(strict_types=1);

// No class can be named int, so this catch never runs.
function checked(int $n): int
{
    try {
        if ($n < 0) {
            throw new RuntimeException('negative');
        }
        return $n;
    } catch (int $e) {
        return 0;
    }
}

function caughtLine(int $n): int
{
    try {
        if ($n < 0) {
            throw new RuntimeException('negative');
        }
        return $n;
    } catch (RuntimeException $e) {
        return $e->getLine();
    }
}

function caughtCopy(int $n): string
{
    try {
        if ($n < 0) {
            throw new RuntimeException('negative');
        }
        return 'fine';
    } catch (RuntimeException $e) {
        $copy = $e;
        return $copy->getMessage();
    }
}

echo checked(1), caughtLine(1), caughtCopy(1), "\n";
