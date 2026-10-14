<?php

declare(strict_types=1);

function cut(string $s, ?int $maybe, ?string $text): string
{
    $a = substr($s, 0, null);
    $b = substr($s, $maybe);
    $c = substr($text, 1);
    $d = substr($s);
    $e = substr(string: $s, offset: 1);
    return $a . $b . $c . $d . $e;
}

echo cut("abc", 1, "x"), "\n";
