<?php

declare(strict_types=1);

function plainValue(string $s): bool
{
    return is_string($s);
}

function noMember(int|bool $v): bool
{
    return is_string($v);
}

function twoArguments(int|string $v): bool
{
    return is_int($v, 1);
}

function otherTests(int|string $v): bool
{
    return is_numeric($v) || is_array($v);
}
