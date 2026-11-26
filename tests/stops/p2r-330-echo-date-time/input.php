<?php

declare(strict_types=1);

// PHP throws an Error here: a DateTime cannot be converted to a string.
function stamp(string $text): string
{
    $date = DateTime::createFromFormat('!Y', $text);
    if ($date === false) {
        return 'none';
    }
    echo $date;
    return 'done';
}
