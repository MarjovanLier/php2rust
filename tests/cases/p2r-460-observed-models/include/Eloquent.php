<?php

// A small stand-in for Eloquent: the parts these tests name, over rows kept in memory, as an
// empty database without constraints would give them. It is not the framework. As in
// Laravel, the model has no static create or find: __callStatic forwards them to a query.
// It declares Laravel's ObservedBy attribute and an observer that reacts to writes only.

namespace Illuminate\Database;

class RecordsNotFoundException extends \RuntimeException
{
}

namespace Illuminate\Database\Eloquent\Attributes;

#[\Attribute(\Attribute::TARGET_CLASS | \Attribute::IS_REPEATABLE)]
class ObservedBy
{
    /**
     * @param array<int, string>|string $classes
     */
    public function __construct(public array|string $classes)
    {
    }
}

namespace Illuminate\Database\Eloquent;

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
     * @return TModel
     */
    public function findOrFail(int $id): Model
    {
        return $this->find($id) ?? throw (new ModelNotFoundException())->setModel($this->class, [$id]);
    }
}

namespace App\Observers;

use Illuminate\Database\Eloquent\Model;

// An observer as the app's are: it reacts to create, update and delete, never to a read.
final class CodeObserver
{
    public function created(Model $model): void
    {
    }

    public function updated(Model $model): void
    {
    }

    public function deleted(Model $model): void
    {
    }
}
