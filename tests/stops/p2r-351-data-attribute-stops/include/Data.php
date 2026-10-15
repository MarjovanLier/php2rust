<?php

// A small stand-in for spatie/laravel-data 4.18: the parts these tests name.

namespace Spatie\LaravelData;

abstract class Data
{
}

namespace Spatie\LaravelData\Attributes;

#[\Attribute(\Attribute::TARGET_CLASS | \Attribute::TARGET_PROPERTY | \Attribute::TARGET_PARAMETER)]
final class MapInputName
{
    public function __construct(public string $input)
    {
    }
}
