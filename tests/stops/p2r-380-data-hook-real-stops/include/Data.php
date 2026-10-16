<?php

// A stand-in for spatie/laravel-data whose Data has real methods named like two hooks,
// which version 4.18.0 does not have.

namespace Spatie\LaravelData;

abstract class Data
{
    /** @return array<string, mixed> */
    public function rules(): array
    {
        return [];
    }

    final public static function authorize(): bool
    {
        return true;
    }
}
