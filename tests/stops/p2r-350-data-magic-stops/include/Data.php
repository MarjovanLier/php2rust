<?php

// A stand-in for a Data that is not laravel-data 4.18: PHP calls __destruct by itself.

namespace Spatie\LaravelData;

abstract class Data
{
    public function __destruct()
    {
        echo "bye\n";
    }
}
