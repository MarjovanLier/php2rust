<?php

declare(strict_types=1);

#[\Deprecated]
function old(int $x): int
{
    return $x;
}

function tagged(#[\SensitiveParameter] #[\Example] string $secret): int
{
    return strlen($secret);
}

/**
 * @param list<int> $values
 * @return list<int>
 */
function doubled(array $values): array
{
    return array_map(fn(#[\Example] int $value): int => $value * 2, $values);
}

/**
 * @param list<int> $values
 * @return list<int>
 */
function tripled(array $values): array
{
    return array_map(#[\Deprecated] fn(int $value): int => $value * 3, $values);
}

echo old(1), tagged('ab'), count(doubled([1])), count(tripled([1])), "\n";
