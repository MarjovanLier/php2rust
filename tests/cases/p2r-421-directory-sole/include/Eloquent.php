<?php

// A small stand-in for Eloquent: the parts these tests name. It is not the framework. As in
// Laravel, the model has no static findOrFail: __callStatic forwards it to a query.

namespace Illuminate\Database;

class RecordsNotFoundException extends \RuntimeException
{
}

namespace Illuminate\Database\Eloquent;

use Illuminate\Database\RecordsNotFoundException;

class ModelNotFoundException extends RecordsNotFoundException
{
}

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
     * @return $this
     */
    public function where(string $column, mixed $value): static
    {
        return $this;
    }

    /**
     * @return TModel
     */
    public function sole(): Model
    {
        throw new \LogicException('a stand-in');
    }

    /**
     * @return TModel
     */
    public function findOrFail(int $id): Model
    {
        throw new \LogicException('a stand-in');
    }
}
