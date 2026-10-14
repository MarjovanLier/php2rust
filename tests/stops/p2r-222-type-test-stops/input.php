<?php

declare(strict_types=1);

function word(): string
{
    return 'w';
}

function called(): bool
{
    return is_scalar(word());
}

function nullable(?int $n): string
{
    return gettype($n);
}
