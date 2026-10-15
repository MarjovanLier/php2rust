<?php

declare(strict_types=1);

function run(int $m, int $d, int $y, ?int $n): void
{
    echo checkdate($m, $d) ? 'y' : 'n', "\n";
    echo checkdate(month: $m, day: $d, year: $y) ? 'y' : 'n', "\n";
    echo checkdate($m, $d, $n) ? 'y' : 'n', "\n";
}
