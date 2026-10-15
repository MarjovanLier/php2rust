<?php

declare(strict_types=1);

function show(bool $value): string
{
    return $value ? 'yes' : 'no';
}

function kind(string $text): string
{
    if (preg_match('/^\d+\z/', $text)) {
        return 'number';
    } elseif (!preg_match('/[^a-z]/', $text) && $text !== '') {
        return 'word';
    }
    return preg_match('/\s/', $text) || $text === '' ? 'spaced' : 'other';
}

function to_last_line(string $text): int
{
    $steps = 0;
    while (preg_match('/\n/', $text)) {
        $text = substr($text, 1);
        $steps++;
    }
    return $steps;
}

function compared(string $text): string
{
    $parts = [
        preg_match('/a/', $text) === 1,
        preg_match('/a/', $text) !== 0,
        preg_match('/a/', $text) === 0,
        preg_match('/a/', $text) !== 1,
        1 === preg_match('/a/', $text),
        0 !== preg_match('/a/', $text),
        0 === preg_match('/a/', $text),
        1 !== preg_match('/a/', $text),
    ];
    $out = '';
    foreach ($parts as $part) {
        $out .= show($part) . ' ';
    }
    return $out;
}

echo kind('123'), ' ', kind('abc'), ' ', kind('a b'), ' ', kind('A1!'), "\n";
echo to_last_line("a\nb\nc"), "\n";
echo compared('cat'), "\n";
echo compared('dog'), "\n";
