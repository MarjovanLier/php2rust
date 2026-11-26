<?php

declare(strict_types=1);

// PHP throws an ArgumentCountError here: date_parse() takes one argument.
function year(string $text): int
{
    $date = DateTime::createFromFormat('!Y', $text);
    if ($date === false) {
        return 0;
    }
    /** @var array{year: int, month: int, day: int, hour: int, minute: int, second: int} $parts */
    $parts = date_parse($date->format('Y-m-d H:i:s'), 'UTC');
    return $parts['year'];
}
