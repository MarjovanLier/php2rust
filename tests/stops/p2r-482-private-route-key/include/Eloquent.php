<?php

// A small stand-in for Eloquent: the parts these tests name. It is not the framework. Its
// Model declares getRouteKeyName as private, which PHP does not count for an #[\Override]
// in a model.

namespace Illuminate\Database\Eloquent;

abstract class Model
{
    /** @var array<string, mixed> */
    private array $attributes = [];

    public function __get(string $key): mixed
    {
        return $this->attributes[$key] ?? null;
    }

    private function getRouteKeyName(): string
    {
        return 'id';
    }
}
