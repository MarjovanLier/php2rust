<?php

declare(strict_types=1);

// PHP throws a TypeError here, because strict types do not let an int pass as a string.
function digits(int $n): string
{
    $out = preg_replace_callback('/\d/', fn (array $m): string => '#', $n) ?? '';
    return is_string($out) ? $out : '';
}

function letters(string $s): string
{
    return preg_replace_callback('/[a-z]/', fn (?array $m): string => '#', $s) ?? '';
}
