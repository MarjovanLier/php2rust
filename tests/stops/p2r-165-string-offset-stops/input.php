<?php

declare(strict_types=1);

function read(string $s, ?string $maybe, ?int $at, bool $flag): void
{
    echo 'abc'[5], 'abc'[-4];
    $s[0] = 'x';
    echo $maybe[0];
    echo $s[$at];
    echo $s[$flag];
}

read("abc", "d", 1, true);
