<?php

declare(strict_types=1);

final readonly class Id
{
    public function __construct(public string $text) {}

    public function birth(): string
    {
        return substr($this->text, 0, 6);
    }
}

function tail(string $s, int $n): string
{
    $owned = $s . "xyz";
    $end = substr($owned, -$n);
    $middle = substr($s, $n + 1, 2);
    return $end . $middle . substr("literal", 3);
}

echo (new Id("8001015009087"))->birth(), "\n";
echo tail("abcdef", 2), "\n";
$inner = substr("abcdef", 1);
echo substr($inner, 1, -1), "\n";
