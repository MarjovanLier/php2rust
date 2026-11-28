<?php

declare(strict_types=1);

namespace App;

use Spatie\LaravelData\Data;
use Spatie\LaravelData\Optional;

// A laravel-data class with Optional properties, written by hand for the harness.
final class Contact extends Data
{
    public function __construct(
        public string $name,
        public string|Optional $email,
        public int|Optional $visits,
    ) {}
}

function emailOf(?string $email): string|Optional
{
    if ($email === null) {
        return Optional::create();
    }
    return $email;
}

function visitsOf(int $visits): int|Optional
{
    if ($visits > 0) {
        return $visits;
    }
    return Optional::create();
}

function described(string $name, ?string $email, int $visits): string
{
    $contact = new Contact($name, emailOf($email), visitsOf($visits));
    $mail = $contact->email instanceof Optional ? 'no email' : 'email';
    $seen = $contact->visits instanceof Optional ? 'never' : 'seen';
    return $contact->name . ': ' . $mail . ', ' . $seen;
}

function hasVisits(int $visits): bool
{
    $contact = new Contact('x', 'x@example.org', visitsOf($visits));
    return !$contact->visits instanceof Optional;
}
