<?php

declare(strict_types=1);

namespace Acme\Shop\Service;

use Acme\Shop\Model\Gender;
use Acme\Shop\Model\Person;

enum Tone
{
    case Male;
    case Warm;
}

final class Greeter
{
    public int $count = 0;

    public function greet(Person $person, Tone $tone = Tone::Warm): string
    {
        $this->count++;
        $title = match ($person->gender) {
            Gender::Male => 'Mr',
            Gender::Female => 'Ms',
        };
        return shout($title . ' ' . $person->label(), $tone === Tone::Warm);
    }

    public static function make(): Greeter
    {
        return new Greeter();
    }
}
