<?php

declare(strict_types=1);

function x(string $s): string
{
    return '<' . $s . '>';
}

echo implode('', array_map(x(...), ['a', 'b'])), "\n";
