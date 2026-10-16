<?php

declare(strict_types=1);

namespace App;

use Illuminate\Database\Eloquent\Builder;
use Illuminate\Database\Eloquent\Collection;
use Illuminate\Database\Eloquent\Model;

final class Code extends Model
{
    protected $fillable = ['name', 'rank'];

    public $timestamps = false;
}

// when() with one or three arguments, or a named argument (P2R-453).
/**
 * @return Collection<int, Code>
 */
function oneArgument(?string $name): Collection
{
    return Code::query()->when($name)->get();
}

/**
 * @return Collection<int, Code>
 */
function withDefault(?string $name): Collection
{
    return Code::query()->when($name, fn (Builder $query) => $query->where('name', 'a'), fn (Builder $query) => $query->where('name', 'b'))->get();
}

/**
 * @return Collection<int, Code>
 */
function named(?string $name): Collection
{
    return Code::query()->when(value: $name, callback: fn (Builder $query) => $query->where('name', 'a'))->get();
}

// A value that is no variable or property read, a closure, or of another type (P2R-451).
/**
 * @return Collection<int, Code>
 */
function literalValue(): Collection
{
    return Code::query()->when(true, fn (Builder $query) => $query->where('name', 'a'))->get();
}

/**
 * @return Collection<int, Code>
 */
function closureValue(): Collection
{
    return Code::query()->when(fn (): bool => true, fn (Builder $query) => $query->where('name', 'a'))->get();
}

/**
 * @param array<string, int> $map
 * @return Collection<int, Code>
 */
function mapValue(array $map): Collection
{
    return Code::query()->when($map, fn (Builder $query) => $query->where('name', 'a'))->get();
}

// Callbacks of another form: use, a return type, another statement, an operator, a value
// of the enclosing body, another parameter type, three parameters (P2R-453).
/**
 * @return Collection<int, Code>
 */
function withUse(?string $name, string $other): Collection
{
    return Code::query()->when($name, function (Builder $query) use ($other): void {
        $query->where('name', $other);
    })->get();
}

/**
 * @return Collection<int, Code>
 */
function returnType(?string $name): Collection
{
    return Code::query()->when($name, function (Builder $query, string $name): Builder {
        return $query->where('name', $name);
    })->get();
}

/**
 * @return Collection<int, Code>
 */
function ordered(?string $name): Collection
{
    return Code::query()->when($name, function (Builder $query): void {
        $query->orderBy('name');
    })->get();
}

/**
 * @return Collection<int, Code>
 */
function operator(?int $rank): Collection
{
    return Code::query()->when($rank, fn (Builder $query, int $rank) => $query->where('rank', '>', $rank))->get();
}

/**
 * @return Collection<int, Code>
 */
function enclosing(?int $rank, string $other): Collection
{
    return Code::query()->when($rank, fn (Builder $query) => $query->where('name', $other))->get();
}

/**
 * @return Collection<int, Code>
 */
function otherType(?string $name): Collection
{
    return Code::query()->when($name, fn (Builder $query, int $name) => $query->where('rank', $name))->get();
}

/**
 * @return Collection<int, Code>
 */
function threeParameters(?string $name): Collection
{
    return Code::query()->when($name, fn (Builder $query, string $name, int $extra) => $query->where('name', $name))->get();
}

// A property the model has no field for, as on a model that takes no struct (P2R-134).
/**
 * @return Collection<int, Code>
 */
function missingProperty(?Code $code): Collection
{
    return Code::query()->when($code, fn (Builder $query, Code $code) => $query->where('name', $code->missing))->get();
}

// Parameters PHP rejects and Mago does not report (P2R-451).
/**
 * @return Collection<int, Code>
 */
function thisParameter(?string $name): Collection
{
    return Code::query()->when($name, static function (Builder $this, string $name): void {
        $this->where('name', $name);
    })->get();
}

/**
 * @return Collection<int, Code>
 */
function superglobalParameter(?string $name): Collection
{
    return Code::query()->when($name, fn (Builder $query, string $_GET) => $query->where('name', 'a'))->get();
}

/**
 * @return Collection<int, Code>
 */
function doubledParameter(?string $name): Collection
{
    return Code::query()->when($name, fn ($query, $query) => $query->where('name', 'a'))->get();
}

// A second parameter typed false or ?string, which PHP checks against the value (P2R-451).
/**
 * @return Collection<int, Code>
 */
function falseParameter(bool $flag): Collection
{
    return Code::query()->when($flag, fn (Builder $query, false $flag) => $query->where('name', 'a'))->get();
}

/**
 * @return Collection<int, Code>
 */
function nullableParameter(?string $name): Collection
{
    return Code::query()->when($name, fn (Builder $query, ?string $name) => $query->where('name', $name))->get();
}

// A model value without null, always truthy, with no where on the second parameter.
/**
 * @return Collection<int, Code>
 */
function alwaysTruthy(Code $code): Collection
{
    return Code::query()->when($code, fn (Builder $query) => $query->where('name', 'a'))->get();
}

// A when link before a call that takes no filter.
function beforeFind(?string $name): Code
{
    return Code::query()->when($name, fn (Builder $query) => $query->where('name', 'a'))->findOrFail(1);
}

function beforeCreate(?string $name): Code
{
    return Code::query()->when($name, fn (Builder $query) => $query->where('name', 'a'))->create(['name' => 'a', 'rank' => 1]);
}

// unless() is no form of section 46.
/**
 * @return Collection<int, Code>
 */
function unlessForm(?string $name): Collection
{
    return Code::query()->unless($name, fn (Builder $query) => $query->where('name', 'a'))->get();
}
