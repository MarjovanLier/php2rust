<?php

declare(strict_types=1);

enum Mode
{
    case Fast;
    case Safe;

    const fast = 1;
    const Safe = 2;
    const SUM = 1 + 1;
    const COPY = self::SUM;
    const A = 1, B = 2;
    const LATE = static::Fast;
}

final readonly class Limits
{
    const MAX = 9223372036854775807;
    const ZERO = 0;
    const X = 1;
    const x = 2;
    const array LIST = [];
    final private const HIDDEN = 1;

    public function __construct(public int $n) {}

    public function over(): int
    {
        return self::MAX + 1;
    }

    public function rest(int $n): int
    {
        return $n % self::ZERO;
    }

    public function pick(int $n): string
    {
        return match ($n) {
            self::X => 'x',
            default => 'other',
        };
    }
}

function name(Limits $limits): string
{
    return Limits::class . $limits::X;
}

echo (new Limits(1))->pick(1), "\n";
