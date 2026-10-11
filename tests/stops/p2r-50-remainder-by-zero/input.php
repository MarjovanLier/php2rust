<?php

declare(strict_types=1);

function broken(int $n): int
{
    return $n % 0;
}
