<?php

// A stand-in for spatie/laravel-data whose Data has real methods named like five hooks,
// one in a trait, one renamed from a trait and one in an interface under an @method tag
// of Data, which version 4.18.0 does not have.

namespace Spatie\LaravelData\Concerns;

trait Bags
{
    public function errorBag(): string
    {
        return 'default';
    }

    public static function target(int $a): string
    {
        return 'home';
    }
}

namespace Spatie\LaravelData\Contracts;

interface HasMessages
{
    /** @return array<string, string> */
    public static function messages(int $a): array;
}

namespace Spatie\LaravelData;

/**
 * @method static string errorBag()
 * @method static array messages(...$args)
 * @method static string redirect()
 */
abstract class Data implements Contracts\HasMessages
{
    use Concerns\Bags {
        target as redirect;
    }

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
