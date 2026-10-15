<?php

declare(strict_types=1);

namespace App;

use Spatie\LaravelData\Optional;

// Optional alone has no Rust type.
function alone(): Optional
{
    return new Optional();
}

function made(): int
{
    $marker = new Optional();
    return 1;
}

function side(): int
{
    return 1;
}

// An argument to new Optional runs, so it stops.
function argued(): string|Optional
{
    return new Optional(side());
}

// A value of another PhpOptional type does not go into this one.
function widened(string|Optional $value): string|Optional|null
{
    return $value;
}

// The arms of ?: must give one Rust type.
function picked(bool $c): string|Optional
{
    return $c ? 'a' : new Optional();
}

// Illuminate's optional is another class.
function other(): string|Optional
{
    return \Illuminate\Support\Optional::create();
}
