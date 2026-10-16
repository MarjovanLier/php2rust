<?php

// A small stand-in for Eloquent: the parts these tests name, over rows kept in memory, as an
// empty database without constraints would give them. It is not the framework. As in
// Laravel, the model has no static create or find: __callStatic forwards them to a query.

namespace Illuminate\Database\Eloquent;

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
     * @return TModel|null
     */
    public function first(): ?Model
    {
        foreach (Model::rowsOf($this->class) as $row) {
            $matches = true;
            foreach ($this->wheres as [$column, $value]) {
                $matches = $matches && ($row[$column] ?? null) === $value;
            }
            if ($matches) {
                return ($this->class)::fromRow($row);
            }
        }
        return null;
    }

    /**
     * @return TModel|null
     */
    public function find(int $id): ?Model
    {
        return $this->where('id', $id)->first();
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
