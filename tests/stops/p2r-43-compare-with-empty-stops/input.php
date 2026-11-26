<?php

declare(strict_types=1);

/** @param ?list<int> $xs */
function maybe(?array $xs): bool
{
    return $xs === [];
}

function text(string $s): bool
{
    return $s !== [];
}

function both(): bool
{
    return [] === [];
}

echo maybe(null) ? 'a' : 'b', text('x') ? 'a' : 'b', both() ? 'a' : 'b', "\n";
