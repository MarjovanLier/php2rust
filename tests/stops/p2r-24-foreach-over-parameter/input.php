<?php

declare(strict_types=1);

/** @param list<string> $names */
function greet(string $name, array $names): void
{
    echo $name, "\n";
    foreach ($names as $name) {
        echo "Hi $name\n";
    }
}

greet('all', ['Ann']);
