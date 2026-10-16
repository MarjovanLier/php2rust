<?php

declare(strict_types=1);

namespace App\Nullable;

use App\Processors\Splitter;

// The only call of the trait's method here is a nullsafe one.
function amountOf(?Splitter $splitter): ?int
{
    return $splitter?->amount();
}
