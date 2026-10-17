<?php

// A small stand-in for Eloquent: the parts these tests name. It is not the framework. It
// declares Laravel's relation classes and the helpers that build them.

namespace Illuminate\Database\Eloquent\Relations;

abstract class Relation
{
}

class BelongsTo extends Relation
{
}

class HasMany extends Relation
{
}

namespace Illuminate\Database\Eloquent;

use Illuminate\Database\Eloquent\Relations\BelongsTo;
use Illuminate\Database\Eloquent\Relations\HasMany;

abstract class Model
{
    /** @var array<string, mixed> */
    private array $attributes = [];

    public function __get(string $key): mixed
    {
        return $this->attributes[$key] ?? null;
    }

    public function belongsTo(string ...$names): BelongsTo
    {
        return new BelongsTo();
    }

    public function hasMany(string ...$names): HasMany
    {
        return new HasMany();
    }
}
