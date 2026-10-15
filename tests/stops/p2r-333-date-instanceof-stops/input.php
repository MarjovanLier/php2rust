<?php

declare(strict_types=1);

final class Stamp
{
}

function of_interface(string $text): bool
{
    $date = DateTime::createFromFormat('!Y', $text);
    return $date instanceof DateTimeInterface;
}

function of_number(int $number): bool
{
    return $number instanceof DateTime;
}

function of_stamp(Stamp $stamp): bool
{
    return $stamp instanceof Stamp;
}

function named(string $text, string $class): bool
{
    $date = DateTime::createFromFormat('!Y', $text);
    return $date instanceof $class;
}
