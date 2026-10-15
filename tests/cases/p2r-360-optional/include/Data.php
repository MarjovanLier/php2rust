<?php

// A small stand-in for spatie/laravel-data 4.18: the parts these tests name. It is not
// the package, so the PHP run through prepend.php checks this stand-in, not 4.18.

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
