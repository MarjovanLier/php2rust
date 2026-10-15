<?php

declare(strict_types=1);

function php_ucwords(string $s): string
{
    return $s;
}

function forms(string $s): string
{
    $php_preg_replace_callback = ucwords($s, '-');
    return $php_preg_replace_callback . ucwords(string: $s);
}
