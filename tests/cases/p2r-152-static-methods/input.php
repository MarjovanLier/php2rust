<?php

declare(strict_types=1);

final class Counter
{
    public int $count = 0;

    public static function bump(Counter $counter, int $by): void
    {
        $counter->count += $by;
    }

    public static function countdown(int $n): int
    {
        if ($n <= 0) {
            return 0;
        }
        return $n + self::countdown($n - 1);
    }

    public static function fresh(): Counter
    {
        return new Counter();
    }
}

$counter = Counter::fresh();
Counter::bump($counter, 2);
Counter::bump($counter, 3);
echo $counter->count, "\n";
echo Counter::countdown(4), "\n";
