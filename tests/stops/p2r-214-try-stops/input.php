<?php

declare(strict_types=1);

function risky(int $n): int
{
    if ($n < 0) {
        throw new RangeException('negative');
    }
    return $n;
}

function finally_clause(): int
{
    try {
        return risky(1);
    } catch (Exception $e) {
        return 0;
    } finally {
        echo "done\n";
    }
}

function nested(): int
{
    try {
        try {
            return risky(1);
        } catch (RangeException $e) {
            return 0;
        }
    } catch (Exception $e) {
        return 1;
    }
}

function breaking(): int
{
    $total = 0;
    foreach ([1, 2] as $n) {
        try {
            $total += risky($n);
            break;
        } catch (Exception $e) {
            $total = -1;
        }
    }
    return $total;
}

function typed(): int
{
    try {
        return risky(1);
    } catch (TypeError $e) {
        return 0;
    }
}

function after_all(): int
{
    try {
        return risky(1);
    } catch (Exception $e) {
        return 0;
    } catch (RangeException $e) {
        return 1;
    }
}

function names(): int
{
    try {
        return risky(1);
    } catch (RangeException $a) {
        return 0;
    } catch (LogicException $b) {
        return 1;
    }
}

function nameless(): int
{
    try {
        return risky(1);
    } catch (RangeException) {
        return 0;
    }
}

function reused(int $e): int
{
    try {
        return risky($e);
    } catch (Exception $e) {
        return 0;
    }
}

function passed(): string
{
    try {
        return (string) risky(1);
    } catch (Exception $e) {
        return get_class($e);
    }
}

function skipping(): int
{
    $total = 0;
    foreach ([1, -2, 3] as $n) {
        try {
            $total += risky($n);
        } catch (RangeException $e) {
            continue;
        }
    }
    return $total;
}

function quiet(): int
{
    try {
        $n = 1;
    } catch (Exception $e) {
        $n = 0;
    }
    return $n;
}
