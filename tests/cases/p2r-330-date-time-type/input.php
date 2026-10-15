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

function maybe(string $text): DateTime|false
{
    if ($text === '') {
        return false;
    }
    return parse($text);
}

function chosen(int $kind, string $text): DateTime|false
{
    return match ($kind) {
        1 => parse($text),
        default => false,
    };
}

function given(DateTime|false $date): string
{
    if ($date === false) {
        return 'false';
    }
    return $date->format('Y');
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
echo given(maybe('')), ' ', given(maybe('1999-12-31')), ' ', given(chosen(1, '2001-02-03')), ' ',
    given(chosen(2, '2001-02-03')), ' ', given(false), "\n";
