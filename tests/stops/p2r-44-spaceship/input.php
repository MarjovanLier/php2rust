<?php

declare(strict_types=1);

function order(int $a, int $b): int
{
    return $a <=> $b;
}
