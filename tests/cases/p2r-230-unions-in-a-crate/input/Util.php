<?php

declare(strict_types=1);

namespace App;

use App\Dto\Box;

final readonly class Util
{
    public static function make(int|string $id): Box
    {
        return new Box($id);
    }
}
