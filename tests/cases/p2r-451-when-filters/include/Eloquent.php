<?php

// A small stand-in for Eloquent: the parts these tests name, over rows kept in memory, as an
// empty database without constraints would give them. It is not the framework. As in
// Laravel, the model has no static create or find (__callStatic forwards them), and the
// builder takes when() from Laravel's Conditionable trait with Laravel's docblock.

namespace Illuminate\Database;

class RecordsNotFoundException extends \RuntimeException
{
}

class MultipleRecordsFoundException extends \RuntimeException
{
    public function __construct(int $count)
    {
        parent::__construct("$count records were found.");
    }
}

namespace Illuminate\Support\Traits;

trait Conditionable
{
    /**
     * Apply the callback if the given "value" is (or resolves to) truthy.
     *
     * @template TWhenParameter
     * @template TWhenReturnType
     *
     * @param  (\Closure($this): TWhenParameter)|TWhenParameter|null  $value
     * @param  (callable($this, TWhenParameter): TWhenReturnType)|null  $callback
     * @param  (callable($this, TWhenParameter): TWhenReturnType)|null  $default
     * @return $this|TWhenReturnType
     */
    public function when($value = null, ?callable $callback = null, ?callable $default = null)
    {
        $value = $value instanceof \Closure ? $value($this) : $value;
        if ($value) {
            return $callback($this, $value) ?? $this;
        } elseif ($default) {
            return $default($this, $value) ?? $this;
        }
        return $this;
    }
}

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

use Illuminate\Database\MultipleRecordsFoundException;
use Illuminate\Database\RecordsNotFoundException;
use Illuminate\Support\Traits\Conditionable;

class ModelNotFoundException extends RecordsNotFoundException
{
    /**
     * @param list<int> $ids
     */
    public function setModel(string $model, array $ids = []): static
    {
        $this->message = "No query results for model [{$model}]";
        $this->message .= count($ids) > 0 ? ' ' . implode(', ', $ids) : '.';
        return $this;
    }
}

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
    use Conditionable;

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
        foreach ($this->get() as $model) {
            return $model;
        }
        return null;
    }

    /**
     * As Laravel's sole(), which reads at most two rows.
     *
     * @return TModel
     */
    public function sole(): Model
    {
        $found = $this->get();
        if (count($found) === 0) {
            throw (new ModelNotFoundException())->setModel($this->class);
        }
        if (count($found) > 1) {
            throw new MultipleRecordsFoundException(2);
        }
        return $this->first() ?? throw (new ModelNotFoundException())->setModel($this->class);
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
