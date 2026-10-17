<?php

// A small stand-in for Eloquent: the parts these tests name. It is not the framework. Its
// Model declares getRouteKeyName as private, over a public one of a trait it uses, so PHP
// sees only the private one and does not count it for an #[\Override] in a model.

namespace Illuminate\Database\Eloquent\Concerns;

trait HasRouteKeys
{
    public function getRouteKeyName(): string
    {
        return 'id';
    }
}

namespace Illuminate\Database\Eloquent;

use Illuminate\Database\Eloquent\Concerns\HasRouteKeys;

abstract class Model
{
    use HasRouteKeys;

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
