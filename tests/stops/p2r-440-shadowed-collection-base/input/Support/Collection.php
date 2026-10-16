<?php

declare(strict_types=1);

namespace Illuminate\Support;

// The input's own base collection, which Laravel's Eloquent collection then inherits from
// (P2R-340), so Eloquent's collection maps to no list (P2R-440).
/**
 * @template TKey of array-key
 * @template TValue
 */
class Collection
{
    public function count(): int
    {
        return 42;
    }
}
