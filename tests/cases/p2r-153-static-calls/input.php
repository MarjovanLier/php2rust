<?php

declare(strict_types=1);

enum Level: int
{
    case Low = 1;
    case High = 2;

    public static function parse(int $n): ?self
    {
        return self::tryFrom($n);
    }

    public static function orLow(int $n): self
    {
        return self::parse($n) ?? static::Low;
    }

    public static function count(): int
    {
        return \count(static::cases());
    }

    public function isHigh(): bool
    {
        return match ($this) {
            SELF::High => true,
            self::Low => false,
        };
    }
}

final readonly class Pair
{
    public function __construct(public int $a, public int $b) {}

    public static function of(int $n): Pair
    {
        return new Pair($n, static::double($n));
    }

    public static function double(int $n): int
    {
        return $n * 2;
    }

    public function sum(): int
    {
        return $this->a + self::double($this->b);
    }
}

echo Level::orLow(2)->value, "\n";
echo Level::orLow(9)->value, "\n";
echo Level::count(), "\n";
if (Level::from(2)->isHigh()) {
    echo "high\n";
}
echo Pair::of(3)->sum(), "\n";
