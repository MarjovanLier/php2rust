<?php

declare(strict_types=1);

// PHP fails to parse a float with _ that does not stand between two digits, and Mago reads
// 1._0 as 1.0 (P2R-503). Main converted this function.
function large(float $x): bool
{
    return $x > 1._0;
}

// Between two digits, _ parses.
function larger(float $x): bool
{
    return $x > 1_000.5 && $x < 2.5e1_0;
}

echo larger(1500.0) ? 'y' : 'n', "\n";
