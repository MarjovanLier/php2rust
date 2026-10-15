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

// A variable keeps one type, so a later null does not go into it.
function blanked(string|Optional|null $email, bool $wipe): string|Optional|null
{
    $copy = $email;
    if ($wipe) {
        $copy = null;
    }
    return $copy;
}

// The first write of a variable gives a match no type for new Optional.
function chosen(int $k): bool
{
    $v = match ($k) {
        1 => Optional::create(),
        default => 'a',
    };
    return $v instanceof Optional;
}
