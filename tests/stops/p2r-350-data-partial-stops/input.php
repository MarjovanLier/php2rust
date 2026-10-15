<?php

declare(strict_types=1);

namespace App;

use Spatie\LaravelData\Data;

// The methods of Data are not all known, so an override cannot be checked.
final class Point extends Data
{
    public function __construct(
        public int $x,
    ) {}
}
