<?php

declare(strict_types=1);

function php_trim(): int
{
    return 1;
}

function size(string $php_ltrim, string $phpRtrim): int
{
    return strlen($php_ltrim) + strlen($phpRtrim);
}

function local(): int
{
    $php_ucfirst = 1;
    $php_lcfirst = 2;
    $php_replace = 3;
    $php_explode = 4;
    return $php_ucfirst + $php_lcfirst + $php_replace + $php_explode;
}
