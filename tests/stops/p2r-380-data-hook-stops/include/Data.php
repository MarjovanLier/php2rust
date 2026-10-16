<?php

// A small stand-in for spatie/laravel-data 4.18: the parts these tests name. It is not
// the package, so the PHP run through prepend.php checks this stand-in, not 4.18. The
// @method tags are those of its ValidateableData trait.

namespace Spatie\LaravelData;

/**
 * @method static array rules(?\Spatie\LaravelData\Support\Validation\ValidationContext $context = null)
 * @method static array messages(...$args)
 * @method static array attributes(...$args)
 * @method static bool stopOnFirstFailure()
 * @method static string redirect()
 * @method static string redirectRoute()
 * @method static string errorBag()
 */
abstract class Data
{
    protected array $_additional = [];

    public static function from(mixed ...$payloads): static
    {
        return new static(...$payloads);
    }

    /** @return array<string, mixed> */
    public function toArray(): array
    {
        return get_object_vars($this);
    }
}

namespace Spatie\LaravelData\Support\Validation;

final class ValidationContext
{
    /** @param array<string, mixed> $payload */
    public function __construct(public array $payload)
    {
    }
}
