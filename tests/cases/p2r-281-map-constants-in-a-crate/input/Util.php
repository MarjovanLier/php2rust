<?php

declare(strict_types=1);

namespace App;

use App\Tables\Mappings;

final readonly class Util
{
    use Mappings;

    public static function plain(string $text): string
    {
        return strtr($text, self::ACCENTS);
    }
}
