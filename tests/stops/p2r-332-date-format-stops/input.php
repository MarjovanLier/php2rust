<?php

declare(strict_types=1);

function unchecked(string $text): string
{
    $date = DateTime::createFromFormat('!Y', $text);
    return (string) $date->format('Y');
}

function forms(string $text, int $width): string
{
    $date = DateTime::createFromFormat('!Y', $text);
    if ($date === false) {
        return '';
    }
    $b = $date->format('D, d M Y');
    $c = $date->format('Y', 'm');
    $d = $date->format($width);
    $e = $date->format('\Y U');
    return $b . $c . $d . $e;
}
