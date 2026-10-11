<?php

declare(strict_types=1);

function untyped($value): int
{
    return 1;
}

function noReturnType(int $value)
{
    return $value;
}
