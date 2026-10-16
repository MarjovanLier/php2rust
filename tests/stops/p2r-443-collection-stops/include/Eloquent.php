<?php

// A small stand-in for Eloquent: the parts these tests name, over rows kept in memory, as an
// empty database without constraints would give them. It is not the framework. The
// collections hold their items in an array, as Laravel's do, with the methods these tests
// call, and get() and all() take the columns to load, as Laravel's do.

namespace Illuminate\Support;

/**
 * @template TKey of array-key
 * @template TValue
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

    public function isEmpty(): bool
    {
        return $this->items === [];
    }

    /**
     * @param (callable(TValue): bool)|null $callback
     * @return TValue|null
     */
    public function first(?callable $callback = null): mixed
    {
        foreach ($this->items as $item) {
            if ($callback === null || $callback($item)) {
                return $item;
            }
        }
        return null;
    }

    /**
     * @return array<TKey, TValue>
     */
    public function all(): array
    {
        return $this->items;
    }

    /**
     * @param TValue $value
     * @return $this
     */
    public function push(mixed $value): static
    {
        $this->items[] = $value;
        return $this;
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
     * @return Builder<static>
     */
    public static function query(): Builder
    {
        return new Builder(static::class);
    }

    /**
     * @param list<string> $columns
     * @return Collection<int, static>
     */
    public static function all(array $columns = ['*']): Collection
    {
        return static::query()->get($columns);
    }

    /**
     * @param list<mixed> $arguments
     */
    public static function __callStatic(string $method, array $arguments): mixed
    {
        return static::query()->$method(...$arguments);
    }

    /**
     * @param array<string, mixed> $attributes
     */
    public static function insertRow(array $attributes): static
    {
        $id = count(self::$rows[static::class] ?? []) + 1;
        self::$rows[static::class][] = ['id' => $id] + $attributes;
        return static::fromRow($attributes + ['id' => $id]);
    }

    /**
     * @return list<array<string, mixed>>
     */
    public static function rowsOf(string $class): array
    {
        return self::$rows[$class] ?? [];
    }

    /**
     * @param array<string, mixed> $row
     */
    public static function fromRow(array $row): static
    {
        $model = new static();
        $model->attributes = $row;
        return $model;
    }
}

/**
 * @template TModel of Model
 */
final class Builder
{
    /** @var list<array{string, mixed}> */
    private array $wheres = [];

    /**
     * @param class-string<TModel> $class
     */
    public function __construct(private string $class)
    {
    }

    /**
     * @return $this
     */
    public function where(string $column, mixed $value): static
    {
        $this->wheres[] = [$column, $value];
        return $this;
    }

    /**
     * @param list<string> $columns
     * @return Collection<int, TModel>
     */
    public function get(array $columns = ['*']): Collection
    {
        $found = [];
        foreach (Model::rowsOf($this->class) as $row) {
            $matches = true;
            foreach ($this->wheres as [$column, $value]) {
                $matches = $matches && ($row[$column] ?? null) === $value;
            }
            if ($matches) {
                $found[] = ($this->class)::fromRow($columns === ['*'] ? $row : array_intersect_key($row, array_flip($columns)));
            }
        }
        return new Collection($found);
    }

    /**
     * @param array<string, mixed> $attributes
     * @return TModel
     */
    public function create(array $attributes): Model
    {
        return ($this->class)::insertRow($attributes);
    }
}
