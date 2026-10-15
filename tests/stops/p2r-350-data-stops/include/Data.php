<?php

// A small stand-in for spatie/laravel-data 4.18: the parts these tests name.

namespace Spatie\LaravelData;

abstract class Data
{
    protected array $_additional = [];

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
