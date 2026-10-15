<?php

declare(strict_types=1);

namespace App;

use Spatie\LaravelData\Attributes\MapInputName;
use Spatie\LaravelData\Data;

// Left out: only laravel-data's attributes on a Data class.
final class Kept extends Data
{
    public function __construct(
        #[MapInputName('data.code')]
        public string $code,
    ) {}
}

// Another attribute in the same group stops the whole group.
final class Grouped extends Data
{
    public function __construct(
        #[MapInputName('data.code'), \SensitiveParameter]
        public string $code,
    ) {}
}

#[\AllowDynamicProperties]
final class Dynamic extends Data
{
}

// laravel-data's attributes on a class that does not extend Data stop.
final class Plain
{
    public function __construct(
        #[MapInputName('data.code')]
        public string $code,
    ) {}
}
