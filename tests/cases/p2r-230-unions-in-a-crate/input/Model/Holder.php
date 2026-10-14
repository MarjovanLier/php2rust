<?php

declare(strict_types=1);

namespace App\Model;

use App\Dto\Box;

final readonly class Holder
{
    public static function kind(Box|int $v): string
    {
        return gettype($v);
    }
}
