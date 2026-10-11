<?php

declare(strict_types=1);

function size(string $text): int
{
    return strlen($text) * 2 - STRLEN('ab');
}

$n = 12;
$len = strlen("n=$n");
echo strlen('héllo'), ' ', -strlen('abc'), ' ', size('four'), ' ', $len, "\n";
