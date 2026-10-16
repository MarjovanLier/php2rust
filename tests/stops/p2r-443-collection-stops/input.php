<?php

declare(strict_types=1);

namespace App;

use Illuminate\Database\Eloquent\Collection;
use Illuminate\Database\Eloquent\Model;

final class Code extends Model
{
    protected $fillable = ['name'];

    public $timestamps = false;
}

// A method other than count(), isEmpty(), isNotEmpty() and first() (P2R-443).
/**
 * @param Collection<int, Code> $codes
 * @return array<int, Code>
 */
function items(Collection $codes): array
{
    return $codes->all();
}

/**
 * @param Collection<int, Code> $codes
 */
function added(Collection $codes, Code $code): int
{
    $codes->push($code);
    return 1;
}

// first() with a callback.
/**
 * @param Collection<int, Code> $codes
 */
function named(Collection $codes): ?Code
{
    return $codes->first(fn (Code $code): bool => $code->name === 'a');
}

// A foreach with a key, which Laravel keeps from the collection.
/**
 * @param Collection<int, Code> $codes
 */
function keyed(Collection $codes): int
{
    $total = 0;
    foreach ($codes as $key => $code) {
        $total += $key;
    }
    return $total;
}

// A condition and empty(): PHP reads an object as true.
/**
 * @param Collection<int, Code> $codes
 */
function truthy(Collection $codes): bool
{
    return empty($codes);
}

// A comparison.
/**
 * @param Collection<int, Code> $codes
 * @param Collection<int, Code> $others
 */
function same(Collection $codes, Collection $others): bool
{
    return $codes === $others;
}

// gettype() of a collection.
/**
 * @param Collection<int, Code> $codes
 */
function typeOf(Collection $codes): string
{
    return gettype($codes);
}

// A nullable collection, one without type arguments, and one in a union stop (P2R-440).
/**
 * @param ?Collection<int, Code> $codes
 */
function maybe(?Collection $codes): int
{
    return 0;
}

function bare(Collection $codes): int
{
    return 0;
}

/**
 * @param Collection<int, Code>|string $codes
 */
function either(Collection|string $codes): int
{
    return 0;
}

// get() and all() with the columns to load (P2R-441).
/**
 * @return Collection<int, Code>
 */
function columns(): Collection
{
    return Code::query()->get(['name']);
}

/**
 * @return Collection<int, Code>
 */
function allColumns(): Collection
{
    return Code::all(['name']);
}

// A collection made other than by the store (P2R-443).
/**
 * @return Collection<int, Code>
 */
function made(): Collection
{
    return new Collection([]);
}
