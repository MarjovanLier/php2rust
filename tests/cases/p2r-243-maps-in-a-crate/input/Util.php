<?php

declare(strict_types=1);

namespace App;

use App\Model\Bag;

final readonly class Util
{
    public static function make(): Bag
    {
        return new Bag(['apple' => 3, 'pear' => 2]);
    }
}
