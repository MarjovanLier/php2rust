<?php

declare(strict_types=1);

namespace App;

// The exception struct's paths start with ::std, because the namespace App\Std gives the
// crate a module std beside it.
function positive(int $n): int
{
    if ($n < 0) {
        throw new \InvalidArgumentException('negative');
    }
    return $n;
}
