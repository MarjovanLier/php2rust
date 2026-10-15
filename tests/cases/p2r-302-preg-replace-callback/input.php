<?php

declare(strict_types=1);

function prefixes(string $name): string
{
    return preg_replace_callback(
        '#\b(van|von|den|der|des|de|du|la|le)\b#i',
        static fn(array $matches): string => strtolower($matches[1]),
        $name,
    ) ?? '';
}

function shown(string $text): string
{
    return preg_replace_callback(
        '#(a)(x)?(b)(y)?#',
        static fn(array $m): string => '<' . count($m) . ':' . $m[0] . ':' . $m[2] . '>',
        $text,
    ) ?? 'failed';
}

function last(string $text): string
{
    return preg_replace_callback('#(?:(a)|b)+#', fn(array $m): string => '[' . $m[1] . ']', $text) ?? '';
}

function doubled(string $text, int $times): string
{
    return preg_replace_callback('#[0-9]+#', fn(array $m): string => $m[0] . ($times > 1 ? '+' : '-'), $text) ?? '';
}

function dashed(string $text): string
{
    return preg_replace_callback('#[aeiou]#', static fn(array $match): string => '-', $text) ?? '';
}

echo prefixes('Van Der Waals de la Cruz'), "\n";
echo dashed('education'), "\n";
echo prefixes("VAN\tDER Berg"), "\n";
echo prefixes('Vanessa Dupont'), "\n";
echo shown('ab axb aby xyz'), "\n";
echo last('aab-bba-ba'), "\n";
echo doubled('a1b22 é 3', 2), "\n";
