<?php

// A small stand-in for Eloquent with Laravel's ObservedBy attribute and observers that have
// a retrieved method in each form method_exists finds, and two forms only Mago's lookup
// finds (P2R-461). Only the types matter.

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

trait HasEvents
{
    /**
     * As Laravel's, which registers a retrieved listener; a model named as an observer
     * therefore has a retrieved method.
     */
    public static function retrieved(mixed $callback): void
    {
    }
}

abstract class Model
{
    use HasEvents;

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
     * @return TModel|null
     */
    public function first(): ?Model
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
        throw new \LogicException('not used');
    }
}

namespace App\Observers;

use Illuminate\Database\Eloquent\Model;

final class CodeObserver
{
    public function created(Model $model): void
    {
    }
}

final class OwnObserver
{
    public function retrieved(Model $model): void
    {
    }
}

final class UpperObserver
{
    public function Retrieved(Model $model): void
    {
    }
}

class BaseObserver
{
    public function retrieved(Model $model): void
    {
    }
}

final class InheritedObserver extends BaseObserver
{
}

trait Watches
{
    public function retrieved(Model $model): void
    {
    }
}

final class TraitObserver
{
    use Watches;
}

final class HiddenObserver
{
    private function retrieved(Model $model): void
    {
    }
}

// PHP's method_exists finds neither of these, but Mago's lookup does, so the tool stops.
class PrivateBase
{
    private function retrieved(Model $model): void
    {
    }
}

final class PrivateParentObserver extends PrivateBase
{
}

/**
 * @method void retrieved(Model $model)
 */
final class TagObserver
{
}
