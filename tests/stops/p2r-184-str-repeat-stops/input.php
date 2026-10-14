<?php

declare(strict_types=1);

function run(string $s, int $n): void
{
    echo str_repeat($s, $n), "\n";
    echo str_repeat($s, -1), "\n";
    echo str_repeat($s, 2, 3), "\n";
}
