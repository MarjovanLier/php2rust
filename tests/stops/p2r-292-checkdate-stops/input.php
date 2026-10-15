<?php

declare(strict_types=1);

function php_checkdate(int $m): int
{
    return $m;
}

function run(int $m, int $d, int $y, ?int $n, string $s, float $f): void
{
    echo checkdate($m, $d) ? 'y' : 'n', "\n";
    echo checkdate(month: $m, day: $d, year: $y) ? 'y' : 'n', "\n";
    echo checkdate($m, $d, $n) ? 'y' : 'n', "\n";
    echo checkdate($s, $d, $y) ? 'y' : 'n', "\n";
    echo checkdate($m, $f, $y) ? 'y' : 'n', "\n";
    echo checkdate(...[1, 1, 2000]) ? 'y' : 'n', "\n";
}
