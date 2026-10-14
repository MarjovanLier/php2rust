<?php

declare(strict_types=1);

final readonly class Code
{
    public function __construct(public string $text) {}

    public function number(): int
    {
        return (int) $this->text;
    }
}

function parse(string $s): int
{
    return (int) $s + ( INT ) "12abc";
}

function label(int $n): string
{
    $owned = "x" . $n;
    $a = (string) $n . (string) -5;
    $b = (STRING)($n + 1) . (string) -$n;
    return $a . $b . (int) $owned;
}

$s = " 42";
echo parse($s), "\n";
echo label(7), "\n";
echo (new Code("1e3"))->number(), "\n";
echo (string) 3000000000, "\n";
