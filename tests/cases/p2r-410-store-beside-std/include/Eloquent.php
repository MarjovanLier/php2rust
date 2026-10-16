<?php

// A small stand-in for Eloquent: the parts these tests name. It is not the framework. As in
// Laravel, the model has no static find: __callStatic forwards it to a query.

namespace Illuminate\Database\Eloquent;

abstract class Model
{
    public function __get(string $key): mixed
    {
        return null;
    }

    /**
     * @return Builder<static>
     */
    public static function query(): Builder
    {
        return new Builder();
    }

    /**
     * @param list<mixed> $arguments
     */
    public static function __callStatic(string $method, array $arguments): mixed
    {
        return static::query()->$method(...$arguments);
    }
}

/**
 * @template TModel of Model
 */
final class Builder
{
    /**
     * @return TModel|null
     */
    public function find(int $id): ?Model
    {
        return null;
    }
}
