<?php

declare(strict_types=1);

final class Stops
{
    private const GROUP = '$1';

    public static function bare(string $text): ?string
    {
        return preg_replace('/a/', 'b', $text);
    }

    public static function limited(string $text): string
    {
        return preg_replace('/a/', 'b', $text, 1) ?? '';
    }

    public static function named(string $text): string
    {
        return preg_replace(pattern: '/a/', replacement: 'b', subject: $text) ?? '';
    }

    public static function emptyMatch(string $text): string
    {
        return preg_replace('#x*#', '-', $text) ?? '';
    }

    public static function dollar(string $text): string
    {
        return preg_replace('/(a)/', self::GROUP, $text) ?? '';
    }

    public static function backslash(string $text): string
    {
        return preg_replace('/a/', '\\0\\0', $text) ?? '';
    }

    public static function variableReplacement(string $text, string $with): string
    {
        return preg_replace('/a/', $with, $text) ?? '';
    }

    public static function variableDefault(string $text, string $default): string
    {
        return preg_replace('/a/', 'b', $text) ?? $default;
    }

    public static function patterns(string $text): string
    {
        return preg_replace(['/a/', '/b/'], 'c', $text) ?? '';
    }
}
