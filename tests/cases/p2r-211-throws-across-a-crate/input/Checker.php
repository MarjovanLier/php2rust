<?php

declare(strict_types=1);

namespace App;

use App\Util\Parse;

final readonly class Checker
{
    public static function sum(string $a, string $b): int
    {
        return Parse::digit($a) + Parse::digit($b);
    }

    public static function safe(string $a): int
    {
        try {
            return Parse::digit($a);
        } catch (\Exception $e) {
            return -1;
        }
    }
}
