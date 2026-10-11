<?php

declare(strict_types=1);

function label(string $name, int $count): string
{
    return $name . ': ' . $count . ' {items}';
}

function shout(string $word): string
{
    return $word . '!';
}

echo label('apples', 3) . "\n";
echo 'Sum: ' . 1 + 2, "\n";
echo shout(label('pears', -4)), "\n";
echo 'a' . ('b' . 7), "\n";
