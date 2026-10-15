<?php

// A small stand-in for spatie/laravel-data 4.18: the parts these tests name, not the
// package.

namespace Spatie\LaravelData;

abstract class Data
{
}

class Optional
{
    public static function create(): Optional
    {
        return new self();
    }
}
