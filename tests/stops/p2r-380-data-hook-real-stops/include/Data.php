<?php

// A stand-in for spatie/laravel-data whose Data has real methods named like three hooks,
// one of them in a trait under an @method tag of Data, which version 4.18.0 does not
// have.

namespace Spatie\LaravelData\Concerns;

trait Bags
{
    public function errorBag(): string
    {
        return 'default';
    }
}

namespace Spatie\LaravelData;

/**
 * @method static string errorBag()
 */
abstract class Data
{
    use Concerns\Bags;

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
