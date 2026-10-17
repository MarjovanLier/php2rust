<?php

// A small stand-in for Eloquent: the parts these tests name. It is not the framework. Its
// Model declares no getRouteKeyName itself and takes one from a trait, which P2R-480 does
// not count for an #[\Override]. It declares relation classes with the links these tests
// chain.

namespace Illuminate\Database\Eloquent\Relations;

abstract class Relation
{
    public function limit(int $value): static
    {
        return $this;
    }

    public function with(string ...$relations): static
    {
        return $this;
    }

    public function latest(): static
    {
        return $this;
    }

    public function oldest(): static
    {
        return $this;
    }
}

class HasMany extends Relation
{
}

class BelongsToMany extends Relation
{
    public function withTimestamps(): static
    {
        return $this;
    }

    public function withPivot(string ...$columns): static
    {
        return $this;
    }
}

namespace Illuminate\Database\Eloquent\Factories;

// A trait that declares getRouteKeyName; PHP does not count it for an #[\Override] in the
// model that uses it.
trait HasFactory
{
    public function getRouteKeyName(): string
    {
        return 'id';
    }
}

namespace Illuminate\Database\Eloquent\Concerns;

trait HasRouteKeys
{
    public function getRouteKeyName(): string
    {
        return 'id';
    }
}

namespace Illuminate\Database\Eloquent;

use Illuminate\Database\Eloquent\Concerns\HasRouteKeys;
use Illuminate\Database\Eloquent\Relations\BelongsToMany;
use Illuminate\Database\Eloquent\Relations\HasMany;

abstract class Model
{
    use HasRouteKeys;

    /** @var array<string, mixed> */
    private array $attributes = [];

    public function __get(string $key): mixed
    {
        return $this->attributes[$key] ?? null;
    }

    public function hasMany(string ...$names): HasMany
    {
        return new HasMany();
    }

    public function belongsToMany(string ...$names): BelongsToMany
    {
        return new BelongsToMany();
    }
}
