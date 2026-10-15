<?php

declare(strict_types=1);

namespace App;

use Spatie\LaravelData\Data;

// The struct would drop the call PHP makes when the object goes.
final class Point extends Data
{
    public function __construct(
        public int $x,
    ) {}
}
