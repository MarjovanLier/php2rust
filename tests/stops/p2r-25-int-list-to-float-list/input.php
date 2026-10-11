<?php

declare(strict_types=1);

/**
 * @param list<float> $values
 */
function first(array $values): float
{
    return $values[0];
}

/**
 * @return list<float>
 */
function ints(): array
{
    return [1, 2];
}

/**
 * @param list<int> $numbers
 */
function head(array $numbers): float
{
    return first($numbers);
}

function literal(): float
{
    return first([3, 4]);
}
