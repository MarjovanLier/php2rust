<?php

declare(strict_types=1);

function forms(string $s, string $d): string
{
    $a = preg_replace_callback('#x*#', static fn(array $m): string => '-', $s) ?? '';
    $b = preg_replace_callback('#\W#', static fn(array $m): string => $m[0], $s) ?? '';
    $c = preg_replace_callback('#a#', static fn(array $m): string => 'b', $s);
    $e = preg_replace_callback('#a#', static fn(array $m): string => 'b', $s) ?? $d;
    $f = preg_replace_callback('#a#', static fn(string $m): string => 'b', $s) ?? '';
    $g = preg_replace_callback('#a#', static fn(array $m) => 'b', $s) ?? '';
    $h = preg_replace_callback('#a#', static function (array $m): string {
        return 'b';
    }, $s) ?? '';
    $i = preg_replace_callback('#a#', static fn(array $m): string => 'b', $s, 1) ?? '';
    $j = preg_replace_callback(pattern: '#a#', callback: static fn(array $m): string => 'b', subject: $s) ?? '';
    $k = preg_replace_callback('#.#', static fn(array $m): string => 'b', $s) ?? '';
    return $a . $b . ($c ?? '') . $e . $f . $g . $h . $i . $j . $k;
}
