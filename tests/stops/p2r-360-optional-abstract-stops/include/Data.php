<?php

// A stand-in for spatie/laravel-data whose Optional is abstract, which PHP cannot make.

namespace Spatie\LaravelData;

abstract class Optional
{
    public static function create(): Optional
    {
        return new static();
    }
}
