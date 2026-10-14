<?php

declare(strict_types=1);

namespace App\Util;

use App\Maker;

final readonly class Today
{
    public static function label(): string
    {
        return Maker::show(['day' => '07']);
    }
}
