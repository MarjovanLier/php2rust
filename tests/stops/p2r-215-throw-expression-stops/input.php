<?php

declare(strict_types=1);

function never(int $n): int
{
    return match ($n) {
        default => throw new LogicException('no'),
    };
}

function chosen(bool $b): int
{
    return $b ? 1 : throw new LogicException('no');
}

function maybe(): ?int
{
    return null;
}

echo maybe() ?? throw new LogicException('top'), "\n";
