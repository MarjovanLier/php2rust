<?php

declare(strict_types=1);

namespace App;

use Spatie\LaravelData\Optional;

// Optional::create() is Absent only where Mago types the call as Optional alone.
function made(): string|Optional|null
{
    return Optional::create();
}
