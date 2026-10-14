<?php

declare(strict_types=1);

final class Counter
{
    public static int $made = 0;

    public int $count = 0;

    public static function peek(): int
    {
        return $this->count;
    }

    public static function forever(int $n): int
    {
        return static::forever($n + 1);
    }
}

echo Counter::peek(), "\n";
