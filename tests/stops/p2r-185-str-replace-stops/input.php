<?php

declare(strict_types=1);

/** @param list<string> $xs */
function run(array $xs, string $s): void
{
    echo str_replace($xs, 'b', $s), "\n";
    echo str_replace('a', 'b', $s, $count), "\n";
    echo str_replace('a', $xs, $s), "\n";
}
