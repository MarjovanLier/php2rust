<?php

// A small stand-in for Eloquent whose builder uses Laravel's Conditionable trait but declares
// its own when(), so a when() link is not Laravel's (P2R-450, P2R-451). Only the types matter.

namespace Illuminate\Support\Traits;

trait Conditionable
{
    /**
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

use Illuminate\Support\Traits\Conditionable;

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
    /**
     * @return Builder<static>
     */
    public static function query(): Builder
    {
        return new Builder();
    }
}

/**
 * @template TModel of Model
 */
final class Builder
{
    use Conditionable;

    /**
     * A builder's own when, which applies the callback when the value is falsy.
     *
     * @return $this
     */
    public function when(mixed $value, callable $callback): static
    {
        if (! $value) {
            $callback($this, $value);
        }
        return $this;
    }

    /**
     * @return $this
     */
    public function where(string $column, mixed $value): static
    {
        return $this;
    }

    /**
     * @return Collection<int, TModel>
     */
    public function get(): Collection
    {
        return new Collection([]);
    }
}
