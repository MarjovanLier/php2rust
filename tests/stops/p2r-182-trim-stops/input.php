<?php

declare(strict_types=1);

function run(string $s): void
{
    echo trim($s, 'x'), "\n";
    echo ltrim(string: $s), "\n";
    echo rtrim(...[$s]), "\n";
}
