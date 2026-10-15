<?php

declare(strict_types=1);

namespace App;

use Spatie\LaravelData\Data;
use Spatie\LaravelData\Optional;

final class Contact extends Data
{
    public function __construct(
        public string $name,
        public string|Optional $email,
        public int|Optional $visits,
        public string|Optional|null $note = null,
    ) {}
}

function sent(Contact $contact): string
{
    $email = $contact->email instanceof Optional ? 'no email' : 'email';
    $note = !$contact->note instanceof Optional ? 'note' : 'no note';
    return $contact->name . ': ' . $email . ', ' . $note;
}

function blank(): string|Optional
{
    return Optional::create();
}

function given(): int|Optional
{
    return 5;
}

function counted(int $n): string|Optional
{
    return match ($n) {
        1 => 'one',
        default => Optional::create(),
    };
}

final class Price extends Data
{
    public function __construct(
        public float|Optional $amount,
        public string|Optional|null $label,
    ) {}
}

// An int goes in as a float, a value that may be null as it is, and a write wraps too.
function priced(?string $label): Price
{
    $price = new Price(4, $label);
    $price->amount = 5;
    return $price;
}

$price = priced(null);
echo $price->amount instanceof Optional ? 'free' : 'paid', ' ', $price->label instanceof Optional ? 'unlabelled' : 'labelled', "\n";
$full = new Contact('Ann', 'a@b.c', 3, 'hi');
$bare = new Contact(name: 'Bob', email: new Optional, visits: new Optional(), note: new Optional());
$nulled = new Contact('Cy', blank(), given());
echo sent($full), "\n", sent($bare), "\n", sent($nulled), "\n";
echo $nulled->visits instanceof Optional ? 'none' : 'some', "\n";
echo counted(1) instanceof Optional ? 'none' : 'one', ' ', counted(2) instanceof Optional ? 'none' : 'one', "\n";
