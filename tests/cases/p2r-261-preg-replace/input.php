<?php

declare(strict_types=1);

final class Clean
{
    /** @var non-empty-string */
    private const NON_DIGIT_REGEX = '#\D#';
    private const DASH = '-';

    public static function digits(string $number): string
    {
        return \preg_replace(self::NON_DIGIT_REGEX, '', $number) ?? '';
    }

    public static function squeeze(string $text): string
    {
        $text = \preg_replace('# {2,}#', ' ', $text) ?? '';
        return trim($text);
    }

    public static function dashes(string $text): string
    {
        return preg_replace('/[[:space:]_]+/', self::DASH, strtolower($text)) ?? 'unused';
    }

    // Both helpers in one output, in the order of P2R-188.
    public static function digitsOnly(string $text): bool
    {
        return preg_match('/\D/', $text) === 0;
    }
}

echo Clean::digits('80-01-01 5009 087'), "\n";
echo Clean::digits("12\u{0663}4"), "\n";
echo Clean::squeeze('  a   b  c d  '), "\n";
echo Clean::dashes("Hello  World_Again\tNow"), "\n";
echo Clean::digitsOnly(Clean::digits('80-01')) ? 'digits' : 'mixed', "\n";
