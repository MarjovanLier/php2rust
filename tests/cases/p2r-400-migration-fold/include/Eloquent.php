<?php

// A small stand-in for Eloquent: the parts these tests name. It is not the framework.

namespace Illuminate\Database\Eloquent;

abstract class Model
{
    public bool $exists = false;

    public function __get(string $key): mixed
    {
        return null;
    }
}

trait SoftDeletes
{
}

namespace Illuminate\Database\Eloquent\Factories;

trait HasFactory
{
}
