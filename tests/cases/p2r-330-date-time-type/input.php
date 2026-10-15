<?php

declare(strict_types=1);

function parse(string $text): DateTime|false
{
    return DateTime::createFromFormat('!Y-m-d', $text);
}

function show(DateTime $date): string
{
    return $date->format('Y-m-d') . ' ' . gettype($date);
}

function describe(string $text): string
{
    $date = parse($text);
    if ($date === false) {
        return 'false';
    }
    $copy = $date;
    return show($copy) . ' ' . show($date);
}

function either(string $text): string
{
    $date = parse($text);
    return ($date !== false ? 'some' : 'none') . (false === $date ? ' false' : ' date');
}

echo describe('1980-02-29'), "\n";
echo describe('1980-2-30'), "\n";
echo describe('1980/02/29'), "\n";
echo either('2000-01-01'), ' ', either('x'), "\n";
