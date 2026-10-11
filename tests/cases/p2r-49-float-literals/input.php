<?php

declare(strict_types=1);

function same(float $a, float $b): void
{
    if ($a === $b) {
        echo 'y';
    } else {
        echo 'n';
    }
}

function above(float $a, float $b): void
{
    if ($a > $b) {
        echo 'y';
    } else {
        echo 'n';
    }
}

same(1.0, 1.);
same(0.5, .5);
same(1_000.5, 1000.5);
same(1e3, 1000.0);
same(1E-3, 0.001);
same(0.1 + 0.2, 0.30000000000000004);
same(0.1 + 0.2, 0.3);
same(1e-400, 0.0);
same(9007199254740993.0, 9007199254740992.0);
above(1E400, 1.7976931348623157e308);
above(1e300, 1e299);
above(5e-324, 0.0);
echo "\n";
