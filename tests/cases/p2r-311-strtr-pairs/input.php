<?php

declare(strict_types=1);

function swap(string $s, string $a, string $b): string
{
    return strtr($s, [$a => $b, $b => $a]);
}

function longest(string $s): string
{
    return strtr($s, ['a' => '1', 'ab' => '2', 'abc' => '3', 'b' => '4']);
}

function repeated(string $s, string $k): string
{
    return strtr($s, [$k => 'first', 'x' => 'X', $k . '' => 'last']);
}

echo swap('abba', 'a', 'b'), "\n";
echo swap('a1b2', '1', '2'), "\n";
echo longest('abcabxab'), "\n";
echo repeated('xyz', 'y'), "\n";
echo strtr('héllo', ['é' => 'e', 'l' => 'L']), "\n";
