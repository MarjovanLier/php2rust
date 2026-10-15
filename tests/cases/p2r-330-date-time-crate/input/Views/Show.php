<?php

declare(strict_types=1);

namespace App\Views;

use App\Dates\Parse;

final class Show
{
    public static function year(string $text): string
    {
        $date = Parse::year($text);
        if ($date === false) {
            return 'none';
        }
        return $date->format('Y');
    }
}
