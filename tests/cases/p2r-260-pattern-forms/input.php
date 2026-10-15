<?php

declare(strict_types=1);

final class Forms
{
    private const HEX = '/^[[:xdigit:]]{2,4}\z/';

    public static function check(string $text): string
    {
        $found = '';
        if (preg_match(self::HEX, $text)) {
            $found .= 'hex ';
        }
        if (preg_match('{^\d+(?:\.\d{1,2})?\z}', $text)) {
            $found .= 'amount ';
        }
        if (preg_match('/^(cat|dog)s?\z/i', $text)) {
            $found .= 'pet ';
        }
        if (preg_match('#^[^[:^alpha:]0-9]+\z#', $text)) {
            $found .= 'letters ';
        }
        if (preg_match('~\x41\x{42}~', $text)) {
            $found .= 'ab ';
        }
        if (preg_match('/a.b/s', $text)) {
            $found .= 'dot ';
        }
        if (preg_match('/\nend$/m', $text)) {
            $found .= 'line ';
        }
        if (preg_match('/\Ax+?y\z/', $text)) {
            $found .= 'lazy ';
        }
        if (preg_match('<\<b\>>', $text)) {
            $found .= 'tag ';
        }
        if (preg_match('/"q"|\bword\b/', $text)) {
            $found .= 'quote ';
        }
        if (preg_match('/[[:space:]\t-]{3}/', $text)) {
            $found .= 'gap ';
        }
        if (preg_match(Mark::COLON, $text)) {
            $found .= 'colon ';
        }
        if (preg_match('/\w\W\S\B\w|\r|\f/', $text)) {
            $found .= 'escapes ';
        }
        if (preg_match('/^a*?b??c{1,3}?\z/si', $text)) {
            $found .= 'lazy2 ';
        }
        if (preg_match('/^[[:alnum:][:blank:][:punct:]]+[[:cntrl:][:digit:]]\z/', $text)) {
            $found .= 'posix ';
        }
        if (preg_match('/^[[:ascii:]][[:graph:]][[:lower:]][[:print:]][[:upper:]][[:word:]]/', $text)) {
            $found .= 'posix2 ';
        }
        return $found;
    }
}

enum Mark: string
{
    public const COLON = '/[:]/';

    case Dot = '.';
}

foreach (['ff', 'FfA0', '12.50', 'Dogs', 'abc', 'xAB', "a\nb", "one\nend\ntwo", 'xxy', '<b>', 'say "q"', 'a word.', "a \t-b", "\u{0663}", 'x:y', 'ab-cd', "x\ry", 'AAC', 'bc', 'a b!1', "x\x01", ' !aPQ_'] as $text) {
    echo Forms::check($text), "\n";
}
