<?php

declare(strict_types=1);

namespace App\Regex;

// This module is named regex, and the helper's ::regex:: paths still name the crate.
final readonly class Matcher
{
    public static function digits(string $text): bool
    {
        return preg_match('/^\d+\z/', $text) === 1;
    }
}
