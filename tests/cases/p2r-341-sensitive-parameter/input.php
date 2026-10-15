<?php

declare(strict_types=1);

final class Secret
{
    public static function masked(#[\SensitiveParameter] string $password): string
    {
        return str_repeat('*', strlen($password));
    }
}

function length(#[\SensitiveParameter()] string $token): int
{
    return strlen($token);
}

/**
 * @param list<int> $pins
 * @return list<int>
 */
function doubled(array $pins): array
{
    return array_map(fn(#[\SensitiveParameter] int $pin): int => $pin * 2, $pins);
}

echo Secret::masked('abc'), ' ', length('abcd'), ' ', count(doubled([1, 2])), ' ', doubled([3])[0], "\n";
