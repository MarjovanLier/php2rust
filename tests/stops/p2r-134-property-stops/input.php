<?php

declare(strict_types=1);

readonly class Person
{
    public function __construct(public ?string $name, public int $age)
    {
    }

    public function shout(): string
    {
        if ($this->name === null) {
            return 'nobody';
        }
        return $this->name . '!';
    }

    public function older(): int
    {
        $this->age = $this->age + 1;
        return $this->age;
    }
}

function uses(Person $p): string
{
    $field = 'age';
    $a = $p->$field;
    $b = $p?->age;
    $c = $p->name ?? 'x';
    return "{$p->age}" . $a . $b . $c;
}
