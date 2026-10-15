<?php

declare(strict_types=1);

final class Lists
{
    public const string RANGE = 'z..a';

    public const string WIDE = 'é';
}

function php_charmask(string $s): string
{
    return $s;
}

function php_rtrim_chars(string $s): string
{
    return $s;
}

function locals(string $s): string
{
    $php_trim_chars = $s;
    $php_ltrim_chars = $php_trim_chars;
    return $php_ltrim_chars;
}

function run(string $s): void
{
    echo trim($s, 'a..'), "\n";
    echo ltrim($s, '..z'), "\n";
    echo rtrim($s, 'z..a'), "\n";
    echo trim($s, 'a..b..c'), "\n";
    echo trim($s, 'é'), "\n";
    echo trim($s, 'a..~'), "\n";
    echo ltrim($s, characters: 'x'), "\n";
    echo trim($s, Lists::RANGE), "\n";
    echo rtrim($s, Lists::WIDE), "\n";
}
