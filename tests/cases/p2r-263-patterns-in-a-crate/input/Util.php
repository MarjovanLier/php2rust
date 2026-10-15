<?php

declare(strict_types=1);

namespace App;

use App\Regex\Matcher;

final readonly class Util
{
    public static function number(string $text): string
    {
        // A pattern constant of another file of the crate (P2R-171).
        $digits = preg_replace(Matcher::NON_DIGIT, '', $text) ?? '';
        return Matcher::digits($digits) ? $digits : '0';
    }
}
