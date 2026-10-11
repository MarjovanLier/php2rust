<?php

declare(strict_types=1);

function ignore(int $value): void
{
}

function unused(int $type): int
{
    return 1;
}

ignore(3);
echo "called\n";
