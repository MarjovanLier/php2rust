<?php

declare(strict_types=1);

/** @param list<int> $xs */
function bars(array $xs, string $s): string
{
    return str_repeat('-', count($xs)) . str_repeat($s, strlen($s));
}

echo str_repeat('ab', 3), '|', str_repeat('x', 0), '|', str_repeat('', 4), "\n";
echo str_repeat('y', -0), '|', "\n";
echo bars([1, 2, 3], 'yz'), '|', bars([], ''), "\n";
