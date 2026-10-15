<?php

declare(strict_types=1);

function forms(string $s): string
{
    $a = preg_replace('#(?<!\p{L})mc(?! )#u', 'Mc', $s) ?? '';
    $b = preg_replace_callback('#(?<!\p{L})mc(?! )#u', static fn(array $m): string => $m[0], $s) ?? '';
    $c = preg_match('#(?<!\p{L})mc(?! )#ui', $s) === 1;
    $d = preg_match('#a(?<!\p{L})mc(?! )#u', $s) === 1;
    $e = preg_match('#(?<!\p{L})mc#u', $s) === 1;
    $f = preg_match('#(?<! )mc(?! )#u', $s) === 1;
    $g = preg_match('#(?<!\p{L})m-c(?! )#u', $s) === 1;
    $h = preg_match('/mc/u', $s) === 1;
    $i = preg_match('<(?<!\p{L})mc(?! )>u', $s) === 1;
    $j = preg_match('!(?<!\p{L})mc(?! )!u', $s) === 1;
    $k = preg_match("\t(?<!\\p{L})mc(?! )\tu", $s) === 1;
    return $a . $b . ($c || $d || $e || $f || $g || $h || $i || $j || $k ? '1' : '0');
}
