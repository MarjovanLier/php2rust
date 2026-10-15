<?php

declare(strict_types=1);

namespace App;

function length(string $text): string
{
    return strlen($text);
}

function rate(): int
{
    return \App\RATE + 1;
}
