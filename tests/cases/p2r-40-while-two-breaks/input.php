<?php

declare(strict_types=1);

function code(int $n): int
{
    $i = 0;
    while (true) {
        if ($i > $n) {
            $r = 1;
            break;
        }
        if ($i === 3) {
            $r = 2;
            break;
        }
        $i++;
    }
    return $r;
}

/** @param list<string> $words */
function find(array $words): string
{
    $i = 0;
    while (true) {
        if ($i >= count($words)) {
            $found = 'none';
            break;
        }
        if ($words[$i] === 'stop') {
            $found = 'stop at ' . $i;
            break;
        }
        $i++;
    }
    return $found . '!';
}

echo code(1), ' ', code(9), "\n";
echo find(['a', 'stop']), ' ', find(['a']), "\n";
