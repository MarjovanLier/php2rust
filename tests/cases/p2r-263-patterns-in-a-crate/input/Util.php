<?php

declare(strict_types=1);

namespace App;

use App\Regex\Matcher;

final readonly class Util
{
    public static function number(string $text): string
    {
        $digits = preg_replace('#\D#', '', $text) ?? '';
        return Matcher::digits($digits) ? $digits : '0';
    }
}
