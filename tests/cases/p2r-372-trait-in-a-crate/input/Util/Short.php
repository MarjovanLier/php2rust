<?php

declare(strict_types=1);

namespace App\Util;

use App\Enums\Kind;

// The only call of the trait's method sits in a default arm that is left out, so this
// module names no trait.
function short(Kind $kind): string
{
    return match ($kind) {
        Kind::Quote => 'Q',
        Kind::Order => 'O',
        default => $kind->label(),
    };
}
