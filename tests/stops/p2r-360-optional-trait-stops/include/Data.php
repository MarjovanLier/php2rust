<?php

// A stand-in for spatie/laravel-data whose Optional is a trait, not a class.

namespace Spatie\LaravelData;

trait Optional
{
    public static function create(): string
    {
        return 'x';
    }
}
