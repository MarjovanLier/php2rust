<?php

declare(strict_types=1);

namespace App\Util;

use InvalidArgumentException;

final readonly class Parse
{
    public static function digit(string $s): int
    {
        if (!ctype_digit($s)) {
            throw new InvalidArgumentException("not a digit: $s");
        }
        return (int) $s;
    }
}
