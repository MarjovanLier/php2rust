<?php

declare(strict_types=1);

function php_preg_replace(string $text): string
{
    return $text;
}

function clean(string $php_preg_match): bool
{
    return preg_match('/a/', $php_preg_match) === 1;
}
