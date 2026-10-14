<?php

declare(strict_types=1);

enum Size: string
{
    case Small = 's';
}

function f(int $p, int $q = 2, int $r = 3): int
{
    return $p + $q + $r;
}

echo f(1, r: 2, q: 3), "\n";
echo f(r: 1, p: 2), "\n";
echo f(1, p: 2), "\n";
echo f(...[1, 2]), "\n";
echo f(1, z: 3), "\n";
echo f(), "\n";
echo f(1, 2, 3, 4), "\n";
echo f(p: 1, 2), "\n";
echo strlen(string: "x"), "\n";
echo Size::from(value: 's')->value, "\n";
