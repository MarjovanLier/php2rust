<?php

declare(strict_types=1);

function same(string $text, string $format): bool
{
    $date = DateTime::createFromFormat($format, $text);
    if (!$date instanceof DateTime || $date->format($format) !== $text) {
        return false;
    }
    return true;
}

function leap(string $text): string
{
    $date = DateTime::createFromFormat('!Y', $text);
    if ($date === false || $date->format('y') === '00' || $date->format('Y') === '1900') {
        return 'skip';
    }
    return $date->format('Y');
}

function parsed(string $text): bool
{
    $date = DateTime::createFromFormat('!Ymd', $text);
    if ($date === false || strlen($text) !== 8) {
        return false;
    }
    return true;
}

function counted(string $text, bool $flag): int
{
    $n = 0;
    $date = DateTime::createFromFormat('!Y', $text);
    if ($date === false || $flag) {
        $n++;
    }
    return $n;
}

function looped(string $texts): int
{
    $n = 0;
    foreach (explode(',', $texts) as $text) {
        $date = DateTime::createFromFormat('!Y', $text);
        if ($date === false || $date->format('Y') === '1900') {
            continue;
        }
        $n += (int) $date->format('Y');
    }
    return $n;
}

function withElse(string $text): string
{
    $date = DateTime::createFromFormat('!Y', $text);
    if ($date === false || strlen($text) > 3) {
        return 'a';
    } else {
        return 'b';
    }
}

function marked(?string $text, int $n): string
{
    if ($text === null || $n < 0) {
        return 'x';
    }
    $text .= '!';
    return $text;
}

function half(?int $value): int
{
    if ($value === null || $value % 2 !== 0) {
        return -1;
    }
    return intdiv($value, 2);
}

echo same('19800229', 'Ymd') ? 'y' : 'n', same('19810229', 'Ymd') ? 'y' : 'n', same('x', 'Ymd') ? 'y' : 'n', "\n";
echo leap('1980'), ' ', leap('2000'), ' ', leap('1900'), ' ', leap('x'), "\n";
echo parsed('19800101') ? 'y' : 'n', parsed('1980011') ? 'y' : 'n', parsed('x') ? 'y' : 'n', "\n";
echo half(8), ' ', half(7), ' ', half(null), "\n";
echo marked('a', 1), marked(null, 1), marked('b', -1), "\n";
echo counted('x', true), counted('x', false), counted('1980', true), counted('1980', false), "\n";
echo looped('1980,x,1900,2000'), ' ', withElse('x'), withElse('1980'), withElse('198'), "\n";
