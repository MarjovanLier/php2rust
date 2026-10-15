<?php

declare(strict_types=1);

namespace App;

use Spatie\LaravelData\Optional;

// A trait named Optional is not the class of 4.18.0, so the union has no Rust type.
function given(string|Optional $value): bool
{
    return $value instanceof Optional;
}
