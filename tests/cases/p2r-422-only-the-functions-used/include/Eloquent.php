<?php

// A small stand-in for Eloquent: the parts these tests name, over rows kept in memory, as an
// empty database without constraints would give them. It is not the framework. As in
// Laravel, the model has no static create or find: __callStatic forwards them to a query.
// The exceptions carry Laravel 12.35.1's classes and messages.

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

namespace Illuminate\Database\Eloquent;

use Illuminate\Database\MultipleRecordsFoundException;
use Illuminate\Database\RecordsNotFoundException;

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
     * @return list<TModel>
     */
    private function matching(int $limit): array
    {
        $found = [];
        foreach (Model::rowsOf($this->class) as $row) {
            $matches = true;
            foreach ($this->wheres as [$column, $value]) {
                $matches = $matches && ($row[$column] ?? null) === $value;
            }
            if ($matches && count($found) < $limit) {
                $found[] = ($this->class)::fromRow($row);
            }
        }
        return $found;
    }

    /**
     * @return TModel|null
     */
    public function first(): ?Model
    {
        return $this->matching(1)[0] ?? null;
    }

    /**
     * @return TModel
     */
    public function firstOrFail(): Model
    {
        return $this->first() ?? throw (new ModelNotFoundException())->setModel($this->class);
    }

    /**
     * As Laravel's sole(), which reads at most two rows.
     *
     * @return TModel
     */
    public function sole(): Model
    {
        $found = $this->matching(2);
        if (count($found) === 0) {
            throw (new ModelNotFoundException())->setModel($this->class);
        }
        if (count($found) > 1) {
            throw new MultipleRecordsFoundException(count($found));
        }
        return $found[0];
    }

    /**
     * @return TModel|null
     */
    public function find(int $id): ?Model
    {
        return $this->where('id', $id)->first();
    }

    /**
     * @return TModel
     */
    public function findOrFail(int $id): Model
    {
        return $this->find($id) ?? throw (new ModelNotFoundException())->setModel($this->class, [$id]);
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
