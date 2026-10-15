<?php

// A polyfill with another signature than PHP's built-in, which must change nothing.
if (!function_exists('str_contains')) {
    function str_contains(int $haystack): int
    {
        return $haystack;
    }
}
