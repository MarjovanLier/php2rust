<?php

// A small stand-in for spatie/laravel-data 4.18: the parts these tests name, not the
// package. As in the package, the methods come from a trait.

namespace Spatie\LaravelData;

abstract class Data
{
    use Concerns\BaseData;

    protected array $_additional = [];

    protected mixed $_dataContext = null;
}

namespace Spatie\LaravelData\Concerns;

trait BaseData
{
    public static function from(mixed ...$payloads): static
    {
        return new static(...$payloads);
    }

    /** @return array<string, mixed> */
    public function toArray(): array
    {
        return get_object_vars($this);
    }
}
