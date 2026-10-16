<?php

// A small stand-in for spatie/laravel-data 4.18: the parts these tests name. It is not
// the package, so the PHP run through prepend.php checks this stand-in, not 4.18.

namespace Spatie\LaravelData;

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

namespace Spatie\LaravelData\Attributes;

#[\Attribute(\Attribute::TARGET_CLASS | \Attribute::TARGET_PROPERTY | \Attribute::TARGET_PARAMETER)]
final class MapInputName
{
    public function __construct(public string $input)
    {
    }
}

#[\Attribute(\Attribute::TARGET_CLASS | \Attribute::TARGET_PROPERTY | \Attribute::TARGET_PARAMETER)]
final class MapOutputName
{
    public function __construct(public string $output)
    {
    }
}

namespace Spatie\LaravelData\Attributes\Validation;

#[\Attribute(\Attribute::TARGET_PROPERTY | \Attribute::TARGET_PARAMETER)]
final class Max
{
    public function __construct(public int $value)
    {
    }
}

#[\Attribute(\Attribute::TARGET_PROPERTY | \Attribute::TARGET_PARAMETER)]
final class Required
{
}
