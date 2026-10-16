<?php

declare(strict_types=1);

namespace App;

use Spatie\LaravelData\Attributes\Validation\Unique;
use Spatie\LaravelData\Data;
use Spatie\LaravelData\Support\Validation\References\RouteParameterReference;

final class Forms extends Data
{
    public function __construct(
        // Only a class name: not self, parent, static or an expression.
        #[Unique(table: 't', ignore: new self('x', 'y', 'z', 'v', 'w'))]
        public string $a,
        #[Unique(table: 't', ignore: new (RouteParameterReference::class)('x'))]
        public string $b,
        // No `new` inside `new`.
        #[Unique(table: 't', ignore: new RouteParameterReference(new RouteParameterReference('x')))]
        public string $c,
        // PHP rejects a positional argument after a named one.
        #[Unique(table: 't', ignore: new RouteParameterReference(property: 'p', 'x'))]
        public string $d,
        // PHP rejects a call in a constant expression.
        #[Unique(table: 't', ignore: new RouteParameterReference(strtoupper('x')))]
        public string $e,
    ) {}
}

// A Mago finding on it still stops.
final class Unknown extends Data
{
    public function __construct(
        #[Unique(table: 't', ignore: new Missing('x'))]
        public string $a,
    ) {}
}
