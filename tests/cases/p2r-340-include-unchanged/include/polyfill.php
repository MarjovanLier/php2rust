<?php

// Polyfills and classes with other signatures than PHP's built-ins, which change nothing.
if (!function_exists('str_contains')) {
    function str_contains(int $haystack): int
    {
        return $haystack;
    }
}

function strlen(string $text): string
{
    return $text;
}

final class DateTime
{
    public function format(): int
    {
        return 0;
    }
}
