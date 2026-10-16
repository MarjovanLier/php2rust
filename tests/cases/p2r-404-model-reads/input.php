<?php

declare(strict_types=1);

namespace App;

use Illuminate\Database\Eloquent\Model;

final class Contact extends Model
{
}

// A model type converts as a readonly class type does, also as a property.
final readonly class Letter
{
    public function __construct(public Contact $contact, public string $body) {}

    public function heading(): string
    {
        $name = $this->contact->name;
        return 'Dear ' . $name . ',';
    }

    public function email(): ?string
    {
        return $this->contact->email;
    }
}

function greet(Contact $contact): string
{
    return shout($contact->name);
}

function shout(string $text): string
{
    return strtoupper($text);
}

function owner(Letter $letter): Contact
{
    return $letter->contact;
}
