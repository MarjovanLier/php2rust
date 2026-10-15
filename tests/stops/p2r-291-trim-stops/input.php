<?php

declare(strict_types=1);

final class Lists
{
    public const string RANGE = 'z..a';

    public const string WIDE = 'é';
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
