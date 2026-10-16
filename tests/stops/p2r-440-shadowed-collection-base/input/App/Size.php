<?php

declare(strict_types=1);

namespace App;

use Illuminate\Database\Eloquent\Collection;

/**
 * @param Collection<int, int> $numbers
 */
function size(Collection $numbers): int
{
    return $numbers->count();
}
