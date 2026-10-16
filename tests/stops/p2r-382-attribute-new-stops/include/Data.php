<?php

// A small stand-in for spatie/laravel-data 4.18 and an app rule: the parts these tests
// name. It is not the package, so the PHP run through prepend.php checks this stand-in.

namespace Spatie\LaravelData;

abstract class Data
{
}

namespace Spatie\LaravelData\Attributes\Validation;

#[\Attribute(\Attribute::TARGET_PROPERTY | \Attribute::TARGET_PARAMETER)]
final class Unique
{
    public function __construct(
        public string $table,
        public ?string $column = null,
        public mixed $ignore = null,
    ) {
    }
}

#[\Attribute(\Attribute::TARGET_PROPERTY | \Attribute::TARGET_PARAMETER)]
final class Rule
{
    /** @var list<mixed> */
    public array $rules;

    public function __construct(mixed ...$rules)
    {
        $this->rules = $rules;
    }
}

namespace Spatie\LaravelData\Support\Validation\References;

class RouteParameterReference
{
    public function __construct(
        public readonly string $routeParameter,
        public readonly ?string $property = null,
    ) {
    }
}

namespace App\Rules;

final class NoSqlInjectionRule
{
}
