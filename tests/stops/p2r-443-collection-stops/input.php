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

// isEmpty(), isNotEmpty() and first() as a statement, whose unused result rustc refuses.
/**
 * @param Collection<int, Code> $codes
 */
function unused(Collection $codes): int
{
    $codes->isEmpty();
    return 0;
}

// first() of a collection of DateTime (P2R-442).
/**
 * @param \Illuminate\Support\Collection<int, \DateTime> $dates
 */
function firstDate(\Illuminate\Support\Collection $dates): bool
{
    return $dates->first() instanceof \DateTime;
}

// A nullable element type, a single type argument, and a collection inside a list, a
// shape or another collection (P2R-440).
/**
 * @param \Illuminate\Support\Collection<int, ?int> $numbers
 */
function nullable(\Illuminate\Support\Collection $numbers): int
{
    return 0;
}

/**
 * @param Collection<Code> $codes
 */
function single(Collection $codes): int
{
    return 0;
}

/**
 * @param list<\Illuminate\Support\Collection<int, string>> $groups
 */
function listed(array $groups): int
{
    return 0;
}

/**
 * @param array{names: \Illuminate\Support\Collection<int, string>} $shape
 */
function shaped(array $shape): int
{
    return 0;
}

/**
 * @param \Illuminate\Support\Collection<int, \Illuminate\Support\Collection<int, string>> $groups
 */
function nested(\Illuminate\Support\Collection $groups): int
{
    return 0;
}

// get() on the model class, through __callStatic (P2R-441).
function staticGet(): int
{
    return Code::get()->count();
}

function staticWhere(): int
{
    return Code::where('name', 'a')->get()->count();
}

// A collection of objects whose class changes them, also in an interface (P2R-440).
final class Counter
{
    public int $n = 0;

    public function bump(): void
    {
        $this->n++;
    }
}

/**
 * @param \Illuminate\Support\Collection<int, Counter> $list
 */
function counters(\Illuminate\Support\Collection $list): int
{
    return 0;
}

interface Counts
{
    /**
     * @param \Illuminate\Support\Collection<int, Counter> $counterList
     */
    public function counted(\Illuminate\Support\Collection $counterList): int;
}

// A union element type and a collection inside a map (P2R-440).
/**
 * @param \Illuminate\Support\Collection<int, int|string> $values
 */
function unionItems(\Illuminate\Support\Collection $values): int
{
    return 0;
}

/**
 * @param array<string, \Illuminate\Support\Collection<int, string>> $groups
 */
function mapped(array $groups): int
{
    return 0;
}

// A foreach by reference, a condition, a negation and a ?: on a collection (P2R-443).
/**
 * @param Collection<int, Code> $codes
 */
function byReference(Collection $codes): int
{
    foreach ($codes as &$code) {
        $code = null;
    }
    return 0;
}

/**
 * @param Collection<int, Code> $codes
 */
function condition(Collection $codes): int
{
    if ($codes) {
        return 1;
    }
    return 0;
}

/**
 * @param Collection<int, Code> $codes
 */
function negated(Collection $codes): bool
{
    return !$codes;
}

/**
 * @param Collection<int, Code> $codes
 */
function chosen(Collection $codes): int
{
    return $codes ? 1 : 0;
}

// is_scalar(), echo and a property read of a collection (P2R-443).
/**
 * @param Collection<int, Code> $codes
 */
function scalar(Collection $codes): bool
{
    return is_scalar($codes);
}

/**
 * @param Collection<int, Code> $codes
 */
function printed(Collection $codes): void
{
    echo $codes;
}

/**
 * @param Collection<int, Code> $codes
 */
function proxied(Collection $codes): int
{
    return (int) $codes->sum;
}

// Laravel's ArrayAccess on a collection (P2R-443).
/**
 * @param Collection<int, Code> $codes
 */
function indexed(Collection $codes): Code
{
    return $codes[0];
}

/**
 * @param Collection<int, Code> $codes
 */
function appended(Collection $codes, Code $code): int
{
    $codes[] = $code;
    return 1;
}

// first() with a default or a named argument, and isNotEmpty() and first() as statements.
/**
 * @param Collection<int, Code> $codes
 */
function firstOr(Collection $codes, Code $fallback): Code
{
    return $codes->first(null, $fallback);
}

/**
 * @param Collection<int, Code> $codes
 */
function firstNamed(Collection $codes): ?Code
{
    return $codes->first(callback: null);
}

/**
 * @param Collection<int, Code> $codes
 */
function unusedNot(Collection $codes): int
{
    $codes->isNotEmpty();
    return 0;
}

/**
 * @param Collection<int, Code> $codes
 */
function unusedFirst(Collection $codes): int
{
    $codes->first();
    return 0;
}

// collect() and a static method of a collection class (P2R-443).
/**
 * @return \Illuminate\Support\Collection<int, string>
 */
function collected(): \Illuminate\Support\Collection
{
    return \Illuminate\Support\collect(['a']);
}

/**
 * @return \Illuminate\Support\Collection<int, string>
 */
function madeStatic(): \Illuminate\Support\Collection
{
    return \Illuminate\Support\Collection::make(['a']);
}

// A where with an operator before get(), and get() and all() on a model whose table a
// migration seeds, which stop with the reason of P2R-410 (P2R-441).
/**
 * @return Collection<int, Code>
 */
function operator(): Collection
{
    return Code::query()->where('name', '=', 'a')->get();
}

final class Seed extends Model
{
    protected $fillable = ['label'];

    public $timestamps = false;
}

/**
 * @return Collection<int, Seed>
 */
function seededAll(): Collection
{
    return Seed::all();
}

/**
 * @return Collection<int, Seed>
 */
function seededGet(): Collection
{
    return Seed::query()->get();
}

// A collection made other than by the store (P2R-443).
/**
 * @return Collection<int, Code>
 */
function made(): Collection
{
    return new Collection([]);
}
