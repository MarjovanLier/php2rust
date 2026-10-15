<?php

declare(strict_types=1);

final class Formats
{
    public const string DAY = 'D, Ymd';
}

function forms(string $text, int $year): string
{
    $a = DateTime::createFromFormat('D Ymd', $text);
    $b = DateTime::createFromFormat('!Y*', $text);
    $c = DateTime::createFromFormat(Formats::DAY, $text);
    $d = DateTime::createFromFormat('!Y', $year);
    $e = DateTime::createFromFormat('!Y', $text, null);
    $f = DateTime::createFromFormat(format: '!Y', datetime: $text);
    $g = DateTime::createFromFormat("!Y\\D+", $text);
    $parts = [$a, $b, $c, $d, $e, $f, $g];
    return count($parts) > 0 ? 'some' : 'none';
}
