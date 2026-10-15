<?php

// A stand-in for spatie/laravel-data whose create() may return null, so a call to it is
// not Optional alone.

namespace Spatie\LaravelData;

class Optional
{
    public static function create(): ?Optional
    {
        return null;
    }
}
