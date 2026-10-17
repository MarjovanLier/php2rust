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

class HasOne extends Relation
{
}

class BelongsToMany extends Relation
{
}

class MorphToMany extends BelongsToMany
{
}

namespace Illuminate\Database\Eloquent;

use Illuminate\Database\Eloquent\Relations\BelongsTo;
use Illuminate\Database\Eloquent\Relations\HasMany;
use Illuminate\Database\Eloquent\Relations\HasOne;
use Illuminate\Database\Eloquent\Relations\MorphToMany;

class PendingHasThroughRelationship
{
}

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

    public function hasOne(string ...$names): HasOne
    {
        return new HasOne();
    }

    public function morphToMany(string ...$names): MorphToMany
    {
        return new MorphToMany();
    }

    public function through(string $name): PendingHasThroughRelationship
    {
        return new PendingHasThroughRelationship();
    }
}
