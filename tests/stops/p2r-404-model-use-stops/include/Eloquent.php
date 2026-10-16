<?php

// A small stand-in for Eloquent: the parts these tests name. It is not the framework.

namespace Illuminate\Database\Eloquent;

abstract class Model
{
    public function __get(string $key): mixed
    {
        return null;
    }

    public function __set(string $key, mixed $value): void
    {
    }

    public function save(): bool
    {
        return true;
    }

    /** @return Builder<static> */
    public static function query(): Builder
    {
        return new Builder();
    }
}

/**
 * @template TModel
 */
final class Builder
{
}
