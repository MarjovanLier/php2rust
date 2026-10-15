<?php

declare(strict_types=1);

namespace App\Dates;

final class Parse
{
    public static function year(string $text): \DateTime|false
    {
        return \DateTime::createFromFormat('!Y', $text);
    }
}
