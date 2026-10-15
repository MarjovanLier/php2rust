<?php

declare(strict_types=1);

final class Swap
{
    private const array MAP = ['a' => 'b'];

    public static function three(string $text): string
    {
        return strtr($text, 'ab', 'xy');
    }

    public static function literal(string $text): string
    {
        return strtr($text, ['a' => 'b']);
    }

    /**
     * @param array<string, string> $pairs
     */
    public static function given(string $text, array $pairs): string
    {
        return strtr($text, $pairs);
    }

    public static function named(string $text): string
    {
        return strtr(string: $text, from: self::MAP);
    }
}

function php_strtr(string $text): string
{
    return $text;
}
