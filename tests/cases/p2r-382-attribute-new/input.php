<?php

declare(strict_types=1);

namespace App;

use App\Rules\NoSqlInjectionRule;
use Spatie\LaravelData\Attributes\Validation\Rule;
use Spatie\LaravelData\Attributes\Validation\Unique;
use Spatie\LaravelData\Data;
use Spatie\LaravelData\Support\Validation\References\RouteParameterReference;

final class Administration extends Data
{
    public function __construct(
        #[Unique(table: 'administrations', column: 'name', ignore: new RouteParameterReference('environment', 'uuid'))]
        public string $name,
        #[Rule(new NoSqlInjectionRule)]
        public string $code,
    ) {}
}

// PHP runs `new` only when reflection reads the arguments, so a class of the input works too.
final class Plain
{
    public function __construct(
        #[Unique(table: 'plain', ignore: new Administration(name: 'a', code: 'b'))]
        public string $label,
    ) {}
}

$administration = new Administration('Main', 'M1');
$plain = new Plain('p');
echo $administration->name, ' ', $administration->code, ' ', $plain->label, "\n";
