<?php

// A small stand-in for Eloquent: the parts these tests name. It is not the framework. Its
// Model has no casts(), as Laravel 10's has not.

namespace Illuminate\Database\Eloquent;

abstract class Model
{
    /** @var array<string, mixed> */
    private array $attributes = [];

    public function __get(string $key): mixed
    {
        return $this->attributes[$key] ?? null;
    }
}
