<?php

declare(strict_types=1);

function mixed_arms(int $n): void
{
    $v = match ($n) {
        1 => 1,
        default => 'one',
    };
    echo $v, "\n";
}

function null_arm(int $n): int
{
    return match ($n) {
        1 => null,
        default => 2,
    };
}

function no_type(int $n, bool $c): int
{
    $empty = match ($n) {
        1 => [],
        default => [],
    };
    $empty[] = 5;
    $none = match ($n) {
        1 => null,
        default => null,
    };
    if ($c) {
        $none = 7;
    }
    $lists = match ($n) {
        1 => [null, 1],
        default => [5],
    };
    return count($empty) + ($none ?? 0) + count($lists);
}

function one_arm(int $n): int
{
    $max = match ($n) {
        default => 9223372036854775807,
    };
    return $max + 1;
}
