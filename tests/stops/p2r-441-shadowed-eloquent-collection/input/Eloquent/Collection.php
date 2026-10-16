<?php

declare(strict_types=1);

namespace Illuminate\Database\Eloquent;

// The input's own Eloquent collection, which all() then gives (P2R-340), so all() does not
// read the store (P2R-441).
/**
 * @template TKey of array-key
 * @template TModel of Model
 */
final class Collection
{
    public function count(): int
    {
        return 42;
    }
}
