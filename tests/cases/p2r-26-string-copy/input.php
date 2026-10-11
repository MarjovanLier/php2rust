<?php

declare(strict_types=1);

function copyOf(string $text): string
{
    $copy = $text;
    $copy .= '?';
    return $copy;
}

function same(string $text): string
{
    return $text;
}

$a = 'one';
$b = $a;
$a .= '!';
$c = $b;
$d = "{$a}";
echo $a, ' ', $c, ' ', copyOf($a), ' ', same('two'), ' ', $d, "\n";
