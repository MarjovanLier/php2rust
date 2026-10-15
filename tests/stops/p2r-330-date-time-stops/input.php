<?php

declare(strict_types=1);

final class PhpDateTime
{
}

function parse(string $text): DateTime|false
{
    return DateTime::createFromFormat('!Y', $text);
}

function either(DateTime|string $value): string
{
    return is_string($value) ? $value : 'date';
}

function made(): DateTime
{
    return new DateTime();
}

function same(string $text): bool
{
    return parse($text) === parse($text);
}

function missing(string $text): bool
{
    return parse($text) === null;
}

function fallback(string $text): DateTime|false
{
    return parse($text) ?? parse('1980');
}

function immutable(string $text): DateTimeImmutable|false
{
    return DateTimeImmutable::createFromFormat('!Y', $text);
}

function errors(): bool
{
    return DateTime::getLastErrors() === false;
}

function stamp(string $text): string
{
    $date = parse($text);
    if ($date === false) {
        return 'none';
    }
    return $text . $date->getTimestamp() . $date;
}
