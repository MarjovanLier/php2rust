<?php

// A small stand-in for Eloquent: the parts these tests name, over rows kept in memory, as an
// empty database without constraints would give them. It is not the framework. As in
// Laravel, the model has no static create or find: __callStatic forwards them to a query.
// The collections hold their items in an array, as Laravel's do, with the methods these
// tests call.

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

    public function isEmpty(): bool
    {
        return $this->items === [];
    }

    public function isNotEmpty(): bool
    {
        return !$this->isEmpty();
    }

    /**
     * @return TValue|null
     */
    public function first(): mixed
    {
        foreach ($this->items as $item) {
            return $item;
        }
        return null;
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
     * @return Collection<int, static>
     */
    public static function all(): Collection
    {
        return static::query()->get();
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
     * @return Collection<int, TModel>
     */
    public function get(): Collection
    {
        $found = [];
        foreach (Model::rowsOf($this->class) as $row) {
            $matches = true;
            foreach ($this->wheres as [$column, $value]) {
                $matches = $matches && ($row[$column] ?? null) === $value;
            }
            if ($matches) {
                $found[] = ($this->class)::fromRow($row);
            }
        }
        return new Collection($found);
    }

    /**
     * @return TModel|null
     */
    public function first(): ?Model
    {
        return $this->get()->first();
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
