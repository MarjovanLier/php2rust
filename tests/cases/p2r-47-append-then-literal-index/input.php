<?php

declare(strict_types=1);

function sum(int $n): int
{
    $a = [1, 2];
    $a[] = $n;
    $a[] = 4;
    return $a[0] + $a[2] + $a[3];
}

/** @return list<string> */
function letters(): array
{
    $a = ['x', 'y'];
    $a[] = 'z';
    echo $a[2], "\n";
    return $a;
}

$a = [1, 2];
$a[] = 3;
echo $a[0], ' ', $a[2], "\n";
echo sum(3), ' ', count(letters()), "\n";
