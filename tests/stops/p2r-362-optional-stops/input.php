<?php

declare(strict_types=1);

namespace App;

use Spatie\LaravelData\Data;
use Spatie\LaravelData\Optional;

final class Contact extends Data
{
    public function __construct(
        public string|Optional|null $email,
        public int|Optional $visits,
    ) {}
}

function length(string $text): int
{
    return strlen($text);
}

// Reading the value inside waits for a check that narrows the type.
function compared(Contact $c): bool
{
    return $c->email === null;
}

function defaulted(Contact $c): string
{
    return $c->email ?? 'none';
}

function joined(Contact $c): string
{
    return 'visits ' . $c->visits;
}

function passed(Contact $c): int
{
    return length($c->email);
}

function shown(Contact $c): void
{
    echo $c->visits;
}

final class Named extends Data
{
    public function __construct(
        public string|Optional $name,
    ) {}
}

// A value that may be Optional where a string is due.
function given(Named $n): int
{
    return length($n->name);
}

function copied(Named $n): string
{
    $name = $n->name;
    return $name;
}

// More than one other type, and a list of them, stop.
function wide(int|string|Optional $value): int
{
    return 1;
}

/** @param list<int|Optional> $values */
function many(array $values): int
{
    return 1;
}

// instanceof Optional needs a value of type T|Optional.
function plain(string $text): bool
{
    return $text instanceof Optional;
}
