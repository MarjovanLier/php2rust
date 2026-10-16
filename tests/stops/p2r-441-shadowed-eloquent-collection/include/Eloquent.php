<?php

// A small stand-in for Eloquent: the parts this test names. It is not the framework.

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
    /**
     * @return Collection<int, static>
     */
    public static function all(): Collection
    {
        return new Collection([]);
    }
}
