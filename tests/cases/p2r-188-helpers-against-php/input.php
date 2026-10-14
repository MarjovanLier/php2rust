<?php

declare(strict_types=1);

function trims(string $s): string
{
    return '[' . trim($s) . '|' . ltrim($s) . '|' . rtrim($s) . ']';
}

function firsts(string $s): string
{
    return '[' . ucfirst($s) . '|' . lcfirst($s) . ']';
}

function replaced(string $search, string $replace, string $subject): string
{
    return '[' . str_replace($search, $replace, $subject) . ']';
}

function middle(string $s): string
{
    return '[' . substr(trim($s), 1, 2) . ']';
}

function pieces(string $separator, string $s): string
{
    $parts = explode($separator, $s);
    return count($parts) . '[' . implode('|', $parts) . ']';
}

echo trims(''), trims(' '), trims("  a b  "), trims("\t\n\r\0\x0Bx\x0B\0\r\n\t"), "\n";
echo trims("\x0Cx\x0C"), trims("\u{a0}x\u{a0}"), trims("é "), trims("\0"), "\n";
echo firsts(''), firsts('a'), firsts('A'), firsts('é'), firsts('1x'), firsts('zZ'), "\n";
echo firsts('Éa'), firsts('[x'), firsts('`y'), firsts('{z'), firsts('@A'), "\n";
echo replaced('a', 'o', 'banana'), replaced('', 'x', 'abc'), replaced('ana', 'A', 'banana'), "\n";
echo replaced('aa', 'b', 'aaaa'), replaced('x', 'y', ''), replaced('é', 'e', 'café'), "\n";
echo replaced('a', '', 'banana'), replaced('ab', 'ba', 'abab'), replaced('', '', ''), "\n";
echo pieces(',', 'a,b,,c'), pieces(',', ''), pieces(',', ','), pieces('ab', 'xabyabz'), "\n";
echo pieces(',', 'abc'), pieces('é', 'aéb'), pieces('aa', 'aaa'), pieces(' ', ' x '), "\n";
echo middle(' abcd '), middle(''), middle('a'), "\n";
