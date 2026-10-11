<?php

declare(strict_types=1);

function greet(string $name, int $age): string
{
    return "Hi $name, age {$age}!";
}

$n = 7;
echo greet('Ann', 40), "\n";
echo "{ $n} {braces} \$n $n$n\n";
echo "\e[1m$n\e[0m\n";
