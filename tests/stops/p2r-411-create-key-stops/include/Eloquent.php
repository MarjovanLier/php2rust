<?php

// A small stand-in for Eloquent: the parts these tests name. It is not the framework. As in
// Laravel, the model has no static create or find: __callStatic forwards them to a query.

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

trait SoftDeletes
{
}

/**
 * @template TModel of Model
 */
final class Builder
{
    /**
     * @return $this
     */
    public function where(string $column, mixed $value): static
    {
        return $this;
    }

    /**
     * @param list<string> $columns
     * @return TModel|null
     */
    public function first(array $columns = ['*']): ?Model
    {
        return null;
    }

    /**
     * @return TModel|null
     */
    public function find(int $id): ?Model
    {
        return null;
    }

    /**
     * @param array<string, mixed> $attributes
     * @return TModel
     */
    public function create(array $attributes): Model
    {
        throw new \LogicException('a stand-in');
    }
}
