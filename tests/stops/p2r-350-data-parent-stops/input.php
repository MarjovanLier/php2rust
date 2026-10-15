<?php

declare(strict_types=1);

namespace App;

use Spatie\LaravelData\Data;

// The override check cannot see the methods of Data's own parent.
final class Point extends Data
{
    public function __construct(
        public int $x,
    ) {}
}
