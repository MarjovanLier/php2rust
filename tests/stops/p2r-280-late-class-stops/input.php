<?php

declare(strict_types=1);

trait Swaps
{
    private const array MAP = ['a' => 'b'];
}

echo Late::swap('abc'), "\n";

// PHP declares this class only when execution reaches it, so the echo above fails.
final class Late
{
    use Swaps;

    public static function swap(string $text): string
    {
        return strtr($text, self::MAP);
    }
}
