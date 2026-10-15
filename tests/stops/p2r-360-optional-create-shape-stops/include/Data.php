<?php

// A stand-in for spatie/laravel-data whose create() takes a parameter, which a call
// without arguments would fill with its default.

namespace Spatie\LaravelData;

class Optional
{
    public static function create(int $count = 0): Optional
    {
        return new self();
    }
}
