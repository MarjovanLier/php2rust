<?php

declare(strict_types=1);

/**
 * @param list<int> $ns
 * @param list<string> $xs
 */
function run(array $ns, array $xs, string $s): void
{
    echo implode($xs), "\n";
    echo implode(',', $ns), "\n";
    echo implode(strtolower($s), $xs), "\n";
    echo count(explode(',', $s, 2)), "\n";
    echo count(explode('', $s)), "\n";
}
