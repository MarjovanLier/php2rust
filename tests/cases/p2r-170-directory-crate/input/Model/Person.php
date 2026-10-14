<?php

declare(strict_types=1);

namespace Acme\Shop\Model;

final readonly class Person
{
    public function __construct(public string $name, public Gender $gender = Gender::Female) {}

    public function label(): string
    {
        return $this->name . ' ' . $this->gender->value;
    }
}
