<?php

declare(strict_types=1);

function plain(string $text): int
{
    /** @var array{year: int, month: int, day: int, hour: int, minute: int, second: int} $parts */
    $parts = date_parse($text);
    return $parts['year'];
}

function other(string $text): int
{
    $date = DateTime::createFromFormat('!Y', $text);
    if ($date === false) {
        return 0;
    }
    /** @var array{year: int, month: int, day: int, hour: int, minute: int, second: int} $parts */
    $parts = date_parse($date->format('Y-m-d'));
    return $parts['year'];
}

function narrow(string $text): int
{
    $date = DateTime::createFromFormat('!Y', $text);
    if ($date === false) {
        return 0;
    }
    /** @var array{year: int} $parts */
    $parts = date_parse($date->format('Y-m-d H:i:s'));
    return $parts['year'];
}

function maybe(string $text): int
{
    $date = DateTime::createFromFormat('!Y', $text);
    /** @var array{year: int, month: int, day: int, hour: int, minute: int, second: int} $parts */
    $parts = date_parse($date->format('Y-m-d H:i:s'));
    return $parts['year'];
}
