<?php

// A small stand-in for Eloquent: the parts these tests name. It is not the framework.

namespace Illuminate\Database\Eloquent;

abstract class Model
{
    public function __get(string $key): mixed
    {
        return null;
    }

    protected static function booted(): void
    {
    }
}

namespace Illuminate\Database\Eloquent\Attributes;

#[\Attribute(\Attribute::TARGET_CLASS)]
final class ObservedBy
{
    /** @param list<class-string> $classes */
    public function __construct(public array $classes)
    {
    }
}

namespace Illuminate\Database\Eloquent\Casts;

final class Attribute
{
}

namespace Illuminate\Database\Eloquent\Relations;

class BelongsTo
{
}
