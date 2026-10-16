<?php

// A small stand-in for Eloquent: the parts this test names, over rows kept in memory, as an
// empty database without constraints would give them. It is not the framework.

namespace Illuminate\Support;

/**
 * @template TKey of array-key
 * @template-covariant TValue
 * @implements \IteratorAggregate<TKey, TValue>
 */
class Collection implements \Countable, \IteratorAggregate
{
    /**
     * @param array<TKey, TValue> $items
     */
    public function __construct(protected array $items = [])
    {
    }

    public function count(): int
    {
        return count($this->items);
    }

    /**
     * @return \ArrayIterator<TKey, TValue>
     */
    public function getIterator(): \ArrayIterator
    {
        return new \ArrayIterator($this->items);
    }
}

namespace Illuminate\Database\Eloquent;

/**
 * @template TKey of array-key
 * @template TModel of Model
 * @extends \Illuminate\Support\Collection<TKey, TModel>
 */
class Collection extends \Illuminate\Support\Collection
{
}

abstract class Model
{
    /** @var array<string, list<array<string, mixed>>> */
    private static array $rows = [];

    /** @var array<string, mixed> */
    private array $attributes = [];

    public function __get(string $key): mixed
    {
        return $this->attributes[$key] ?? null;
    }

    /**
     * @return Collection<int, static>
     */
    public static function all(): Collection
    {
        $found = [];
        foreach (self::$rows[static::class] ?? [] as $row) {
            $model = new static();
            $model->attributes = $row;
            $found[] = $model;
        }
        return new Collection($found);
    }

    /**
     * @param list<mixed> $arguments
     */
    public static function __callStatic(string $method, array $arguments): mixed
    {
        if ($method !== 'create') {
            throw new \BadMethodCallException($method);
        }
        $id = count(self::$rows[static::class] ?? []) + 1;
        self::$rows[static::class][] = ['id' => $id] + $arguments[0];
        return null;
    }
}
