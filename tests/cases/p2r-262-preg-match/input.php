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

function first_digit(string $text): int
{
    $at = 0;
    for (; $at < strlen($text) && !preg_match('/\d/', substr($text, $at, 1)); $at++) {
        $text .= '';
    }
    return $at;
}

function flags(string $text): string
{
    $none = !preg_match('/a/', $text);
    return (preg_match('/b/', $text) ? 'b' : '-') . ($none ? 'no-a' : 'a');
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
echo first_digit('ab3c'), ' ', first_digit('abc'), "\n";
echo flags('ab'), ' ', flags('c'), "\n";
echo compared('cat'), "\n";
echo compared('dog'), "\n";
