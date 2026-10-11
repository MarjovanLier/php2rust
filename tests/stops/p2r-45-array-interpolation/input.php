<?php

declare(strict_types=1);

function first(string $word): string
{
    return "First: $word[0], second: {$word[1]}";
}

echo first('abc'), "\n";
