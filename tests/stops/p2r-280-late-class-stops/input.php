<?php

declare(strict_types=1);

trait Swaps
{
    private const array MAP = ['a' => 'b'];
}

echo Late::swap('abc'), "\n";
echo Counts::LIMIT, "\n";

/**
 * @implements ArrayAccess<string, int>
 */
final readonly class Counts implements ArrayAccess
{
    public const int LIMIT = 3;

    public function offsetExists(mixed $offset): bool
    {
        return false;
    }

    public function offsetGet(mixed $offset): int
    {
        return 0;
    }

    public function offsetSet(mixed $offset, mixed $value): void
    {
    }

    public function offsetUnset(mixed $offset): void
    {
    }
}

// PHP declares this class only when execution reaches it, so the echo above fails.
final class Late
{
    use Swaps;

    public static function swap(string $text): string
    {
        return strtr($text, self::MAP);
    }
}
