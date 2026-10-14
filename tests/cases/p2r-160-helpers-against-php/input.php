<?php

declare(strict_types=1);

function digits(string $text): string
{
    if (ctype_digit($text)) {
        return "1";
    }
    return "0";
}

function number(string $text): int
{
    return (int) $text;
}

function cut(string $text, int $start): string
{
    return "[" . substr($text, $start) . "]";
}

function part(string $text, int $start, int $length): string
{
    return "[" . substr($text, $start, $length) . "]";
}

function at(string $text, int $index): string
{
    return "[" . $text[$index] . "]";
}

echo digits(""), digits("0"), digits("123"), digits("12a"), digits(" 1"), "\n";
echo digits("-1"), digits("1.0"), digits("+1"), digits("٣"), digits("0123456789"), "\n";
echo number(""), " ", number(" "), " ", number("abc"), " ", number("-"), " ", number("+"), "\n";
echo number("42"), " ", number(" 42"), " ", number("\t\n\r\v\f42"), " ", number("42 "), "\n";
echo number("+7"), " ", number("-7"), " ", number("007"), " ", number("12abc"), "\n";
echo number("1.9"), " ", number("-1.9"), " ", number(".5"), " ", number("5."), "\n";
echo number("-.5"), " ", number("."), " ", number("-."), " ", number("0x1A"), "\n";
echo number("1e3"), " ", number("1E3"), " ", number("1e"), " ", number("1e+2"), "\n";
echo number("1e-2"), " ", number("2.5e1"), " ", number("1e+"), " ", number("-1e3x"), "\n";
echo number("9223372036854775807"), " ", number("9223372036854775808"), "\n";
echo number("-9223372036854775808"), " ", number("-9223372036854775809"), "\n";
echo number("99999999999999999999"), " ", number("-99999999999999999999"), "\n";
echo number("1e100"), " ", number("-1e100"), " ", number("1e400"), " ", number("-1e400"), "\n";
echo number("9.2233720368547758e18"), " ", number("1.5e18"), " ", number(" +3.7e2z"), "\n";
echo cut("abcdef", 0), cut("abcdef", 1), cut("abcdef", 5), cut("abcdef", 6), "\n";
echo cut("abcdef", 7), cut("abcdef", -1), cut("abcdef", -6), cut("abcdef", -7), "\n";
echo cut("abcdef", -100), cut("", 0), cut("", 1), cut("", -1), "\n";
echo part("abcdef", 0, 0), part("abcdef", 0, 3), part("abcdef", 1, -1), "\n";
echo part("abcdef", 2, -10), part("abcdef", 0, 100), part("abcdef", -3, 2), "\n";
echo part("abcdef", -3, -1), part("abcdef", 6, 1), part("abcdef", 5, -1), "\n";
echo part("abcdef", 7, 2), part("abcdef", -10, 2), part("abcdef", -10, -5), "\n";
echo part("", 0, 0), part("", 0, 1), part("", 0, -1), part("abc", 3, -1), "\n";
echo at("abc", 0), at("abc", 2), at("abc", -1), at("abc", -3), at("aé", 0), "\n";
